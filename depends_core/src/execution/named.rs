use std::rc::Rc;

pub trait Named {
    fn name() -> &'static str;
}

impl<T: Named> Named for Rc<T> {
    fn name() -> &'static str {
        T::name()
    }
}
