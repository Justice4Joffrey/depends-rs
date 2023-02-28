/// Whilst [State](super::State) can be used to determine whether a node has
/// been _altered_, `HashedValue` can be used to determine whether a node has
/// _changed_ i.e. if the value is different to what it was when previously
/// observed.
///
/// The default value of `NotHashed` will always compare as `false`.
///
/// ```
/// # use depends_core::execution::HashedValue;
/// assert_ne!(HashedValue::NotHashed, HashedValue::NotHashed);
/// assert_ne!(HashedValue::NotHashed, HashedValue::Hashed(0));
/// assert_ne!(HashedValue::Hashed(0), HashedValue::Hashed(1));
/// assert_eq!(HashedValue::Hashed(1), HashedValue::Hashed(1));
/// ```
#[derive(Copy, Clone, Debug, Default, Hash)]
pub enum HashedValue {
    #[default]
    NotHashed,
    Hashed(usize),
}

impl PartialEq for HashedValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Hashed(a), Self::Hashed(b)) => a == b,
            _ => false,
        }
    }
}
