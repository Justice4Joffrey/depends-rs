use crate::execution::error::EarlyExit;

pub trait UpdateDerived<T, F> {
    fn update(&mut self, deps: T) -> Result<(), EarlyExit>;
}
