use depends::derives::Dependencies;

#[derive(Dependencies)]
union Bar {
    node1: usize,
    node2: usize,
}

fn main() {}
