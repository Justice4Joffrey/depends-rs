#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
use examples::*;

// Multiple labels for an edge.
#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];

        comments_posts [label="CommentsPosts"];

        comments -> comments_posts [label="TrackCommentPostIds", class="Dependencies2"];
        posts -> comments_posts [label="AnotherLabel", class="Dependencies2"];
    }
)]
struct Graph1 {}

// Multiple classes for an edge.
#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];

        comments_posts [label="CommentsPosts"];

        comments -> comments_posts [label="TrackCommentPostIds", class="Dependencies3"];
        posts -> comments_posts [label="TrackCommentPostIds", class="Dependencies2"];
    }
)]
struct Graph2 {}

fn main() {}
