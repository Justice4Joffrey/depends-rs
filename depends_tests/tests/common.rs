#![allow(dead_code)]

use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use depends::{
    core::{Dependency, Depends, LeafNode, UpdateDependee, UpdateLeaf},
    derives::{dependencies, Dependee, Leaf},
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
/// this must be wrapped in a struct marked as `#[dependencies]` as shown.
#[dependencies]
pub struct Components {
    left: LeafNode<NumberInput>,
    right: LeafNode<NumberInput>,
}

#[dependencies]
pub struct AnswerComponents {
    left: SumNode,
    right: MultiplyNode,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = AnswerComponents, node_name = AnswerNode)]
pub struct Answer {
    value: i32,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = Dependency<Rc<LeafNode<NumberInput>>>, node_name = SquareNode)]
pub struct Square {
    value: i32,
}

#[derive(Dependee, Default, Hash)]
#[depends(dependencies = Components, node_name = SumNode)]
pub struct Sum {
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
        self.value = left.value + 2 * right.value;
    }
}

impl UpdateDependee for Multiply {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.value * right.value;
    }
}

pub struct MyGraph {
    pub a: Rc<LeafNode<NumberInput>>,
    pub b: Rc<LeafNode<NumberInput>>,
    pub c: Rc<LeafNode<NumberInput>>,
    pub answer: Rc<AnswerNode>,
}

pub fn my_graph() -> MyGraph {
    let a = NumberInput::default().into_leaf();
    let b = NumberInput::default().into_leaf();
    let c = NumberInput::default().into_leaf();

    let sum = Sum::default().into_node(Components::new(Rc::clone(&a), Rc::clone(&b)));
    let multiply = Multiply::default().into_node(Components::new(Rc::clone(&a), Rc::clone(&c)));
    let answer =
        Answer::default().into_node(AnswerComponents::new(Rc::clone(&sum), Rc::clone(&multiply)));

    MyGraph { a, b, c, answer }
}
