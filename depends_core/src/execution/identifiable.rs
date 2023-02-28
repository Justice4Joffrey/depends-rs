use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::Named;

/// Global node Ids are kept in order to track execution across graphs.
static NODE_ID: AtomicUsize = AtomicUsize::new(0);

/// Public interface to the global [NODE_ID].
#[doc(hidden)]
pub fn next_node_id() -> usize {
    NODE_ID.fetch_add(1, Ordering::Relaxed)
}

/// A unique integer value assigned to each node created in a particular
/// runtime, allowing a [Visitor](super::Visitor) to track visited nodes when
/// resolving graphs.
pub trait Identifiable: Named {
    fn id(&self) -> usize;
}

impl<T> Identifiable for Rc<T>
where
    T: Identifiable,
{
    fn id(&self) -> usize {
        T::id(self)
    }
}
