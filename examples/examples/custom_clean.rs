#![allow(dead_code)]

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use depends::{
    core::{
        error::EarlyExit, Clean, DerivedNode, InputNode, Resolve, TargetMut, UpdateDerived,
        UpdateInput,
    },
    derives::{Dependencies, Operation, Value},
    graphviz::GraphvizVisitor,
};

/// A sequence of numbers.
#[derive(Value, Default, Hash)]
struct Sequence {
    value: Vec<i32>,
}

impl UpdateInput for Sequence {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        self.value.push(update);
    }
}

/// A sequence of numbers that tracks the values which have changed since
/// last resolved.
#[derive(Value, Default, Hash)]
#[depends(custom_clean)]
struct EfficientSequence {
    value: Vec<i32>,
    dirty_index: usize,
}

impl UpdateInput for EfficientSequence {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        self.value.push(update);
    }
}

impl Clean for EfficientSequence {
    fn clean(&mut self) {
        // Reset the dirty index to the end of the sequence.
        self.dirty_index = self.value.len();
    }
}

impl EfficientSequence {
    pub fn iter_dirty(&self) -> impl Iterator<Item = i32> + '_ {
        self.value.iter().skip(self.dirty_index).copied()
    }
}

#[derive(Dependencies)]
struct SequenceDependencies {
    sequence: Sequence,
    efficient_sequence: EfficientSequence,
}

#[derive(Operation)]
struct Totals;

impl UpdateDerived for Totals {
    type Input<'a> = SequenceDependenciesRef<'a> where Self: 'a;
    type Target<'a> = TargetMut<'a, SequenceTotals> where Self: 'a;

    fn update_derived(
        SequenceDependenciesRef {
            sequence,
            efficient_sequence,
        }: Self::Input<'_>,
        mut target: Self::Target<'_>,
    ) -> Result<(), EarlyExit> {
        // to calculate the total, we need to sum all the values in the
        // sequence every time this node is resolved.
        target.sequence_value = sequence.value.iter().sum();
        // With a bit of state tracking, however, we can avoid summing the
        // entire sequence every time, and only iterate the values which are
        // new.
        target.efficient_value += efficient_sequence.iter_dirty().sum::<i32>();
        Ok(())
    }
}

/// Our final result which tracks the total value of each sequence.
#[derive(Value, Default, Hash)]
struct SequenceTotals {
    efficient_value: i32,
    sequence_value: i32,
}

fn main() {
    // Create the input nodes
    let sequence = InputNode::new(Sequence::default());
    let efficient_sequence = InputNode::new(EfficientSequence::default());

    // Create the derived node.
    let sequence_dependencies =
        SequenceDependencies::init(Rc::clone(&sequence), Rc::clone(&efficient_sequence));
    let totals = DerivedNode::new(sequence_dependencies, Totals, SequenceTotals::default());

    let mut visitor = GraphvizVisitor::new();
    totals.resolve(&mut visitor).unwrap();
    println!("{}", visitor.render().unwrap());

    let mut visitor = HashSet::<usize>::new();
    for i in 0..=10 {
        println!("------------");
        println!("Iteration {}", i);
        // update the sequences
        sequence.update(i).unwrap();
        efficient_sequence.update(i).unwrap();

        // resolve the totals
        let res = totals.resolve_root(&mut visitor).unwrap();

        println!("total (regular):   {}", res.sequence_value);
        println!("total (efficient): {}", res.efficient_value);

        // The values are equal, but the performance implications are
        // different.
        assert_eq!(res.sequence_value, res.efficient_value);
    }
}
