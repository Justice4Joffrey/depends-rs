use std::rc::Rc;

use crate::execution::Visitor;

pub trait Resolve {
    type Output<'a>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_>;

    fn clean(&self, visitor: &mut impl Visitor);
}

impl<T: Resolve> Resolve for Rc<T> {
    type Output<'a> = T::Output<'a> where Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> Self::Output<'_> {
        T::resolve(self, visitor)
    }

    fn clean(&self, visitor: &mut impl Visitor) {
        T::clean(self, visitor)
    }
}
