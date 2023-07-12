use crate::execution::error::ResolveResult;

/// For any dependee's dependencies (or single [Dependency](super::Dependency)),
/// this is used to check whether previously observed values have changed,
/// indicating its stored value needs to be recomputed.
pub trait IsDirty {
    fn is_dirty(&self) -> bool;
}

// TODO: i don't think this is right, but it might compile
impl<T: IsDirty> IsDirty for ResolveResult<T> {
    fn is_dirty(&self) -> bool {
        match self {
            Ok(t) => t.is_dirty(),
            Err(_) => true,
        }
    }
}
