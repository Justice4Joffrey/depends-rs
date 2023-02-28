use std::rc::Rc;

use crate::execution::Visitor;

/// A [Depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) resolver, used to
/// recursively pass a [Visitor] through a graph, updating dependencies.
pub trait Resolve {
    type Output<'a>
    where
        Self: 'a;

    /// Pass a [Visitor] through this node, resolve the latest version of all
    /// dependencies and return this node's output.
    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_>;
}

impl<T: Resolve> Resolve for Rc<T> {
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        T::resolve(self, visitor)
    }
}
