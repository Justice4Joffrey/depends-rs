use std::{cell::Ref, hash::Hasher, ops::Deref};

use super::{NodeHash, NodeState};

/// A unique number derived from the internal state of a node.
pub trait HashValue {
    /// Either a unique number, or a value detailing that this node cannot be
    /// hashed.
    fn hash_value(&self, hasher: &mut impl Hasher) -> NodeHash;
}

impl<T: HashValue, N> HashValue for Ref<'_, NodeState<T, N>> {
    fn hash_value(&self, hasher: &mut impl Hasher) -> NodeHash {
        self.deref().hash_value(hasher)
    }
}
