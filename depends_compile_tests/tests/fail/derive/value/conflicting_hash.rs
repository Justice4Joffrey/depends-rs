use depends::derives::Value;

#[derive(Value)]
#[depends(unhashable)]
struct Foo<T> {
    bar: Vec<T>,
    #[depends(hash)]
    number: usize,
}

fn main() {}
