use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

pub use hrtb_workaround::IsDirtyInferenceWorkaround;

use crate::execution::{
    error::ResolveError, next_node_id, Clean, HashValue, Identifiable, IsDirty, Named, NodeState,
    Resolve, UpdateDerived, Visitor,
};

/// # Derived Node
///
/// A node which manages a value which depends on other nodes. This node will
/// keep track of the hash value of any dependencies it has and recompute
/// the internal value if any of its dependencies appear to have changed.
///
/// ## Dependencies
///
/// A derived node must specify its dependencies. This can be a single node,
/// wrapped in a [Dependency](crate::Dependency), or a struct with multiple
/// node types which has derived [Dependencies](crate::derives::Dependencies).
///
/// ## Operation
///
/// Along with the dependencies, a derived node must specify a function type
/// which outlines how to transform the target (wrapped value), given the
/// state of the dependencies.
///
/// For more, see the [Operation](crate::derives::Operation) macro.
///
/// ## Resolving nodes
///
/// Nodes can be [Resolved](Resolve) to compute and return a reference to the
/// internal value. To do so, the node must be passed a [Visitor] which will
/// be used to traverse the graph and compute the value.
///
/// > Be sure to use the same visitor between calls to `resolve`, as the
/// > visitor is responsible for determining node hashes, and this will
/// > not be consistent between different visitor instances.
///
/// ```
/// # use std::rc::Rc;
/// # use depends::{DerivedNode, HashSetVisitor, InputNode, Resolve, TargetMut, UpdateDerived};
/// # use depends::derives::Operation;
/// # use depends::error::EarlyExit;
/// # use depends_derives::Dependencies;
/// # #[derive(Operation)]
/// # struct Concat;
/// # impl UpdateDerived for Concat {
/// #     type Input<'a> = TwoStringsRef<'a> where Self: 'a;
/// #     type Target<'a> = TargetMut<'a, String> where Self: 'a;
/// #     fn update_derived(
/// #         TwoStringsRef { first, second }: Self::Input<'_>,
/// #         mut target: Self::Target<'_>,
/// #     ) -> Result<(), EarlyExit> {
/// #         **target = format!("{} {}", first.value(), second.value());
/// #         Ok(())
/// #     }
/// # }
/// # #[derive(Dependencies)]
/// # struct TwoStrings {
/// #     first: String,
/// #     second: String,
/// # }
/// // Create some input nodes.
/// let input_1 = InputNode::new("Hello,".to_string());
/// let input_2 = InputNode::new("???".to_string());
///
/// // Create a derived node.
/// let node = DerivedNode::new(
///     TwoStrings::init(Rc::clone(&input_1), Rc::clone(&input_2)),
///     Concat,
///     String::new()
/// );
///
/// let mut visitor = HashSetVisitor::new();
///
/// // Resolve the node.
/// assert_eq!(
///     node.resolve_root(&mut visitor).unwrap().value(),
///     "Hello, ???"
/// );
///
/// // Update the input nodes.
/// input_2.update("world!".to_string()).unwrap();
///
/// // The node will detect the input has changed and recompute its value.
/// assert_eq!(
///     node.resolve_root(&mut visitor).unwrap().value(),
///     "Hello, world!"
/// );
///
/// // Any nodes which `resolve` to the dependency types can be combined.
/// let input_3 = InputNode::new("See ya.".to_string());
///
/// let another_node = DerivedNode::new(
///     TwoStrings::init(Rc::clone(&node), Rc::clone(&input_3)),
///     Concat,
///     String::new()
/// );
///
/// assert_eq!(
///     another_node.resolve_root(&mut visitor).unwrap().value(),
///     "Hello, world! See ya."
/// );
/// ```
pub struct DerivedNode<D, F, T> {
    /// The dependencies of this node. This can be a single node, or a
    /// struct containing multiple nodes.
    dependencies: D,
    /// The wrapped value of this node.
    value: RefCell<NodeState<T>>,
    /// The unique runtime Id of this node.
    id: usize,
    /// A type representing the function used to update the value of this
    /// node. This is only called if the dependencies appear to have changed.
    phantom: PhantomData<F>,
}

