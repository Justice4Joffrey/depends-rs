use std::cell::Ref;

use super::NodeState;

/// Short-hand for the output read-reference of a node.
pub type NodeRef<'a, T, N> = Ref<'a, NodeState<T, N>>;
