use std::hash::BuildHasher;

use hashbrown::HashSet;

use super::Visitor;
use crate::execution::Identifiable;

impl Visitor for HashSet<usize> {
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
