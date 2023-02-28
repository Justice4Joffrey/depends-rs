use std::{cell::RefCell, marker::PhantomData};

use super::{HashValue, Identifiable, IsDirty, Named, NodeHash, Resolve};

// TODO: this type should be the _only_ thing possible to pass to a graph as a
// dependency, or everything gets super annoying with weird trait-bound errors.
/// Wraps a dependency and tracks the hashed value each time it's resolved. This
/// allows the resolver to know if a dependency is 'dirty' from the perspective
/// of the Dependee.
#[derive(Debug)]
pub struct Dependency<T> {
    /// The state observed of the inner dependency when it was last resolved.
    last_state: RefCell<Option<NodeHash>>,
    dependency: T,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum State {
    Dirty,
    Clean,
}

// TODO: move this somewhere and figure out how to clean the API, as in probably
// implement deref on this.
#[derive(Debug)]
pub struct DepState<'a, T> {
    state: State,
    data: T,
    phantom: PhantomData<&'a T>,
}

impl<T> DepState<'_, T> {
    pub fn state(&self) -> State {
        self.state
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> IsDirty for DepState<'_, T> {
    fn is_dirty(&self) -> bool {
        self.state == State::Dirty
    }
}

impl<T> Dependency<T> {
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
        = DepState<'a, T::Output<'a>>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl super::Visitor) -> Self::Output<'_> {
        let mut last_state = self.last_state.borrow_mut();
        let data = self.dependency.resolve(visitor);
        let current_state = data.hash_value();
        if last_state.map(|s| s == current_state).unwrap_or(false) {
            DepState {
                state: State::Clean,
                data,
                phantom: PhantomData,
            }
        } else {
            (*last_state) = Some(current_state);
            DepState {
                state: State::Dirty,
                data,
                phantom: PhantomData,
            }
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
