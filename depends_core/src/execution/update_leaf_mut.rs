use crate::execution::{clean::Clean, Named};

/// Describe a `Leaf` node's input, and how that mutates the internal state.
pub trait UpdateLeafMut: Clean + Named {
    type Input;

    fn update_mut(&mut self, input: Self::Input);
}
