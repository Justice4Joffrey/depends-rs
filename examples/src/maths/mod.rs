use depends::{
    derives::{Operation, Value},
    error::EarlyExit,
    DepRef, DepRef2, DepRef3, UpdateDerived, UpdateInput,
};

pub trait NumberLike {
    fn value(&self) -> i32;
}

/// A unit of data within a graph.
#[derive(Value, Default, Hash)]
pub struct NumberValueI32 {
    pub value: i32,
}

impl NumberValueI32 {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

// By implementing UpdateInput, we can change the value of this node from
// outside of the graph.
impl UpdateInput for NumberValueI32 {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        // Implementing this trait will provide a way for code outside of this graph to
        // change its internal state. This is just a simple replace for now.
        self.value = update;
    }
}

/// Another unit of data within a graph, just to demonstrate we can combine
/// arbitrary types, so long as we know how to translate the output of one to
/// the input of another.
#[derive(Value, Default, Hash)]
pub struct NumberValueI8 {
    pub value: i8,
}

impl NumberValueI8 {
    pub fn new(value: i8) -> Self {
        Self { value }
    }
}

impl NumberLike for NumberValueI8 {
    fn value(&self) -> i32 {
        self.value as i32
    }
}
impl NumberLike for NumberValueI32 {
    fn value(&self) -> i32 {
        self.value
    }
}

impl UpdateInput for NumberValueI8 {
    type Update = i8;

    fn update_mut(&mut self, update: Self::Update) {
        // Implementing this trait will provide a way for code outside of this graph to
        // change its internal state. This is just a simple replace for now.
        self.value = update;
    }
}

#[derive(Operation)]
pub struct Sum;

impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Sum> for NumberValueI32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() + value.1.data().value();
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Sum>
    for NumberValueI32
{
    fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() + value.1.data().value() + value.2.data().value();
        Ok(())
    }
}

#[derive(Operation)]
pub struct Square;

impl<A: NumberLike> UpdateDerived<DepRef<'_, A>, Square> for NumberValueI32 {
    fn update(&mut self, value: DepRef<'_, A>) -> Result<(), EarlyExit> {
        self.value = value.data().value().pow(2);
        Ok(())
    }
}

#[derive(Operation)]
pub struct Multiply;
impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Multiply> for NumberValueI32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() * value.1.data().value();
        Ok(())
    }
}
