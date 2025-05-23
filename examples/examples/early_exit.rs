use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    rc::Rc,
};

use depends::{
    derives::{Operation, Value},
    error::{EarlyExit, ResolveError},
    DepRef, DepRef3, Dependencies3, Dependency, DerivedNode, InputNode, Resolve, UpdateDerived,
    UpdateInput,
};

/// A dummy tracker for the number of open orders.
#[derive(Value, Default)]
pub struct OpenOrders {
    pub value: Vec<i32>,
    /// Keep track of every time this node is updated, use that as the hash.
    #[depends(hash)]
    generation: usize,
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum OpenOrdersOperation {
    Add(i32),
    Cancel,
}

// By implementing UpdateInput, we can change the value of this node from
// outside of the graph.
impl UpdateInput for OpenOrders {
    type Update = OpenOrdersOperation;

    fn update_mut(&mut self, update: Self::Update) {
        match update {
            OpenOrdersOperation::Add(order) => self.value.push(order),
            OpenOrdersOperation::Cancel => {
                self.value.remove(0);
            }
        }
        self.generation += 1;
    }
}

/// This node is only used to raise an error if the number of open orders is
/// too high.
#[derive(Value, Hash, Debug)]
pub struct RiskLimit {
    max_orders: usize,
}

impl RiskLimit {
    pub fn new(max_orders: usize) -> Self {
        Self { max_orders }
    }
}

#[derive(Operation)]
struct CheckRiskLimit;

impl UpdateDerived<DepRef<'_, OpenOrders>, CheckRiskLimit> for RiskLimit {
    fn update(&mut self, value: DepRef<'_, OpenOrders>) -> Result<(), EarlyExit> {
        let orders = value.value.len();
        if orders >= self.max_orders {
            Err(EarlyExit::new(format!("Risk limit exceeded ({})", orders)))
        } else {
            Ok(())
        }
    }
}

/// An expensive calculation that we only want to perform if we're ok to
/// continue trading.
#[derive(Value, Hash, Default, Debug)]
pub struct ExpensiveCalculation {
    pub next_number: i32,
}

#[derive(Operation)]
struct CalculateNextNumber;

impl UpdateDerived<DepRef<'_, OpenOrders>, CalculateNextNumber> for ExpensiveCalculation {
    fn update(&mut self, _: DepRef<'_, OpenOrders>) -> Result<(), EarlyExit> {
        println!("Expensive calculation performed!");
        self.next_number += 1;
        Ok(())
    }
}

#[derive(Value, Hash, Default, Debug)]
pub struct DecisionNode {
    value: Option<OpenOrdersOperation>,
}

/// We must define the transformation functions for each derived node.
/// Given a set of inputs and a target value, describe how to update the
/// state. In this example, it's just a simple addition.
#[derive(Operation)]
struct Decide;

impl UpdateDerived<DepRef3<'_, OpenOrders, RiskLimit, ExpensiveCalculation>, Decide>
    for DecisionNode
{
    fn update(
        &mut self,
        value: DepRef3<'_, OpenOrders, RiskLimit, ExpensiveCalculation>,
    ) -> Result<(), EarlyExit> {
        self.value = Some(OpenOrdersOperation::Add(value.2.next_number));
        Ok(())
    }
}

fn main() {
    // Our only input node to this graph is the number of open orders.
    let open_orders = InputNode::new(OpenOrders::default());

    let risk_node = DerivedNode::new(
        Dependency::new(Rc::clone(&open_orders)),
        CheckRiskLimit,
        RiskLimit::new(5),
    );
    let expensive_node = DerivedNode::new(
        Dependency::new(Rc::clone(&open_orders)),
        CalculateNextNumber,
        ExpensiveCalculation::default(),
    );
    // It's not _necessary_ to create edges from risk_limit and open_orders to
    // this node, because they are already transitively connected through
    // `expensive_node`.
    let decision = DerivedNode::new(
        Dependencies3::new(Rc::clone(&open_orders), risk_node, expensive_node),
        Decide,
        DecisionNode::default(),
    );

    let mut visitor = HashSet::<usize>::new();
    for i in 0..=10 {
        println!("------------");
        println!("Iteration {}", i);
        // Resolve the graph.
        let decision = match decision.resolve_root(&mut visitor) {
            Ok(order) => order.value.unwrap(),
            Err(ResolveError::EarlyExit(e)) => {
                println!("Early exit: {}, popping order", e);
                OpenOrdersOperation::Cancel
            }
            _ => panic!("Unexpected error"),
        };
        println!("Decision: {:?}", decision);
        // Update the graph
        open_orders.update(decision).unwrap();
        println!("Current orders: {:?}", open_orders.value().unwrap().value);
    }
}
