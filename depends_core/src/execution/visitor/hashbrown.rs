use std::hash::BuildHasher;

use hashbrown::HashSet;

use super::Visitor;
use crate::execution::Identifiable;

pub type HashBrownVisitor = HashSet<usize>;

impl Visitor for HashBrownVisitor {
    type Hasher = ahash::AHasher;

    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable,
    {
        self.insert(node.id())
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn hasher(&self) -> Self::Hasher {
        self.hasher().build_hasher()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution::Named;

    struct Foo(usize);

    impl Named for Foo {
        fn name() -> &'static str {
            "Foo"
        }
    }

    impl Identifiable for Foo {
        fn id(&self) -> usize {
            self.0
        }
    }

    #[test]
    fn test_hashbrown_visitor() {
        let mut visitor = HashBrownVisitor::new();
        visitor.visit(&Foo(0));
        visitor.visit(&Foo(345));
        assert_eq!(visitor.len(), 2);
        assert!(visitor.contains(&0));
        assert!(visitor.contains(&345));
        visitor.clear();
        assert_eq!(visitor.len(), 0);
        let hasher = visitor.hasher();
        hasher.hash_one(654);
    }
}
