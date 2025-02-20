use depends::{derives::Value, Clean, UpdateInput};
use hashbrown::{HashMap, HashSet};

use crate::models::Like;

#[derive(Debug, Default, Value)]
#[depends(custom_clean)]
pub struct Likes {
    /// Map of comment Ids to all user Ids who've liked.
    user_likes_by_comment: HashMap<i64, HashSet<i64>>,
    /// Map of all likes that have been added since the last generation.
    new_likes: Vec<Like>,
    /// The current generation of the likes. If the generation changes, there
    /// are new likes.
    #[depends(hash)]
    generation: usize,
}

impl Likes {
    pub fn new() -> Self {
        Self {
            user_likes_by_comment: HashMap::with_capacity(512),
            new_likes: Vec::with_capacity(512),
            generation: 0,
        }
    }

    /// An iterator over all likes that have been added since the last
    /// generation.
    pub fn new_likes(&self) -> impl Iterator<Item = &Like> + '_ {
        self.new_likes.iter()
    }
}

impl Clean for Likes {
    fn clean(&mut self) {
        self.new_likes.clear();
    }
}

impl UpdateInput for Likes {
    type Update = Like;

    fn update_mut(&mut self, update: Self::Update) {
        self.user_likes_by_comment
            .entry(update.comment_id)
            .or_default()
            .insert(update.user_id);
        self.new_likes.push(update);
        self.generation += 1;
    }
}
