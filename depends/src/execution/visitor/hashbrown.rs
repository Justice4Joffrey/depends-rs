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
    use serial_test::serial;

    use super::*;
    use crate::{
        execution::{identifiable::reset_node_id, internal_test_utils::TestData},
        InputNode,
    };

    #[test]
    #[serial]
    fn test_hashbrown_visitor() {
        reset_node_id();
        let node_1 = InputNode::new(TestData::new(5));
        let _ = InputNode::new(TestData::new(0));
        let node_2 = InputNode::new(TestData::new(6));
        let mut visitor = HashBrownVisitor::new();
        visitor.visit(&node_1);
        visitor.visit(&node_2);
        assert_eq!(visitor.len(), 2);
        assert!(visitor.contains(&0));
        assert!(visitor.contains(&2));
        visitor.clear();
        assert_eq!(visitor.len(), 0);
        let hasher = visitor.hasher();
        hasher.hash_one(654);
        // TODO: must be unique.
    }
}
