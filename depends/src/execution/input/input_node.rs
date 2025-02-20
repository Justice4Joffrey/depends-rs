use std::{
    cell::{BorrowError, RefCell},
    ops::DerefMut,
    rc::Rc,
};

use crate::execution::{
    error::ResolveResult, identifiable::next_node_id, Clean, Identifiable, InputState, Named,
    NodeRef, NodeState, Resolve, UpdateInput, Visitor,
};

/// # Input Node
///
/// Input Nodes are the leaves of the graph. They wrap a value of some type
/// `T` and provide a public interface to mutate it.
///
/// ## UpdateInput
///
/// To construct an input node, the value must implement the [UpdateInput]
/// trait. This specifies the type of the update and how it affects the
/// wrapped value.
///
/// ## Updating values
///
/// `InputNode` uses interior mutability to allow the wrapped value to be
/// updated with a shared reference.
///
/// To update the value, use [update](Self::update).
/// A node which can update values from outside of the graph via
/// [update](Self::update).
///
/// ```rust
/// # use depends::{InputNode, UpdateInput};
/// # use depends::derives::Value;
/// #[derive(Value, Hash)]
/// struct MyType {
///     inner: String,
/// }
///
/// impl UpdateInput for MyType {
///     type Update = String;
///
///     fn update_mut(&mut self, update: Self::Update) {
///         self.inner.extend(update.chars());
///     }
/// }
///
/// // Create an input node.
/// let input = InputNode::new(MyType {
///     inner: "Hello, ".to_string(),
/// });
///
/// // Update the internal value. This will fail if the node is currently
/// // being written to.
/// input.update("world!".to_string()).unwrap();
///
/// assert_eq!(input.value().unwrap().inner, "Hello, world!");
/// ```
#[derive(Debug)]
pub struct InputNode<T> {
    /// The resolve state of this node. This is used to ensure that a
    /// node is cleaned only once per resolve.
    resolve_state: RefCell<InputState>,
    /// The inner value of this node.
    value: RefCell<NodeState<T>>,
    /// Unique runtime identifier.
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
    pub fn new(value: T) -> Rc<Self> {
        Self::new_with_id(value, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(value: T, id: usize) -> Rc<Self> {
        Rc::new(Self {
            resolve_state: RefCell::new(InputState::default()),
            value: RefCell::new(NodeState::new(value)),
            id,
        })
    }

    /// The public interface to provide data to mutate the inner value via
    /// a shared reference.
    pub fn update(&self, input: T::Update) -> ResolveResult<()> {
        let mut node_state = self.value.try_borrow_mut()?;
        let mut resolve_state = self.resolve_state.try_borrow_mut()?;
        // Flush any changes since it was resolved.
        if *resolve_state == InputState::Resolving {
            node_state.clean();
        }
        *resolve_state = InputState::Updating;
        node_state.deref_mut().update_mut(input);
        Ok(())
    }

    /// Access the inner value.
    pub fn value(&self) -> Result<NodeRef<'_, T>, BorrowError> {
        self.value.try_borrow()
    }
}

impl<T> Resolve for InputNode<T>
where
    T: UpdateInput,
{
    type Output<'a>
        = NodeRef<'a, T>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        visitor.touch(self, None);
        if visitor.visit(self) {
            let mut node_state = self.value.try_borrow_mut()?;
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
        Ok(self.value.try_borrow()?)
    }
}
