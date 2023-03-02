use std::marker::PhantomData;

use super::DependencyState;
use crate::execution::IsDirty;

/// A read reference to the resolved state of a [Dependency](super::Dependency)
#[derive(Debug)]
pub struct DepRef<'a, T> {
    state: DependencyState,
    data: T,
    phantom: PhantomData<&'a T>,
}

impl<T> DepRef<'_, T> {
    pub fn new(state: DependencyState, data: T) -> Self {
        Self {
            state,
            data,
            phantom: PhantomData,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> IsDirty for DepRef<'_, T> {
    fn is_dirty(&self) -> bool {
        self.state == DependencyState::Dirty
    }
}
