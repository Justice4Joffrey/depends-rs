use std::rc::Rc;

use depends::{derives::Operation, error::EarlyExit, *};

use crate::docs::simple_value::SomeNumber;

#[derive(Operation)]
pub struct Multiply;

impl UpdateDerived<DepRef2<'_, SomeNumber, SomeNumber>, Multiply> for SomeNumber {
    fn update(&mut self, deps: DepRef2<'_, SomeNumber, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.0.value * deps.1.value;
        Ok(())
    }
}

fn main() {
    // Define our inputs to the graph.
    let a = InputNode::new(SomeNumber { value: 7 });
    let b = InputNode::new(SomeNumber { value: 6 });
    // This type has a `Resolve::Output<'_> of 'DepRef2<'_, SomeNumber,
    // SomeNumber>`, so it's compatible with our implementation of `Multiply`
    // above.
    let dependencies = Dependencies2::new(a, Rc::clone(&b));
    let a_times_b = DerivedNode::new(dependencies, Multiply, SomeNumber::default());
    // Any node which holds `SomeNumber` can be used to create more dependencies in
    // this graph.
    let dependencies_2 = Dependencies2::new(a_times_b, b);
    let a_times_b_times_b = DerivedNode::new(dependencies_2, Multiply, SomeNumber::default());
}
