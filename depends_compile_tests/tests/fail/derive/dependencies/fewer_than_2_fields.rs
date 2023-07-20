use depends::derives::Dependencies;

#[derive(Dependencies)]
struct Foo {
    bar: usize,
}

#[derive(Dependencies)]
struct Bar {}

fn main() {}
