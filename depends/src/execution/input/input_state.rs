/// Used to ensure that pending data is resolved at most once between calls to
/// `update`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum InputState {
    /// This node is being updated.
    #[default]
    Updating,
    /// Indicates this node must be cleaned when next resolved or updated.
    Resolving,
    /// This node has been resolved, no further cleaning is necessary.
    Resolved,
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use super::*;

    #[test]
    fn test_input_state() {
        let state = InputState::default();
        assert_eq!(state, InputState::Updating);
        assert_eq!("Updating", format!("{state:?}"));
        let hasher = &mut std::collections::hash_map::DefaultHasher::new();
        state.hash(hasher);
        assert_eq!(hasher.finish(), 13646096770106105413);
        assert_eq!(InputState::Updating, InputState::Updating.clone());
    }
}
