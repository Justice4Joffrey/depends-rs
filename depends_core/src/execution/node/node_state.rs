use std::{
    hash::Hasher,
    ops::{Deref, DerefMut},
};

use super::NodeHash;
use crate::execution::{Clean, HashValue, Named};

/// A wrapper for some data `T`, tracking some context around the data's
/// computation state.
#[derive(Debug)]
pub struct NodeState<T> {
    /// A value representing the unique state of the data (i.e. the Hash).
    node_hash: NodeHash,
    /// The data being wrapped.
    data: T,
}

impl<T: HashValue> NodeState<T> {
    pub fn new(data: T) -> Self {
        Self {
            node_hash: NodeHash::default(),
            data,
        }
    }

    pub fn node_hash(&self) -> NodeHash {
        self.node_hash
    }

    pub fn node_hash_mut(&mut self) -> &mut NodeHash {
        &mut self.node_hash
    }

    /// Update the stored hash value of the data.
    pub fn update_node_hash(&mut self, hasher: &mut impl Hasher) {
        self.node_hash = self.data.hash_value(hasher)
    }
}

impl<T: HashValue> HashValue for NodeState<T> {
    fn hash_value(&self, _: &mut impl Hasher) -> NodeHash {
        self.node_hash
    }
}

impl<T: Named> Named for NodeState<T> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Clean> Clean for NodeState<T> {
    fn clean(&mut self) {
        self.data.clean()
    }
}

impl<T> Deref for NodeState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for NodeState<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
