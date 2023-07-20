use depends::derives::Value;

#[derive(Value)]
struct Foo<T> {
    bar: Vec<T>,
    #[depends(hash)]
    number: usize,
    #[depends(hash)]
    other: usize,
}

fn main() {}
