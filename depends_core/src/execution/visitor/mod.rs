#[cfg(feature = "hashbrown")]
pub mod hashbrown;

use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{BuildHasher, Hasher},
};

use super::Identifiable;

/// A collection passed in to a graph, tracking the identifiers of each nodes to
/// avoid traversing
pub trait Visitor {
    type Hasher: Hasher;

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

    fn hasher(&self) -> Self::Hasher;
}

impl Visitor for HashSet<usize> {
    type Hasher = DefaultHasher;

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
        HashSet::<usize>::hasher(self).build_hasher()
    }
}
