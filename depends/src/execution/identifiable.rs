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

#[cfg(test)]
pub fn reset_node_id() {
    NODE_ID.store(0, Ordering::Relaxed);
}

/// Put this behind unsafe so that tests outside of this crate can reset the
/// node id.
///
/// This is unsafe to call because it will affect the node count to other
/// co-existing graphs. Make sure you're running this test in `serial`.
///
/// # Safety
///
/// Only for use in testing.
pub unsafe fn ext_reset_node_id() {
    NODE_ID.store(0, Ordering::Relaxed);
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

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::execution::{internal_test_utils::TestData, InputNode};

    #[test]
    #[serial]
    fn test_identifiable_rc() {
        reset_node_id();
        for i in 0..32 {
            let node = InputNode::new(TestData::new(i));
            let rc = Rc::new(node);
            assert_eq!(rc.id(), i as usize);
        }
    }

    #[test]
    #[serial]
    fn test_next_node_id() {
        reset_node_id();
        for i in 0..32_usize {
            assert_eq!(next_node_id(), i);
        }
    }
}
