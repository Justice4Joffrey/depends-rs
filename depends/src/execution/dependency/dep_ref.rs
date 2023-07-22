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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dep_ref() {
        let mut dep_ref = DepRef::new(DependencyState::Dirty, 123_i32);
        assert_eq!(
            "DepRef { state: Dirty, data: 123, phantom: PhantomData<&i32> }",
            format!("{:?}", dep_ref)
        );
        assert!(dep_ref.is_dirty());
        dep_ref.state = DependencyState::Clean;
        assert!(!dep_ref.is_dirty());
    }
}
