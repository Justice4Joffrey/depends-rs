use super::NodeHash;
use crate::execution::{Clean, Depends, HashValue, Named, UpdateDependee, UpdateLeaf};

// TODO: this is only applicable to leaves, therefore should be exported
/// Used to ensure that pending data is resolved at most once between calls to
/// `update`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum ResolveState {
    /// This node is being updated.
    #[default]
    Updating,
    /// Indicates this node must be cleaned when next resolved or updated.
    Resolving,
    /// This node has been resolved, no further cleaning is necessary.
    Resolved,
}

pub struct NodeState<T> {
    state: ResolveState,
    /// A value representing the unique state of the data (i.e. the Hash).
    node_hash: NodeHash,
    data: T,
}

impl<T: HashValue> NodeState<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: ResolveState::default(),
            node_hash: NodeHash::default(),
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn node_hash(&self) -> NodeHash {
        self.node_hash
    }

    pub fn state(&self) -> ResolveState {
        self.state
    }

    pub fn node_hash_mut(&mut self) -> &mut NodeHash {
        &mut self.node_hash
    }

    pub fn state_mut(&mut self) -> &mut ResolveState {
        &mut self.state
    }

    pub fn update_node_hash(&mut self) {
        self.node_hash = self.data.hash_value()
    }
}

impl<T> UpdateDependee for NodeState<T>
where
    T: UpdateDependee,
{
    fn update_mut(&mut self, input: Self::Input<'_>) {
        self.data.update_mut(input)
    }
}

impl<T> Depends for NodeState<T>
where
    T: Depends,
{
    type Input<'a> = T::Input<'a> where Self: 'a;
}

impl<T> UpdateLeaf for NodeState<T>
where
    T: UpdateLeaf,
{
    type Input = T::Input;

    fn update_mut(&mut self, input: Self::Input) {
        self.data.update_mut(input)
    }
}

impl<T> HashValue for NodeState<T>
where
    T: HashValue,
{
    fn hash_value(&self) -> NodeHash {
        self.node_hash
    }
}

impl<T> Named for NodeState<T>
where
    T: Named,
{
    fn name() -> &'static str {
        T::name()
    }
}

impl<T: Clean> Clean for NodeState<T> {
    fn clean(&mut self) {
        self.data.clean()
    }
}
