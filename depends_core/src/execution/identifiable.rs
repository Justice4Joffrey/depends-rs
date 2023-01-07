use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::Named;

/// Global node Ids are kept in order to track execution across graphs.
static NODE_ID: AtomicUsize = AtomicUsize::new(0);

pub trait Identifiable: Named {
    fn id(&self) -> usize;
}

pub fn next_node_id() -> usize {
    NODE_ID.fetch_add(1, Ordering::Relaxed)
}

impl<T> Identifiable for Rc<T>
where
    T: Identifiable,
{
    fn id(&self) -> usize {
        T::id(self)
    }
}
