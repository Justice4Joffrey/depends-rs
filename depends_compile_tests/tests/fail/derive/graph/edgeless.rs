use depends::derives::Graph;
#[allow(unused_imports)]
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        likes [label="Likes"];
    }
)]
struct Graph {}

fn main() {}
