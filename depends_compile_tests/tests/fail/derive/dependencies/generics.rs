use depends::derives::Dependencies;

#[derive(Dependencies)]
struct Bar<A, B> {
    node1: A,
    node2: B,
}

fn main() {}
