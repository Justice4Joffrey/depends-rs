/// Whether this dependency has a hash value which is different to the one
/// previously observed (if any).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DependencyState {
    /// No local hash was present, or the latest hash was not equal to the
    /// stored value.
    Dirty,
    /// This dependency was previously resolved and the hash is equivalent.
    Clean,
}
