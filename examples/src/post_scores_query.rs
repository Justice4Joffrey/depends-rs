use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use chrono::{DateTime, Utc};
use depends::{
    core::{error::EarlyExit, TargetMut, UpdateDerived},
    derives::{Dependencies, Operation, Value},
};

use crate::{Comments, CommentsToPosts, Likes, Posts};

/// Cumulative score and timestamp for a post.
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct PostScore {
    score: u32,
    ts: DateTime<Utc>,
    id: i64,
}

#[derive(Debug, Default, Value)]
pub struct PostScoresQuery {
    /// Map of post id to scores.
    post_scores: HashMap<i64, PostScore>,
    /// Map of comments to posts.
    /// The top 3 posts.
    top_posts: BinaryHeap<Reverse<PostScore>>,
    /// Use whether the top posts has changed as a hash. This is unnecessary
    /// as no node depends on this one, but it's a good example of how to
    /// cache multiple nodes behind a single node, which will dramatically
    /// increase the performance of further dependees.
    #[depends(hash)]
    top_posts_generation: usize,
}

impl PostScoresQuery {
    pub fn top_posts(&self) -> String {
        let sorted = self.top_posts.clone().into_sorted_vec();
        sorted
            .iter()
            .map(|p| p.0.id.to_string())
            .collect::<Vec<_>>()
            .join("|")
    }

    fn get_post_id_mut(&mut self, post_id: i64) -> Result<&mut PostScore, EarlyExit> {
        self.post_scores
            .get_mut(&post_id)
            .ok_or_else(|| EarlyExit::new(format!("No score found for post id {}", post_id)))
    }

    fn update_post_score(&mut self, post_id: i64, score: u32) -> Result<bool, EarlyExit> {
        let post_score = self.get_post_id_mut(post_id)?;
        post_score.score += score;
        let post_score = post_score.clone();
        Ok(self.update_top_posts(post_score))
    }

    fn update_top_posts(&mut self, post_score: PostScore) -> bool {
        if self.top_posts.len() < 3 {
            self.top_posts.push(Reverse(post_score));
            return true;
        } else {
            let smallest = self.top_posts.peek().unwrap();
            if post_score > smallest.0 {
                // remove any existing post with the same id
                self.retain_top_posts(post_score.id);
                if self.top_posts.len() < 3 {
                    self.top_posts.push(Reverse(post_score));
                } else {
                    // the current id was not present => pop the smallest
                    self.top_posts.pop();
                    self.top_posts.push(Reverse(post_score));
                }
                return true;
            }
        }
        false
    }

    /// Only stable since 1.70.0 and we want to keep MRSV to 1.65
    #[cfg(nightly)]
    fn retain_top_posts(&mut self, other_than: i64) {
        self.top_posts.retain(|p| p.0.id != other_than);
    }

    /// Only stable since 1.70.0 and we want to keep MRSV to 1.65. Don't
    /// care about the performance of this function.
    #[cfg(not(nightly))]
    fn retain_top_posts(&mut self, other_than: i64) {
        let mut heap = BinaryHeap::new();
        while let Some(post) = self.top_posts.pop() {
            if post.0.id != other_than {
                heap.push(post);
            }
        }
        std::mem::swap(&mut self.top_posts, &mut heap);
    }
}

#[derive(Dependencies)]
pub struct CommentsPostsLikes {
    comments: Comments,
    comments_to_posts: CommentsToPosts,
    posts: Posts,
    likes: Likes,
}

#[derive(Operation)]
pub struct UpdatePostScoresQuery;

impl UpdateDerived for UpdatePostScoresQuery {
    type Input<'a> = CommentsPostsLikesRef<'a>
    where
        Self: 'a;
    type Target<'a> = TargetMut<'a, PostScoresQuery>
    where
        Self: 'a;

    fn update_derived(
        CommentsPostsLikesRef {
            comments,
            comments_to_posts,
            posts,
            likes,
        }: Self::Input<'_>,
        mut target: Self::Target<'_>,
    ) -> Result<(), EarlyExit> {
        for post in posts.new_posts() {
            target.post_scores.insert(
                post.id,
                PostScore {
                    score: 0,
                    ts: post.ts,
                    id: post.id,
                },
            );
        }

        let mut delta = 0;
        for comment in comments.new_comments() {
            let post_id = comments_to_posts.get_post_id(comment.id)?;
            if target.update_post_score(post_id, 10)? {
                delta = 1;
            }
        }

        for like in likes.new_likes() {
            let post_id = comments_to_posts.get_post_id(like.comment_id)?;
            if target.update_post_score(post_id, 1)? {
                delta = 1;
            }
        }
        target.top_posts_generation += delta;
        Ok(())
    }
}
