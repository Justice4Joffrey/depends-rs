use std::{
    hash::Hasher,
    ops::{Deref, DerefMut},
};

use super::NodeHash;
use crate::execution::{Clean, Depends, HashValue, LeafState, Named, UpdateDependee, UpdateLeaf};

/// Dependee nodes don't have custom fields.
#[doc(hidden)]
pub struct DependeeNodeState;

/// Custom state for a [LeafNode](crate::execution::LeafNode).
#[derive(Default)]
pub struct LeafNodeState {
    /// Tracks whether this node is being updated or resolved.
    state: LeafState,
}

/// A wrapper for some data `T`, tracking some context around the data's
/// computation state.
pub struct NodeState<T, N = DependeeNodeState> {
    /// A value representing the unique state of the data (i.e. the Hash).
    node_hash: NodeHash,
    /// The data being wrapped.
    data: T,
    /// Optional node-type specific behaviour.
    node_type: N,
}

impl<T: HashValue, N> NodeState<T, N> {
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

impl<T: UpdateLeaf> NodeState<T, LeafNodeState> {
    pub fn new_leaf(data: T) -> Self {
        Self {
            node_hash: NodeHash::default(),
            data,
            node_type: LeafNodeState::default(),
        }
    }

    pub fn state(&self) -> LeafState {
        self.node_type.state
    }

    pub fn state_mut(&mut self) -> &mut LeafState {
        &mut self.node_type.state
    }
}

impl<T: UpdateLeaf> UpdateLeaf for NodeState<T, LeafNodeState> {
    type Input = T::Input;

    fn update_mut(&mut self, input: Self::Input) {
        self.data.update_mut(input)
    }
}

impl<T: UpdateDependee> NodeState<T> {
    pub fn new_dependee(data: T) -> Self {
        Self {
            node_hash: NodeHash::default(),
            data,
            node_type: DependeeNodeState,
        }
    }
}

impl<T: UpdateDependee> UpdateDependee for NodeState<T> {
    fn update_mut(&mut self, input: Self::Input<'_>) {
        self.data.update_mut(input)
    }
}

impl<T: Depends> Depends for NodeState<T> {
    type Input<'a> = T::Input<'a> where Self: 'a;
}

impl<T: HashValue, N> HashValue for NodeState<T, N> {
    fn hash_value(&self, _: &mut impl Hasher) -> NodeHash {
        self.node_hash
    }
}

impl<T: Named, N> Named for NodeState<T, N> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Clean, N> Clean for NodeState<T, N> {
    fn clean(&mut self) {
        self.data.clean()
    }
}

impl<T, N> Deref for NodeState<T, N> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, N> DerefMut for NodeState<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
