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

/// A node which has a value derived from other nodes. This node will keep
/// track of the state of any dependencies it has, and recompute its value if
/// any of its dependencies appear to have changed.
pub struct DerivedNode<D, F, T> {
    /// The dependencies of this node. This can be a single node, or a
    /// struct containing multiple nodes.
    dependencies: D,
    /// The value of this node.
    data: RefCell<NodeState<T>>,
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
    pub fn new(dependencies: D, update: F, data: T) -> Rc<Self> {
        Self::new_with_id(dependencies, update, data, next_node_id())
    }

    /// Create this node with a specified Id. Useful for tests.
    pub fn new_with_id(dependencies: D, _: F, data: T, id: usize) -> Rc<Self> {
        // TODO: we should store `update` and make the `update_derived` call
        //  take a &self so that values can be provided for update fns.
        Rc::new(Self {
            dependencies,
            phantom: PhantomData::<F>,
            data: RefCell::new(NodeState::new(data)),
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
            let input = self.dependencies.resolve_workaround(visitor)?;
            if input.is_dirty() {
                let mut node_state = self.data.try_borrow_mut()?;
                node_state.clean();
                F::update_derived(input, node_state)?;
                // TODO: I'm running in to lifetime issues passing a
                //  &mut node_state above, which would prevent the need to
                //  reborrow here. For some reason, a mutable reference
                //  causes the borrow checker to want node_state to live
                //  beyond the current block (presumably to match input),
                //  whereas a shared reference does not.
                let mut node_state = self.data.try_borrow_mut()?;
                node_state.update_node_hash(&mut visitor.hasher());
                visitor.notify_recalculated(self);
            }
        }
        visitor.leave(self);
        Ok(self.data.try_borrow()?)
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
