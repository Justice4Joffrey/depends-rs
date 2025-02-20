//! # Social Network Example
//!
//! This example is to approximate the benchmark outline in [this article](https://link.springer.com/article/10.1007/s10270-021-00927-5#Sec91).

pub mod models;

mod comments;
mod comments_to_posts;
mod docs;
mod friends;
mod likes;
pub mod maths;
mod post_scores_query;
mod posts;
mod users;
pub use comments::Comments;
pub use comments_to_posts::*;
pub use friends::Friends;
pub use likes::Likes;
pub use post_scores_query::*;
pub use posts::Posts;
pub use users::Users;
