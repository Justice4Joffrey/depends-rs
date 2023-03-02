use crate::execution::depends::Depends;

/// Describe how a `Dependee` updates its internal state when its dependencies
/// change. As with [UpdateLeaf](super::UpdateLeaf), correct
/// implementation of this involves a contract with the [Clean](super::Clean)
/// trait.
pub trait UpdateDependee: Depends {
    /// Update the state of this node given the latest state of its
    /// dependencies.
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>);
}
