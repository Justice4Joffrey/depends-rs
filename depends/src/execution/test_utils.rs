use std::hash::{Hash, Hasher};

use crate::{Clean, HashValue, Named, NodeHash, UpdateInput};

/// A test node which pushes old values to a `recent` vector and replaces
/// `inner` with the new value.
#[derive(Debug, PartialEq)]
pub struct TestData {
    pub inner: u32,
    pub recent: Vec<u32>,
}

impl TestData {
    pub fn new(inner: u32) -> Self {
        Self {
            inner,
            recent: Vec::new(),
        }
    }
}

impl Named for TestData {
    fn name() -> &'static str {
        "TestData"
    }
}

impl HashValue for TestData {
    fn hash_value(&self, hasher: &mut impl Hasher) -> NodeHash {
        self.inner.hash(hasher);
        NodeHash::Hashed(hasher.finish())
    }
}

impl Clean for TestData {
    fn clean(&mut self) {
        self.recent.clear();
    }
}

impl UpdateInput for TestData {
    type Update = u32;

    fn update_mut(&mut self, update: Self::Update) {
        self.recent.push(self.inner);
        self.inner = update;
    }
}

#[test]
fn test_test_data() {
    // Unfortunately coverage requires us to test our tests
    assert_eq!(TestData::name(), "TestData");
    let mut data = TestData::new(42);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    assert_eq!(
        data.hash_value(&mut hasher),
        NodeHash::Hashed(15387811073369036852)
    );
    assert_eq!("TestData { inner: 42, recent: [] }", format!("{:?}", data));
    data.update_mut(420);
    assert_eq!(
        TestData {
            inner: 420,
            recent: vec![42]
        },
        data
    );
    data.clean();
    assert_eq!(
        TestData {
            inner: 420,
            recent: vec![]
        },
        data
    );
}
