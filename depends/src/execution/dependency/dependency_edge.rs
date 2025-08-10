use std::{marker::PhantomData, ops::Deref};

use super::DependencyState;
use crate::execution::IsDirty;

/// A read reference to the resolved state of a [Dependency](super::Dependency).
/// This is an edge in the dependency graph.
#[derive(Debug)]
pub struct DependencyEdge<'a, T> {
    state: DependencyState,
    data: T,
    phantom: PhantomData<&'a T>,
}

impl<T> DependencyEdge<'_, T> {
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

impl<T> IsDirty for DependencyEdge<'_, T> {
    fn is_dirty(&self) -> bool {
        self.state == DependencyState::Dirty
    }
}

impl<T> Deref for DependencyEdge<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dep_edge() {
        let mut dep_edge = DependencyEdge::new(DependencyState::Dirty, 123_i32);
        assert_eq!(
            "DependencyEdge { state: Dirty, data: 123, phantom: PhantomData<&i32> }",
            format!("{dep_edge:?}")
        );
        assert!(dep_edge.is_dirty());
        dep_edge.state = DependencyState::Clean;
        assert!(!dep_edge.is_dirty());
    }
}
