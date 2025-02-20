#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
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
