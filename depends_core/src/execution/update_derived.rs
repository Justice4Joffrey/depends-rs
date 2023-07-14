use crate::execution::{error::EarlyExit, Named};

pub trait UpdateDerived: Named {
    type Input<'a>
    where
        Self: 'a;
    type Target<'a>
    where
        Self: 'a;

    fn update_derived(input: Self::Input<'_>, target: Self::Target<'_>) -> Result<(), EarlyExit>;
}
