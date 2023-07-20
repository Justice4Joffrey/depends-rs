use depends::derives::Graph;
#[allow(unused_imports)]
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {
        posts [label="Posts"];
        likes [label="Likes"];
        friends [label="Friends"];
        posts -> likes [label="TrackPostLikes"];
        likes -> friends [label="TrackLikeFriends"];
        friends -> posts [label="TrackFriendPosts"];
    }
)]
struct Graph {}

fn main() {}
