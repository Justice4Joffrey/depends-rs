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

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use serial_test::serial;

    use super::*;
    use crate::execution::{Clean, HashValue, InputNode, NodeHash, UpdateInput};

    #[test]
    #[serial]
    fn test_identifiable_rc() {
        struct Foo(i32);
        impl Named for Foo {
            fn name() -> &'static str {
                "Foo"
            }
        }
        impl Clean for Foo {
            fn clean(&mut self) {}
        }
        impl HashValue for Foo {
            fn hash_value(&self, _: &mut impl Hasher) -> NodeHash {
                NodeHash::Hashed(42)
            }
        }
        impl UpdateInput for Foo {
            type Update = ();

            fn update_mut(&mut self, _: Self::Update) {}
        }
        // seems coverage doesn't like these definitions
        assert_eq!(Foo::name(), "Foo");
        let mut foo = Foo(123);
        let hasher = &mut std::collections::hash_map::DefaultHasher::new();
        assert_eq!(foo.hash_value(hasher), NodeHash::Hashed(42));
        foo.clean();
        let node = InputNode::new(foo);
        let rc = Rc::new(node);
        assert_eq!(rc.id(), 0);
        let node = InputNode::new(Foo(123));
        let rc = Rc::new(node);
        assert_eq!(rc.id(), 1);
        NODE_ID.store(0, Ordering::Relaxed);
    }

    #[test]
    #[serial]
    fn test_next_node_id() {
        assert_eq!(next_node_id(), 0);
        assert_eq!(next_node_id(), 1);
        assert_eq!(next_node_id(), 2);
        NODE_ID.store(0, Ordering::Relaxed);
    }
}
