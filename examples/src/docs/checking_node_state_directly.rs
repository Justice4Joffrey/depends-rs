use depends::{
    derives::{Operation, Value},
    error::EarlyExit,
    DepRef2, IsDirty, UpdateDerived,
};

// ANCHOR: is_dirty
#[derive(Value, Hash)]
pub struct StuffToBuy {
    amount: i32,
    last_purchase_time: i64,
}

#[derive(Operation)]
pub struct CheckBankBalance;

// A dependency of time and money.
impl UpdateDerived<DepRef2<'_, i64, i32>, CheckBankBalance> for StuffToBuy {
    fn update(&mut self, deps: DepRef2<'_, i64, i32>) -> Result<(), EarlyExit> {
        // Is dirty is a trait implemented on all dependencies to indicate
        // that the inner value of this node has changed since last observed.
        if !deps.1.is_dirty() {
            // Time's always changing, we don't need to check it.
            return Ok(());
        }
        // It's been a while since we've bought anything and we've just been
        // paid.
        if deps.0.value() - self.last_purchase_time > 24 * 60 * 60 {
            self.last_purchase_time = *deps.0.value();
            self.amount = deps.1.value() / 10;
        }
        Ok(())
    }
}
// ANCHOR_END: is_dirty
