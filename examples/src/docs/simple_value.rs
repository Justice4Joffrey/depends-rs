use depends::{derives::Value, DepRef, UpdateDerived, UpdateInput};
use serial_test::serial;

#[rustfmt::skip]
// Keep these separate to make the example clearer.
#[derive(Debug, Default, PartialEq, Eq)]
// ANCHOR: update_input
// ANCHOR: some_number
#[derive(Value, Hash)]
pub struct SomeNumber {
    pub value: i32,
}
// ANCHOR_END: some_number

impl UpdateInput for SomeNumber {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        // Simply replace the value with the update.
        self.value = update;
    }
}
// ANCHOR_END: update_input

use depends::{derives::Operation, error::EarlyExit};

// ANCHOR: square
#[derive(Operation)]
pub struct Square;

impl UpdateDerived<DepRef<'_, SomeNumber>, Square> for SomeNumber {
    fn update(&mut self, deps: DepRef<'_, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.value.pow(2);
        Ok(())
    }
}
// ANCHOR_END: square

#[serial]
#[test]
#[rustfmt::skip]
fn test_update_input() {
use depends::InputNode;
// ANCHOR: create_simple_input
// `input` is an `Rc`, so can be cloned and passed to dependents.
let input = InputNode::new(SomeNumber { value: 2 });

// The `update` method allows us to change the value of the node. It will
// fail if the node is currently being read/written to.
input.update(6).unwrap();

// The `value` method allows us to read the value of the node.
assert_eq!(input.value().unwrap().value, 6);
// ANCHOR_END: create_simple_input
}

#[test]
#[serial]
#[rustfmt::skip]
fn test_update_derived() {
use std::rc::Rc;
use depends::{DerivedNode, InputNode, Dependency};

let input = InputNode::new(SomeNumber { value: 2 });
let derived = DerivedNode::new(
    // Since `Square` only has one dependency, we can use the standard
    // `Dependency` type.
    Dependency::new(Rc::clone(&input)),
    Square,
    SomeNumber { value: 0 },
);

// To get a value from the derived node, we'll need to resolve the graph.
// More on that later!

let _ = input;
let _ = derived;
}
