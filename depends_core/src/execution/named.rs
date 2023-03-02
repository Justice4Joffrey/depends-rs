use std::rc::Rc;

/// A string name for each graph node, useful for rendering graph
/// visualisations.
pub trait Named {
    fn name() -> &'static str;
}

impl<T: Named> Named for Rc<T> {
    fn name() -> &'static str {
        T::name()
    }
}
