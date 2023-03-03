use super::{Clean, HashValue};
use crate::execution::Named;

/// Describe a `Leaf` node's input, and how that mutates the internal state.
/// Correct implentation of this trait requires that any temporary state tracked
/// is cleared up when implementing [Clean](super::Clean).
///
/// # Example
///
/// It's not uncommon for a node which wraps a (potentially large) collection to
/// want to provide dependent nodes an accessor to know which of those have
/// changed since this node was last resolved. _Correct_ implementation of that
/// pattern requires implementation of [Clean](super::Clean), alongside
/// [UpdateDependee](super::UpdateDependee).
///
/// ```
/// # use depends_core::execution::{Clean, HashValue, Named, NodeHash, UpdateLeaf};
/// # use std::hash::Hasher;
/// #[derive(Default)]
/// pub struct MyNode {
///     // Appends every item to this collection.
///     all_things: Vec<i32>,
///     // An index where the 'new' items start.
///     new_things_index: usize,
/// }
/// #
/// # impl Named for MyNode {
/// #     fn name() -> &'static str { unimplemented!() }
/// # }
/// # impl HashValue for MyNode {
/// #     fn hash_value(&self, _: &mut impl Hasher) -> NodeHash { unimplemented!() }
/// # }
///
/// impl UpdateLeaf for MyNode {
///     type Input = i32;
///
///     fn update_mut(&mut self, input: Self::Input) {
///         self.all_things.push(input)
///     }
/// }
///
/// impl MyNode {
///     // Provide a convenience for dependees to iterate only the new things.
///     pub fn just_the_new_things_thanks(&self) -> impl Iterator<Item = i32> + '_ {
///         (self.new_things_index..).map_while(|idx| self.all_things.get(idx).copied())
///     }
/// }
///
/// impl Clean for MyNode {
///     fn clean(&mut self) {
///         // Ensure nothing is yielded next time.
///         self.new_things_index = self.all_things.len();
///     }
/// }
///
/// let mut node = MyNode::default();
///
/// // Add some initial data
/// node.update_mut(5);
///
/// assert_eq!(
///     node.just_the_new_things_thanks().collect::<Vec<_>>(),
///     vec![5]
/// );
///
/// // After cleaning, any temporary state is flushed.
/// node.clean();
/// assert_eq!(
///     node.just_the_new_things_thanks().collect::<Vec<_>>(),
///     vec![]
/// );
///
/// // Only the latest values are shown.
/// node.update_mut(7);
/// node.update_mut(6);
/// assert_eq!(
///     node.just_the_new_things_thanks().collect::<Vec<_>>(),
///     vec![7, 6]
/// );
///
/// // Just to hammer home the point.
/// node.clean();
/// assert_eq!(
///     node.just_the_new_things_thanks().collect::<Vec<_>>(),
///     vec![]
/// );
/// ```
///
/// In the example above, if [Clean](super::Clean) _wasn't_ implemented, the new
/// values would be displayed to dependent nodes _after_ this node had been
/// cleaned. This is an unsound implementation, which violates the caching logic
/// of `depends`.
pub trait UpdateLeaf: Named + HashValue + Clean {
    type Input;

    fn update_mut(&mut self, input: Self::Input);
}
