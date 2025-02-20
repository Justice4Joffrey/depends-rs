#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
use examples::*;

// Two disconnected graphs.
#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];
        likes [label="Likes"];
        friends [label="Friends"];

        comments_posts [label="CommentsPosts"];
        likes_friends [label="LikesFriends"];

        comments -> comments_posts [label="TrackCommentPostIds"];
        posts -> comments_posts [label="TrackPostCommentIds"];
        likes -> likes_friends [label="TrackLikeFriendIds"];
        friends -> likes_friends [label="TrackFriendLikeIds"];
    }
)]
struct Graph1 {}

// Connected with multiple roots.
#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];
        likes [label="Likes"];

        comments_posts [label="CommentsPosts"];
        comments_likes [label="CommentsLikes"];

        comments -> comments_posts [label="TrackCommentPostIds"];
        posts -> comments_posts [label="TrackPostCommentIds"];
        comments -> comments_likes [label="TrackPostLikeIds"];
        likes -> comments_likes [label="TrackPostLikeIds"];
    }
)]
struct Graph2 {}

fn main() {}
