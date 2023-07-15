use std::collections::{HashMap, HashSet};

use depends::{
    core::{Clean, UpdateInput},
    derives::Value,
};

use crate::models::Friend;

#[derive(Debug, Default, Value)]
#[depends(custom_clean)]
pub struct Friends {
    /// Map of all friends. Each friendship is represented twice, once for
    /// each user.
    friends: HashMap<i64, HashSet<i64>>,
    /// Map of all friends that have been added since the last generation.
    new_friends: Vec<Friend>,
    /// The current generation of the friends. If the generation changes, there
    /// are new friends.
    #[depends(hash)]
    generation: usize,
}

impl Friends {
    /// An iterator over all friends that have been added since the last
    /// generation.
    pub fn new_friends(&self) -> impl Iterator<Item = &Friend> + '_ {
        self.new_friends.iter()
    }

    fn insert_friendship(&mut self, user_1_id: i64, user_2_id: i64) {
        self.friends.entry(user_1_id).or_default().insert(user_2_id);
    }
}

impl Clean for Friends {
    fn clean(&mut self) {
        self.new_friends.clear();
    }
}

impl UpdateInput for Friends {
    type Update = Friend;

    fn update_mut(&mut self, update: Self::Update) {
        self.insert_friendship(update.user_1_id, update.user_2_id);
        self.insert_friendship(update.user_2_id, update.user_1_id);
        self.new_friends.push(update);
        self.generation += 1;
    }
}
