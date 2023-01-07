/// For any individual or collection of nodes, this is used to check whether
/// dependent nodes should recalculate their internal state.
pub trait IsDirty {
    fn is_dirty(&self) -> bool;
}
