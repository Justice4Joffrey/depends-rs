#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
use examples::*;

#[derive(Graph)]
struct Graph1 {}

#[derive(Graph)]
#[depends(
    digraph Dag {
        comment [label="Comments"];
        comment_to_post [label="CommentsToPosts"];
        comment -> comment_to_post [label="TrackCommentPostIds"];
    }
)]
#[depends(
    digraph Dag {
        comment [label="Comments"];
        comment_to_post [label="CommentsToPosts"];
        comment -> comment_to_post [label="TrackCommentPostIds"];
    }
)]
struct Graph2 {}

fn main() {}
