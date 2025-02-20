#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];

        comments_posts [label="CommentsPosts"];

        comments -> comments_posts [label="TrackCommentPostIds"];
        posts -> comments_posts [label="TrackCommentPostIds"];
    }
)]
struct Graph1 {}

fn main() {}
