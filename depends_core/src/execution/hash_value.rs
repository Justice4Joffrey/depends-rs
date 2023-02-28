use std::{cell::Ref, ops::Deref};

use super::{NodeHash, NodeState};

/// A unique number derived from the internal state of a node.
pub trait HashValue {
    /// Either a unique number, or a value detailing that this node cannot be
    /// hashed.
    fn hash_value(&self) -> NodeHash;
}

impl<T: HashValue> HashValue for Ref<'_, NodeState<T>> {
    fn hash_value(&self) -> NodeHash {
        self.deref().hash_value()
    }
}
