use std::{marker::PhantomData, ops::Deref};

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
}

impl<T> IsDirty for DepRef<'_, T> {
    fn is_dirty(&self) -> bool {
        self.state == DependencyState::Dirty
    }
}

impl<T> Deref for DepRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
