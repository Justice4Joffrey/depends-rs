#![allow(dead_code)]

use std::rc::Rc;

use depends::{
    core::{Dependency, Depends, HashValue, LeafNodeRc, UpdateDependeeMut, UpdateLeafMut},
    derives::{Dependee, Dependencies, Leaf},
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
        self.value = left.data().data().value + 2 * right.data().data().value;
    }
}

impl UpdateDependeeMut for Multiply {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>) {
        let ComponentsRef { left, right } = input;
        self.value = left.data().data().value * right.data().data().value;
    }
}

pub struct MyGraph {
    pub a: LeafNodeRc<NumberInput>,
    pub b: LeafNodeRc<NumberInput>,
    pub c: LeafNodeRc<NumberInput>,
    pub answer: Rc<AnswerNode>,
}

pub fn my_graph() -> MyGraph {
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

    MyGraph { a, b, c, answer }
}
