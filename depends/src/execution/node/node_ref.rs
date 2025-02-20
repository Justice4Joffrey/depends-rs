use std::cell::Ref;

use crate::NodeState;

/// Short-hand for the output read-reference of a node.
pub type NodeRef<'a, T> = Ref<'a, NodeState<T>>;
