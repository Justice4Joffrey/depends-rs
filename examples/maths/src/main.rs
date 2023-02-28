use std::{collections::HashSet, rc::Rc};

use depends::{
    core::{Dependency, Depends, HashValue, LeafNodeRc, Resolve, UpdateDependeeMut, UpdateLeafMut},
    derives::{Dependee, Dependencies, Leaf},
    graphviz::GraphvizVisitor,
};

/// A number which can be edited from the _outside_ i.e. has _no_ dependencies.
#[derive(Leaf, Default)]
pub struct NumberInput {
    value: i32,
}

impl HashValue for NumberInput {
    fn hash_value(&self) -> depends::core::NodeHash {
        depends::core::NodeHash::Hashed(self.value as usize)
    }
}

impl UpdateLeafMut for NumberInput {
    type Input = i32;

    fn update_mut(&mut self, input: Self::Input) {
        // Implementing this trait will provide a way for code outside of this graph to
        // change its internal state. This is just a simple replace for now.
        self.value = input;
    }
}

/// Any *derived* node must state its dependencies. If there are more than one,
/// this must be wrapped in a struct which derives [Dependencies] as shown.
#[derive(Dependencies)]
#[depends(ref_name = ComponentsRef)]
pub struct Components {
    left: Dependency<LeafNodeRc<NumberInput>>,
    right: Dependency<LeafNodeRc<NumberInput>>,
}

#[derive(Dependencies)]
#[depends(ref_name = AnswerComponentsRef)]
pub struct AnswerComponents {
    left: Dependency<Rc<SumNode>>,
    right: Dependency<Rc<MultiplyNode>>,
}

#[derive(Dependee, Default)]
#[depends(dependencies = AnswerComponents, node_name = AnswerNode)]
pub struct Answer {
    value: i32,
}

impl HashValue for Answer {
    fn hash_value(&self) -> depends::core::NodeHash {
        depends::core::NodeHash::Hashed(self.value as usize)
    }
}

#[derive(Dependee, Default)]
#[depends(dependencies = Components, node_name = SumNode)]
pub struct Sum {
    value: i32,
}

impl HashValue for Sum {
    fn hash_value(&self) -> depends::core::NodeHash {
        depends::core::NodeHash::Hashed(self.value as usize)
    }
}

#[derive(Dependee, Default)]
#[depends(dependencies = Components, node_name = MultiplyNode)]
pub struct Multiply {
    value: i32,
}

impl HashValue for Multiply {
    fn hash_value(&self) -> depends::core::NodeHash {
        depends::core::NodeHash::Hashed(self.value as usize)
    }
}

impl UpdateDependeeMut for Sum {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.data().data().value + right.data().data().value;
    }
}

impl UpdateDependeeMut for Answer {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let AnswerComponentsRef { left, right } = input;
        // TODO oppressive dereferencing
        self.value = left.data().data().value + 2 * right.data().data().value;
    }
}

impl UpdateDependeeMut for Multiply {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.data().data().value * right.data().data().value;
    }
}

struct MyGraph {
    a: LeafNodeRc<NumberInput>,
    b: LeafNodeRc<NumberInput>,
    c: LeafNodeRc<NumberInput>,
    answer: Rc<AnswerNode>,
}

fn main() {
    let a = NumberInput::default().into_leaf();
    let b = NumberInput::default().into_leaf();
    let c = NumberInput::default().into_leaf();

    let sum = Sum::default().into_node(Components::new(
        Dependency::new(Rc::clone(&a)),
        Dependency::new(Rc::clone(&b)),
    ));
    let multiply = Multiply::default().into_node(Components::new(
        Dependency::new(Rc::clone(&a)),
        Dependency::new(Rc::clone(&c)),
    ));
    let answer = Answer::default().into_node(AnswerComponents::new(
        Dependency::new(Rc::clone(&sum)),
        Dependency::new(Rc::clone(&multiply)),
    ));

    let graph = MyGraph { a, b, c, answer };

    // check the graphviz of this graph is as expected.
    let mut gv_visitor = GraphvizVisitor::new();
    graph.answer.resolve(&mut gv_visitor);
    assert_eq!(
        r#"
digraph G {
  5[label="Answer"];
  3[label="Sum"];
  0[label="NumberInput"];
  1[label="NumberInput"];
  4[label="Multiply"];
  2[label="NumberInput"];
  0 -> 3;
  1 -> 3;
  0 -> 4;
  2 -> 4;
  3 -> 5;
  4 -> 5;
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
    assert_eq!(graph.answer.resolve(&mut visitor).data().value, 42);

    graph.c.update(2);

    // between each run, the visitor must be cleared to visit all nodes.
    visitor.clear();

    assert_eq!(graph.answer.resolve(&mut visitor).data().value, 202);
}
