use parking_lot::RwLockReadGuard;

use super::NodeState;

/// Short-hand for the output read-reference of a node.
pub type NodeRef<'a, T, N> = RwLockReadGuard<'a, NodeState<T, N>>;
