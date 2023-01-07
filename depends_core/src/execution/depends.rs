/// Enforces that a dependee type derived by the proc macro can only use a
/// single type of dependency.
pub trait Depends {
    type Input<'a>
    where
        Self: 'a;
}
