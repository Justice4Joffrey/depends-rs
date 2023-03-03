use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use depends::{
    core::{Dependency, Depends, LeafNode, Resolve, UpdateDependee, UpdateLeaf},
    derives::{dependencies, Dependee, Leaf},
    graphviz::GraphvizVisitor,
};

/// A number which can be edited from the _outside_ i.e. has _no_ dependencies.
#[derive(Leaf, Default, Hash)]
pub struct NumberInput {
    value: i32,
}

impl UpdateLeaf for NumberInput {
    type Input = i32;

    fn update_mut(&mut self, input: Self::Input) {
        // Implementing this trait will provide a way for code outside of this graph to
        // change its internal state. This is just a simple replace for now.
        self.value = input;
    }
}

/// Any *derived* node must state its dependencies. If there are more than one,
/// this must be wrapped in a struct using the [dependencies] attribute.
#[dependencies]
pub struct Components {
    left: LeafNode<NumberInput>,
    right: LeafNode<NumberInput>,
}

#[dependencies]
pub struct AnswerComponents {
    left: SumNode,
    right: SquareNode,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = AnswerComponents, node_name = AnswerNode)]
pub struct Answer {
    value: i32,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = Components, node_name = SumNode)]
pub struct Sum {
    value: i32,
}

// An example of how a single dependency can be used.
#[derive(Dependee, Default, Hash)]
#[depends(dependencies = Dependency<Rc<MultiplyNode>>, node_name = SquareNode)]
pub struct Square {
    value: i32,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = Components, node_name = MultiplyNode)]
pub struct Multiply {
    value: i32,
}

impl UpdateDependee for Square {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        self.value = input.value.pow(2);
    }
}

impl UpdateDependee for Sum {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.value + right.value;
    }
}

impl UpdateDependee for Answer {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let AnswerComponentsRef { left, right } = input;
        // TODO oppressive dereferencing
        self.value = left.value + 2 * right.value;
    }
}

impl UpdateDependee for Multiply {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.value * right.value;
    }
}

struct MyGraph {
    a: Rc<LeafNode<NumberInput>>,
    b: Rc<LeafNode<NumberInput>>,
    c: Rc<LeafNode<NumberInput>>,
    answer: Rc<AnswerNode>,
}

fn main() {
    let a = NumberInput::default().into_leaf();
    let b = NumberInput::default().into_leaf();
    let c = NumberInput::default().into_leaf();

    let sum = Sum::default().into_node(Components::new(Rc::clone(&a), Rc::clone(&b)));
    let multiply = Multiply::default().into_node(Components::new(Rc::clone(&a), Rc::clone(&c)));
    let square = Square::default().into_node(Dependency::new(Rc::clone(&multiply)));
    let answer =
        Answer::default().into_node(AnswerComponents::new(Rc::clone(&sum), Rc::clone(&square)));

    let graph = MyGraph { a, b, c, answer };

    // check the graphviz of this graph is as expected.
    let mut gv_visitor = GraphvizVisitor::new();
    graph.answer.resolve(&mut gv_visitor);

    assert_eq!(
        r#"
digraph G {
  0[label="NumberInput"];
  1[label="NumberInput"];
  2[label="NumberInput"];
  3[label="Sum"];
  0 -> 3;
  1 -> 3;
  4[label="Multiply"];
  0 -> 4;
  2 -> 4;
  5[label="Square"];
  4 -> 5;
  6[label="Answer"];
  3 -> 6;
  5 -> 6;
}
"#
        .trim(),
        gv_visitor.render().unwrap()
    );

    graph.a.update(40);
    graph.b.update(2);

    // a visitor is used to track nodes which have been visited.
    let mut visitor = HashSet::<usize>::new();

    // we can now sum the latest values!
    assert_eq!(graph.answer.resolve_root(&mut visitor).value, 42);

    graph.c.update(2);

    assert_eq!(graph.answer.resolve_root(&mut visitor).value, 12842);
}
