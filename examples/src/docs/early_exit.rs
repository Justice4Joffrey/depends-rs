use depends::{derives::Operation, error::EarlyExit, SingleRef, TargetMut, UpdateDerived};

use crate::docs::simple_value::SomeNumber;

// ANCHOR: early_exit
#[derive(Operation)]
pub struct CheckAllIsOk;

impl UpdateDerived for CheckAllIsOk {
    type Input<'a> = SingleRef<'a, SomeNumber>;
    type Target<'a> = TargetMut<'a, SomeNumber>;

    fn update_derived(
        input: Self::Input<'_>,
        mut target: Self::Target<'_>,
    ) -> Result<(), EarlyExit> {
        if input.value >= 100 {
            return Err(EarlyExit::new("Things are a bit too spicy!"));
        }
        target.value = input.value;
        Ok(())
    }
}
// ANCHOR_END: early_exit
