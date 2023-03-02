use hashbrown::HashSet;

use super::Visitor;
use crate::execution::Identifiable;

impl Visitor for HashSet<usize> {
    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable,
    {
        self.insert(node.id())
    }

    fn clear(&mut self) {
        self.clear()
    }
}
