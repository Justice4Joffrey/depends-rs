use depends::derives::Value;

#[derive(Value)]
#[depends(unhashable)]
enum Foo {
    Bar,
}

fn main() {}
