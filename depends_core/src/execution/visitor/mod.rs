#[cfg(feature = "hashbrown")]
pub mod hashbrown;

use std::collections::{BTreeSet, HashSet};

use super::Identifiable;

/// A collection passed in to a graph, tracking the identifiers of each nodes to
/// avoid traversing
pub trait Visitor {
    /// Return true *iff* this node hasn't been visited yet.
    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable;

    /// Clear the internal collection, prompting this visitor to revisit all
    /// nodes on the next traversal.
    fn clear(&mut self);

    /// Touch the node. Useful for building graph visualisations.
    fn touch<N>(&mut self, _node: &N)
    where
        N: Identifiable,
    {
    }

    /// Undo a [touch](Self::touch). Useful for building graph visualisations.
    fn leave<N>(&mut self, _node: &N)
    where
        N: Identifiable,
    {
    }
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

impl Visitor for BTreeSet<usize> {
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
