use crate::execution::depends::Depends;

/// Describe how a `Dependee` updates its internal state when its dependencies
/// change. As with [UpdateLeafMut](super::UpdateLeafMut), correct
/// implementation of this involves a contract with the [Clean](super::Clean)
/// trait.
pub trait UpdateDependeeMut: Depends {
    ///
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>);
}
