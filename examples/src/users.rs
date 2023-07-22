use depends::{derives::Value, Clean, UpdateInput};
use hashbrown::HashMap;

use crate::models::User;

#[derive(Debug, Default, Value)]
#[depends(custom_clean)]
pub struct Users {
    /// Map of all users.
    users: HashMap<i64, User>,
    /// Map of all users that have been added since the last generation.
    new_user_ids: Vec<i64>,
    /// The current generation of the users. If the generation changes, there
    /// are new users.
    #[depends(hash)]
    generation: usize,
}

impl Users {
    /// An iterator over all users that have been added since the last
    /// generation.
    pub fn new_users(&self) -> impl Iterator<Item = &User> + '_ {
        self.new_user_ids.iter().map(|id| &self.users[id])
    }
}

impl Clean for Users {
    fn clean(&mut self) {
        self.new_user_ids.clear();
    }
}

impl UpdateInput for Users {
    type Update = User;

    fn update_mut(&mut self, update: Self::Update) {
        let user_id = update.id;
        self.users.insert(user_id, update);
        self.new_user_ids.push(user_id);
        self.generation += 1;
    }
}
