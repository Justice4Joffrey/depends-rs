use std::{
    hash::Hasher,
    ops::{Deref, DerefMut},
};

use super::NodeHash;
use crate::execution::{Clean, HashValue, Named};

/// A wrapper for some value `T`, tracking some context around the value's
/// computation state.
#[derive(Debug)]
pub struct NodeState<T> {
    /// A value representing the unique state of the value (i.e. the Hash).
    node_hash: NodeHash,
    /// The value being wrapped.
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

    /// Update the stored hash value of the value.
    pub fn update_node_hash(&mut self, hasher: &mut impl Hasher) {
        self.node_hash = self.data.hash_value(hasher)
    }

    pub fn data(&self) -> &T {
        &self.data
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_state() {
        let mut state = NodeState::new(123_i32);
        assert_eq!(
            "NodeState { node_hash: NotHashed, data: 123 }",
            format!("{:?}", state)
        );
        assert_eq!(<NodeState<i32> as Named>::name(), "i32");
        let hasher = &mut std::collections::hash_map::DefaultHasher::new();
        state.update_node_hash(hasher);
        assert_eq!(state.node_hash(), NodeHash::Hashed(14370432302296844161));
        assert_eq!(
            state.node_hash_mut(),
            &mut NodeHash::Hashed(14370432302296844161)
        );
        state.clean();
        assert_eq!(state.data(), &123);
    }
}
