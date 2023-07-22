mod derived_node;

use std::cell::RefMut;

pub use derived_node::DerivedNode;

use crate::execution::NodeState;

/// Short-hand for the type of a mutable reference to a node.
pub type TargetMut<'a, T> = RefMut<'a, NodeState<T>>;
