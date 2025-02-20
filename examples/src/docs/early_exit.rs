use depends::{derives::Operation, error::EarlyExit, DepRef, UpdateDerived};

use crate::docs::simple_value::SomeNumber;

// ANCHOR: early_exit
#[derive(Operation)]
pub struct CheckAllIsOk;

impl UpdateDerived<DepRef<'_, SomeNumber>, CheckAllIsOk> for SomeNumber {
    fn update(&mut self, deps: DepRef<'_, SomeNumber>) -> Result<(), EarlyExit> {
        if deps.value >= 100 {
            return Err(EarlyExit::new("Things are a bit too spicy!"));
        }
        self.value = deps.value;
        Ok(())
    }
}
// ANCHOR_END: early_exit
