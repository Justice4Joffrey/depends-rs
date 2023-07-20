use depends::derives::Value;

#[derive(Value)]
#[depends(unhashable = true)]
struct Foo {
    bar: Vec<usize>,
}

fn main() {}
