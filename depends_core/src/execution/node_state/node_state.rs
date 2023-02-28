use crate::execution::{
    clean::Clean, Depends, HashedValue, IsDirty, Named, State, UpdateDependeeMut, UpdateLeafMut,
};

pub struct NodeState<T> {
    state: State,
    clean_hash: HashedValue,
    data: T,
}

impl<T> NodeState<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: State::Dirty,
            data,
            clean_hash: HashedValue::NotHashed,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl<T> Clean for NodeState<T>
where
    T: Clean,
{
    fn clean(&mut self) {
        self.state = State::Clean;
        self.data.clean()
    }
}

impl<T> UpdateDependeeMut for NodeState<T>
where
    T: UpdateDependeeMut,
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

impl<T> UpdateLeafMut for NodeState<T>
where
    T: UpdateLeafMut,
{
    type Input = T::Input;

    fn update_mut(&mut self, input: Self::Input) {
        self.state = State::Dirty;
        self.data.update_mut(input)
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

impl<T> IsDirty for NodeState<T> {
    fn is_dirty(&self) -> bool {
        self.state == State::Dirty
    }
}
