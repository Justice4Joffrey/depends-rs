/// For the majority of cases, this trait can be ignored, and a default
/// implementation (no-op) can be used.
///
/// # When to use Clean
///
/// Nodes which provide dependees the ability to see _recently changed_ values
/// (since the last call to [Resolve](super::Resolve)), such as which keys have
/// been recently mutated in an internal hashmap, must reset the state used to
/// track this using this trait.
///
/// ## Example
///
/// An example is given [here](super::UpdateLeaf).
pub trait Clean {
    fn clean(&mut self);
}
