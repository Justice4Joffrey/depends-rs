use std::{
    cell::{BorrowError, Ref, RefCell},
    ops::DerefMut,
    rc::Rc,
};

use crate::execution::{
    error::ResolveResult, identifiable::next_node_id, Clean, Identifiable, InputState, Named,
    NodeRef, NodeState, Resolve, UpdateInput, Visitor,
};

/// A node which has no dependencies. Leaf nodes receive their state from
/// _outside_ of the graph structure from calls to [update](Self::update).
#[derive(Debug)]
pub struct InputNode<T> {
    /// The resolve state of this node. This is used to ensure that a
    /// node is cleaned only once per resolve.
    resolve_state: RefCell<InputState>,
    data: RefCell<NodeState<T>>,
    id: usize,
}

impl<T: Named> Named for InputNode<T> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Named> Identifiable for InputNode<T> {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T> InputNode<T>
where
    T: UpdateInput,
{
    /// Wrap this leaf in a node.
    pub fn new(data: T) -> Rc<Self> {
        Self::new_with_id(data, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(data: T, id: usize) -> Rc<Self> {
        Rc::new(Self {
            resolve_state: RefCell::new(InputState::default()),
            data: RefCell::new(NodeState::new(data)),
            id,
        })
    }

    /// The public interface to provide data to mutate the inner leaf.
    pub fn update(&self, input: T::Update) -> ResolveResult<()> {
        let mut node_state = self.data.try_borrow_mut()?;
        let mut resolve_state = self.resolve_state.try_borrow_mut()?;
        // Flush any changes since it was resolved.
        if *resolve_state == InputState::Resolving {
            node_state.clean();
        }
        *resolve_state = InputState::Updating;
        node_state.deref_mut().update_mut(input);
        Ok(())
    }

    pub fn data(&self) -> Result<Ref<'_, NodeState<T>>, BorrowError> {
        self.data.try_borrow()
    }
}

impl<T> Resolve for InputNode<T>
where
    T: UpdateInput,
{
    type Output<'a> = NodeRef<'a, T> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        visitor.touch(self, None);
        if visitor.visit(self) {
            let mut node_state = self.data.try_borrow_mut()?;
            let mut resolve_state = self.resolve_state.try_borrow_mut()?;
            // Ensures `update` changes are only flushed once.
            match *resolve_state {
                InputState::Updating => *resolve_state = InputState::Resolving,
                InputState::Resolving => {
                    node_state.clean();
                    *resolve_state = InputState::Resolved
                }
                InputState::Resolved => {}
            }
            // The hash is only set when this node is being read.
            node_state.update_node_hash(&mut visitor.hasher());
        }
        visitor.leave(self);
        Ok(self.data.try_borrow()?)
    }
}
