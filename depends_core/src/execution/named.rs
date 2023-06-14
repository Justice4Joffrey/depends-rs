use std::sync::Arc;

/// A string name for each graph node, useful for rendering graph
/// visualisations.
pub trait Named {
    fn name() -> &'static str;
}

impl<T: Named> Named for Arc<T> {
    fn name() -> &'static str {
        T::name()
    }
}
