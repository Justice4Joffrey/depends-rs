/// Used to ensure that pending data is resolved at most once between calls to
/// `update`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum LeafState {
    /// This node is being updated.
    #[default]
    Updating,
    /// Indicates this node must be cleaned when next resolved or updated.
    Resolving,
    /// This node has been resolved, no further cleaning is necessary.
    Resolved,
}
