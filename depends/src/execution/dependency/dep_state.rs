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

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use super::*;

    #[test]
    fn test_dependency_state_derives() {
        let state = DependencyState::Dirty;
        assert_eq!(state.clone(), state);
        assert_eq!("Dirty", format!("{state:?}"));
        let hasher = &mut std::collections::hash_map::DefaultHasher::new();
        state.hash(hasher);
        assert_eq!(hasher.finish(), 13646096770106105413);
    }
}
