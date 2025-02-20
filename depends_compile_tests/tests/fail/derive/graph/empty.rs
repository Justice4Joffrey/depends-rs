#![allow(unused_imports)]
use depends::*;
use depends::derives::*;
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {

    }
)]
struct Graph {}

fn main() {}
