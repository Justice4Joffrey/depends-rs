use std::{cell::RefCell, rc::Rc};

use crate::execution::{
    identifiable::next_node_id, Clean, Dep, Identifiable, IsDirty, Named, NodeState, Resolve,
    State, UpdateLeafMut, Visitor,
};

pub type LeafNodeRc<T> = Rc<LeafNode<T>>;

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
    T: UpdateLeafMut,
{
    /// Wrap this leaf in a node.
    pub fn new(data: T) -> LeafNodeRc<T> {
        Self::new_with_id(data, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(data: T, id: usize) -> LeafNodeRc<T> {
        Rc::new(Self {
            data: RefCell::new(NodeState::new(data)),
            id,
        })
    }

    /// The public interface to provide data to mutate the inner leaf.
    pub fn update(&self, input: T::Input) {
        let mut node_state = self.data.borrow_mut();
        node_state.data_mut().update_mut(input);
        *node_state.state_mut() = State::Dirty;
    }
}

impl<T> Resolve for LeafNode<T>
where
    T: UpdateLeafMut,
{
    type Output<'a> = Dep<'a, T> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        visitor.visit(self);
        self.data.borrow()
    }

    fn clean(&self, visitor: &mut impl Visitor) {
        visitor.visit(self);
        self.data.borrow_mut().clean()
    }
}

impl<T> IsDirty for LeafNode<T> {
    fn is_dirty(&self) -> bool {
        self.data.borrow().state() == State::Dirty
    }
}
