use std::collections::HashSet;

use super::Identifiable;

pub trait Visitor {
    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable;

    fn mark_edge<N>(&mut self, _node: &N)
    where
        N: Identifiable,
    {
    }

    fn clear(&mut self);
}

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
