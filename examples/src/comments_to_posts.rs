use depends::{
    derives::{Operation, Value},
    error::EarlyExit,
    DepRef, UpdateDerived,
};
use hashbrown::HashMap;

use crate::Comments;

/// A map of comment ids to the oldest ancestor post id.
/// The data model is a tree, where the root is a post and the rest are
/// comments.
///
/// It's not 'optimal' for query speed to separate this logic from the end
/// query as this results in an extra iteration over each dirty comment, but
/// it's a good example of how to break up logic into smaller, composable
/// pieces.
#[derive(Debug, Default, Value)]
pub struct CommentsToPosts {
    /// A map of comment ids to the oldest ancestor post id.
    comments_to_posts: HashMap<i64, i64>,
    /// The length of the comments_to_posts map. This is fine to use as a
    /// hash. Comments are append-only.
    #[depends(hash)]
    len: usize,
}

impl CommentsToPosts {
    pub fn new() -> Self {
        Self {
            comments_to_posts: HashMap::with_capacity(512),
            len: 0,
        }
    }

    pub fn get_post_id(&self, comment_id: i64) -> Result<i64, EarlyExit> {
        self.comments_to_posts
            .get(&comment_id)
            .cloned()
            .ok_or_else(|| EarlyExit::new(format!("No post found for comment id {}", comment_id)))
    }
}

#[derive(Operation)]
pub struct TrackCommentPostIds;

impl UpdateDerived<DepRef<'_, Comments>, TrackCommentPostIds> for CommentsToPosts {
    fn update(&mut self, value: DepRef<'_, Comments>) -> Result<(), EarlyExit> {
        for comment in value.new_comments() {
            let post_id = if let Some(post_id) = self.comments_to_posts.get(&comment.parent_id) {
                *post_id
            } else {
                comment.parent_id
            };
            self.comments_to_posts.insert(comment.id, post_id);
        }
        self.len = self.comments_to_posts.len();
        Ok(())
    }
}
