use std::collections::HashSet;

use super::Identifiable;

/// A collection passed in to a graph, tracking the identifiers of each nodes to
/// avoid traversing
pub trait Visitor {
    /// Return true *iff* this node hasn't been visited yet.
    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable;

    /// Mark an edge from the previously visited node to the current node.
    /// Useful for creating graph visualisations.
    fn mark_edge<N>(&mut self, _node: &N)
    where
        N: Identifiable,
    {
    }

    /// Clear the internal collection, prompting this visitor to revisit all
    /// nodes on the next traversal.
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
