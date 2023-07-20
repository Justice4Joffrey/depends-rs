use depends::derives::Graph;
#[allow(unused_imports)]
use examples::*;

// Multiple labels for an edge.
#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];

        comments_posts [label="CommentsPosts"];

        comments -> comments_posts [label="TrackCommentPostIds", class="CommentTracker"];
        posts -> comments_posts [label="AnotherLabel", class="CommentTracker"];
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

        comments -> comments_posts [label="TrackCommentPostIds", class="CommentTracker"];
        posts -> comments_posts [label="TrackCommentPostIds", class="AnotherClass"];
    }
)]
struct Graph2 {}

fn main() {}
