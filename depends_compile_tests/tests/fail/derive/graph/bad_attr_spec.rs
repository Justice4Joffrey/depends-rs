use depends::derives::Graph;
#[allow(unused_imports)]
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {
        comment [class="Comments"];
        comment_to_post [label="CommentsToPosts"];
        comment -> comment_to_post [label="TrackCommentPostIds"];
    }
)]
struct Graph1 {}

#[derive(Graph)]
    #[depends(
        digraph Dag {
        comment [label="Comments"];
        comment_to_post [label="CommentsToPosts"];
        comment -> comment_to_post [class="TrackCommentPostIds"];
    }
)]
struct Graph2 {}

fn main() {}
