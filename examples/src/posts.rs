use depends::{
    core::{Clean, UpdateInput},
    derives::Value,
};
use hashbrown::HashMap;

use crate::models::Post;

#[derive(Debug, Default, Value)]
#[depends(custom_clean)]
pub struct Posts {
    /// Map of all posts.
    posts: HashMap<i64, Post>,
    /// Map of all posts that have been added since the last generation.
    new_post_ids: Vec<i64>,
    /// The current generation of the posts. If the generation changes, there
    /// are new posts.
    #[depends(hash)]
    generation: usize,
}

impl Posts {
    /// An iterator over all posts that have been added since the last
    /// generation.
    pub fn new_posts(&self) -> impl Iterator<Item = &Post> + '_ {
        self.new_post_ids.iter().map(|id| &self.posts[id])
    }
}

impl Clean for Posts {
    fn clean(&mut self) {
        self.new_post_ids.clear();
    }
}

impl UpdateInput for Posts {
    type Update = Post;

    fn update_mut(&mut self, update: Self::Update) {
        let post_id = update.id;
        self.posts.insert(post_id, update);
        self.new_post_ids.push(post_id);
        self.generation += 1;
    }
}
