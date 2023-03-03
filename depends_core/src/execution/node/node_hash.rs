/// A Hash of the current node state used to signal whether a dependent node
/// needs to update its internal state.
///
/// Every edge in a `depends` graph tracks the previously observed `NodeHash` of
/// the child node and signals to the dependee whether this has _changed_ since
/// the last run using the [IsDirty](crate::execution::IsDirty) trait.
/// This is used to cache internal computations.
///
/// # NotHashed
///
/// If `NotHashed` is used, dependent nodes will *never* consider previously
/// observed states equal and will therefore update their internal state
/// _every time_ they're resolved. For this reason, it's recommended that
/// `NotHashed` be used sparingly (in cases where computing a hash of a node's
/// state is impossible or costlier than recomputing the dependee's state).
///
/// Whichever implementation is chosen, it's important to structure graphs so
/// that the number of nodes and edges _to_ nodes implementing `NotHashed` is
/// kept to a minimum, especially where performance is a concern.
///
/// ```
/// # use depends_core::execution::NodeHash;
/// // not equal
/// assert_ne!(NodeHash::NotHashed, NodeHash::NotHashed);
/// assert_ne!(NodeHash::NotHashed, NodeHash::Hashed(0));
/// assert_ne!(NodeHash::Hashed(0), NodeHash::Hashed(1));
/// // equal
/// assert_eq!(NodeHash::Hashed(1), NodeHash::Hashed(1));
/// ```
#[derive(Copy, Clone, Debug, Default, Eq)]
pub enum NodeHash {
    /// _Never_ equal to another value.
    #[default]
    NotHashed,
    /// Equal to another value if the internal number is equal.
    Hashed(u64),
}

impl PartialEq for NodeHash {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Hashed(a), Self::Hashed(b)) => a == b,
            _ => false,
        }
    }
}
