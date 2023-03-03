mod dep_ref;
mod dep_state;

use std::cell::RefCell;

pub use dep_ref::DepRef;
pub use dep_state::DependencyState;

use super::{HashValue, Identifiable, Named, NodeHash, Resolve};

/// Wraps a dependency and tracks the hashed value each time it's resolved. This
/// allows the resolver to know if a dependency is 'dirty' from the perspective
/// of the Dependee.
#[derive(Debug)]
pub struct Dependency<T> {
    /// The state observed of the inner dependency when it was last resolved.
    last_state: RefCell<Option<NodeHash>>,
    /// The wrapped node.
    dependency: T,
}

impl<T> Dependency<T>
where
    T: Resolve,
{
    pub fn new(dependency: T) -> Self {
        Self {
            last_state: RefCell::new(None),
            dependency,
        }
    }
}

impl<T> Resolve for Dependency<T>
where
    T: Resolve,
    for<'a> <T as Resolve>::Output<'a>: HashValue,
{
    type Output<'a>
        = DepRef<'a, T::Output<'a>>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl super::Visitor) -> Self::Output<'_> {
        let mut last_state = self.last_state.borrow_mut();
        let data = self.dependency.resolve(visitor);
        let current_state = data.hash_value(&mut visitor.hasher());
        if last_state.map(|s| s == current_state).unwrap_or(false) {
            DepRef::new(DependencyState::Clean, data)
        } else {
            (*last_state) = Some(current_state);
            DepRef::new(DependencyState::Dirty, data)
        }
    }
}

impl<T: Named> Named for Dependency<T> {
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Identifiable> Identifiable for Dependency<T> {
    fn id(&self) -> usize {
        self.dependency.id()
    }
}
