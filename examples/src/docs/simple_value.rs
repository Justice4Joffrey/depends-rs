use depends::{derives::Value, UpdateInput};

#[derive(Value, Hash)]
struct SomeNumber {
    value: i32,
}

impl UpdateInput for SomeNumber {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        // Simply replace the value with the update.
        self.value = update;
    }
}
