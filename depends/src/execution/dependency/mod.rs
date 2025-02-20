mod dep_state;
mod dependency_edge;
mod impls;

use std::cell::{Ref, RefCell};

pub use dep_state::DependencyState;
pub use dependency_edge::DependencyEdge;
pub use impls::*;

use super::{HashValue, NodeHash, Resolve};
use crate::execution::{error::ResolveResult, NodeState, Visitor};

/// Short-hand for a reference to a single dependency.
pub type DepRef<'a, T> = DependencyEdge<'a, Ref<'a, NodeState<T>>>;

/// Wraps a dependency and tracks the hashed value each time it's resolved. This
/// allows the resolver to know if a dependency is 'dirty' from the perspective
/// of the Dependee.
#[derive(Debug)]
pub struct Dependency<T> {
    /// The state observed of the inner dependency when it was last resolved.
    last_state: RefCell<Option<NodeHash>>,
    /// The wrapped node.
    dependency: T,
}

impl<T> Dependency<T>
where
    T: Resolve,
    for<'a> <T as Resolve>::Output<'a>: HashValue,
{
    pub fn new(dependency: T) -> Self {
        Self {
            last_state: RefCell::new(None),
            dependency,
        }
    }
}

impl<T> Resolve for Dependency<T>
where
    T: Resolve,
    for<'a> <T as Resolve>::Output<'a>: HashValue,
{
    type Output<'a>
        = DependencyEdge<'a, T::Output<'a>>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        let mut last_state = self.last_state.try_borrow_mut()?;
        let data = self.dependency.resolve(visitor)?;
        let current_state = data.hash_value(&mut visitor.hasher());
        if last_state.map(|s| s == current_state).unwrap_or(false) {
            Ok(DependencyEdge::new(DependencyState::Clean, data))
        } else {
            *last_state = Some(current_state);
            Ok(DependencyEdge::new(DependencyState::Dirty, data))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use serial_test::serial;

    use super::*;
    use crate::execution::{
        identifiable::reset_node_id, internal_test_utils::TestData, HashSetVisitor, InputNode,
        IsDirty,
    };

    #[test]
    #[serial]
    fn test_dependency() {
        reset_node_id();
        let node = InputNode::new(TestData::new(57));
        let dependency = Dependency::new(Rc::clone(&node));
        assert_eq!(
            r#"
Dependency {
    last_state: RefCell {
        value: None,
    },
    dependency: InputNode {
        resolve_state: RefCell {
            value: Updating,
        },
        value: RefCell {
            value: NodeState {
                node_hash: NotHashed,
                value: TestData {
                    inner: 57,
                    recent: [],
                },
            },
        },
        id: 0,
    },
}"#
            .trim(),
            format!("{:#?}", dependency)
        );
        let mut visitor = HashSetVisitor::new();
        {
            let output = dependency.resolve_root(&mut visitor).unwrap();
            assert_eq!(***output, TestData::new(57));
            assert!(output.is_dirty());
        }
        {
            let output = dependency.resolve_root(&mut visitor).unwrap();
            assert_eq!(***output, TestData::new(57));
            assert!(!output.is_dirty());
        }
        node.update(42).unwrap();
        {
            let output = dependency.resolve_root(&mut visitor).unwrap();
            assert_eq!(
                ***output,
                TestData {
                    inner: 42,
                    recent: vec![57]
                }
            );
            assert!(output.is_dirty());
        }
    }
}
