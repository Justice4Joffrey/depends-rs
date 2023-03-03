use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::execution::{
    identifiable::next_node_id, Clean, Identifiable, LeafNodeState, LeafState, Named, NodeRef,
    NodeState, Resolve, UpdateLeaf, Visitor,
};

/// A node which has no dependencies. Leaf nodes receive their state from
/// _outside_ of the graph structure from calls to [update](Self::update).
pub struct LeafNode<T> {
    data: RefCell<NodeState<T, LeafNodeState>>,
    id: usize,
}

impl<T: Named> Named for LeafNode<T> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Named> Identifiable for LeafNode<T> {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T> LeafNode<T>
where
    T: UpdateLeaf,
{
    /// Wrap this leaf in a node.
    pub fn new(data: T) -> Rc<LeafNode<T>> {
        Self::new_with_id(data, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(data: T, id: usize) -> Rc<LeafNode<T>> {
        Rc::new(Self {
            data: RefCell::new(NodeState::new_leaf(data)),
            id,
        })
    }

    /// The public interface to provide data to mutate the inner leaf.
    pub fn update(&self, input: T::Input) {
        let mut node_state = self.data.borrow_mut();
        // Flush any changes since it was resolved.
        if node_state.state() == LeafState::Resolving {
            node_state.clean();
        }
        *(node_state.state_mut()) = LeafState::Updating;
        node_state.deref_mut().update_mut(input);
    }
}

impl<T> Resolve for LeafNode<T>
where
    T: UpdateLeaf,
{
    type Output<'a> = NodeRef<'a, T, LeafNodeState> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        visitor.touch(self);
        if visitor.visit(self) {
            let mut node_state = self.data.borrow_mut();
            // Ensures `update` changes are only flushed once.
            match node_state.state() {
                LeafState::Updating => *node_state.state_mut() = LeafState::Resolving,
                LeafState::Resolving => {
                    node_state.clean();
                    *node_state.state_mut() = LeafState::Resolved
                }
                LeafState::Resolved => {}
            }
            // The hash is only set when this node is being read.
            node_state.update_node_hash(&mut visitor.hasher());
        }
        visitor.leave(self);
        self.data.borrow()
    }
}
