use depends::derives::Value;

#[derive(Value)]
#[depends(unhashable)]
struct Foo<T> {
    bar: Vec<T>,
}

fn main() {}
