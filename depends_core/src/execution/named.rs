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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_rc() {
        assert_eq!(Rc::<String>::name(), "String");
    }
}
