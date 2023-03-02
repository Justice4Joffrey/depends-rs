use std::{cell::RefCell, rc::Rc};

use crate::execution::{
    identifiable::next_node_id, Clean, Identifiable, Named, NodeRef, NodeState, Resolve,
    ResolveState, UpdateLeaf, Visitor,
};

pub struct LeafNode<T> {
    data: RefCell<NodeState<T>>,
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
            data: RefCell::new(NodeState::new(data)),
            id,
        })
    }

    /// The public interface to provide data to mutate the inner leaf.
    pub fn update(&self, input: T::Input) {
        let mut node_state = self.data.borrow_mut();
        // Flush any changes since it was resolved.
        if node_state.state() == ResolveState::Resolving {
            node_state.clean();
        }
        *(node_state.state_mut()) = ResolveState::Updating;
        node_state.data_mut().update_mut(input);
    }
}

impl<T> Resolve for LeafNode<T>
where
    T: UpdateLeaf,
{
    type Output<'a> = NodeRef<'a, T> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        visitor.touch(self);
        if visitor.visit(self) {
            let mut node_state = self.data.borrow_mut();
            // Ensures `update` changes are only flushed once.
            match node_state.state() {
                ResolveState::Updating => *node_state.state_mut() = ResolveState::Resolving,
                ResolveState::Resolving => {
                    node_state.clean();
                    *node_state.state_mut() = ResolveState::Resolved
                }
                ResolveState::Resolved => {}
            }
            // The hash is only set when this node is being read.
            node_state.update_node_hash();
        }
        visitor.leave(self);
        self.data.borrow()
    }
}
