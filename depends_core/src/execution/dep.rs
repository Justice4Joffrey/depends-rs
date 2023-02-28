use std::cell::Ref;

use super::NodeState;

/// Short-hand for the output read-reference of a node.
pub type Dep<'a, T> = Ref<'a, NodeState<T>>;
