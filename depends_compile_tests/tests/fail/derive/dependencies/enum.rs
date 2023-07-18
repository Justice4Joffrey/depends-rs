use depends::derives::Dependencies;

#[derive(Dependencies)]
enum Bar {
    Node1(usize),
    Node2(usize),
}

fn main() {}