impl<D, F, T> DerivedNode<D, F, T>
where
    for<'a> D: Resolve + IsDirtyInferenceWorkaround<'a> + 'a,
    for<'a> F: UpdateDerived<
            Input<'a> = <D as IsDirtyInferenceWorkaround<'a>>::OutputWorkaround,
            Target<'a> = RefMut<'a, NodeState<T>>,
        > + 'a,
    T: HashValue + Clean + Named,
{
    /// Construct this node.
    pub fn new(dependencies: D, update: F, value: T) -> Rc<Self> {
        Self::new_with_id(dependencies, update, value, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(dependencies: D, _: F, value: T, id: usize) -> Rc<Self> {
        // TODO: we should store `update` and make the `update_derived` call
        //  take a &self so that values can be provided for update fns.
        Rc::new(Self {
            dependencies,
            phantom: PhantomData::<F>,
            value: RefCell::new(NodeState::new(value)),
            id,
        })
    }
}

impl<D, F, T> Resolve for DerivedNode<D, F, T>
where
    for<'a> D: Resolve + IsDirtyInferenceWorkaround<'a> + 'a,
    for<'a> F: UpdateDerived<
            Input<'a> = <D as IsDirtyInferenceWorkaround<'a>>::OutputWorkaround,
            Target<'a> = RefMut<'a, NodeState<T>>,
        > + 'a,
    T: HashValue + Clean + Named,
{
    type Output<'a> = Ref<'a, NodeState<T>> where Self: 'a ;

    fn resolve(&self, visitor: &mut impl Visitor) -> Result<Self::Output<'_>, ResolveError> {
        visitor.touch(self, Some(F::name()));
        if visitor.visit(self) {
            let mut node_state = self.value.try_borrow_mut()?;
            node_state.clean();
            let input = self.dependencies.resolve_workaround(visitor)?;
            if input.is_dirty() {
                F::update_derived(input, node_state)?;
                // TODO: I'm running in to lifetime issues passing a
                //  &mut node_state above, which would prevent the need to
                //  reborrow here. For some reason, a mutable reference
                //  causes the borrow checker to want node_state to live
                //  beyond the current block (presumably to match input),
                //  whereas a shared reference does not.
                let mut node_state = self.value.try_borrow_mut()?;
                node_state.update_node_hash(&mut visitor.hasher());
                visitor.notify_recalculated(self);
            }
        }
        visitor.leave(self);
        Ok(self.value.try_borrow()?)
    }
}

impl<D, F, T: Named> Named for DerivedNode<D, F, T> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<D, F, T: Named> Identifiable for DerivedNode<D, F, T> {
    fn id(&self) -> usize {
        self.id
    }
}

mod hrtb_workaround {
    use super::*;

    /// If we just provide the constraint
    /// ``` text
    /// impl<D, F, T> Resolve for DerivedNode<D, F, T>
    /// where
    ///     D: Resolve,
    ///     for<'a> <D as Resolve>::Output<'a>: IsDirty
    ///     // ...
    /// {}
    /// ```
    /// It appears HRTB type-inference cannot work out what D is. This seems to
    /// be to do with the fact we can't constrain `D: 'a` in the same expression
    /// we constrain `<D as Resolve>::Output<'a>: IsDirty`.
    ///
    /// See [this issue](https://github.com/rust-lang/rust/issues/90950).
    ///
    /// As a workaround, create a super trait which binds the same lifetime to
    /// its output as the type, and ensures the the output is [IsDirty].
    pub trait IsDirtyInferenceWorkaround<'a>: Resolve + 'a {
        type OutputWorkaround: IsDirty;

        fn resolve_workaround(
            &'a self,
            visitor: &'a mut impl Visitor,
        ) -> Result<Self::OutputWorkaround, ResolveError>;
    }

    impl<'a, T> IsDirtyInferenceWorkaround<'a> for T
    where
        T: Resolve + 'a,
        <T as Resolve>::Output<'a>: IsDirty,
    {
        type OutputWorkaround = <T as Resolve>::Output<'a>;

        fn resolve_workaround(
            &'a self,
            visitor: &'a mut impl Visitor,
        ) -> Result<Self::OutputWorkaround, ResolveError> {
            self.resolve(visitor)
        }
    }
}
