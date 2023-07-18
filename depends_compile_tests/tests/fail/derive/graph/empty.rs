use depends::derives::Graph;
#[allow(unused_imports)]
use examples::*;

#[derive(Graph)]
#[depends(
    digraph Dag {

    }
)]
struct Graph {}

fn main() {}
