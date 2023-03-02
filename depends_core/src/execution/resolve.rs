use std::rc::Rc;

use crate::execution::Visitor;

/// A [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) resolver, used to
/// recursively pass a [Visitor] through a graph, updating dependencies.
pub trait Resolve {
    type Output<'a>
    where
        Self: 'a;

    /// You're probably looking for [resolve_root](Self::resolve_root). This is
    /// recursively called on each node when a graph is being resolved.
    ///
    /// Pass a [Visitor] through this node, resolve the latest version of all
    /// dependencies and return this node's output.
    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_>;

    /// Pass a [Visitor] through this node, resolve the latest version of all
    /// dependencies, reset the visitor and return this node's output.
    fn resolve_root(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        let res = self.resolve(visitor);
        visitor.clear();
        res
    }
}

impl<T: Resolve> Resolve for Rc<T> {
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        T::resolve(self, visitor)
    }
}
