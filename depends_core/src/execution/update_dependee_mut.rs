use std::cell::Ref;

use crate::execution::{clean::Clean, depends::Depends, Named, NodeState};

pub type Dep<'a, T> = Ref<'a, NodeState<T>>;

/// Describe how a `Dependee` updates its internal state when its dependencies
/// change.
pub trait UpdateDependeeMut: Depends + Clean + Named {
    fn update_mut(&mut self, input: <Self as Depends>::Input<'_>);
}
