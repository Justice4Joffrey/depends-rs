use depends::derives::Value;

#[derive(Value)]
#[depends(unhashable)]
union Foo {
    bar: usize,
}

fn main() {}
