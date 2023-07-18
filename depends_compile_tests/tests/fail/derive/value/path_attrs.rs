use depends::derives::Value;

#[derive(Value)]
#[depends(is::unhashable)]
struct Foo {
    bar: Vec<usize>,
}

fn main() {}
