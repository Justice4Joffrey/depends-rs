use depends::{
    derives::{Dependencies, Operation, Value},
    error::EarlyExit,
    IsDirty, TargetMut, UpdateDerived,
};

// ANCHOR: is_dirty
#[derive(Value, Hash)]
pub struct StuffToBuy {
    amount: i32,
    last_purchase_time: i64,
}

#[derive(Operation)]
pub struct CheckBankBalance;

#[derive(Dependencies)]
pub struct TimeAndMoney {
    pub time: i64,
    pub money: i32,
}

impl UpdateDerived for CheckBankBalance {
    type Input<'a> = TimeAndMoneyRef<'a> where Self: 'a;
    type Target<'a> = TargetMut<'a, StuffToBuy> where Self: 'a;

    fn update_derived(
        TimeAndMoneyRef { time, money }: Self::Input<'_>,
        mut target: Self::Target<'_>,
    ) -> Result<(), EarlyExit> {
        // Is dirty is a trait implemented on all dependencies to indicate
        // that the inner value of this node has changed since last observed.
        if !money.is_dirty() {
            // Time's always changing, we don't need to check it.
            return Ok(());
        }
        let time = ***time;
        let money = ***money;
        // It's been a while since we've bought anything and we've just been
        // paid.
        if time - target.last_purchase_time > 24 * 60 * 60 {
            target.last_purchase_time = time;
            target.amount = money / 10;
        }
        Ok(())
    }
}
// ANCHOR_END: is_dirty
