use depends::{
    derives::{Operation, Value},
    error::EarlyExit,
    DepRef, DepRef2, UpdateDerived,
};
use serial_test::serial;

use crate::docs::simple_value::SomeNumber;

#[derive(Operation)]
pub struct Add;

impl UpdateDerived<DepRef2<'_, SomeNumber, SomeNumber>, Add> for SomeNumber {
    fn update(&mut self, deps: DepRef2<'_, SomeNumber, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.0.value + deps.1.value;
        Ok(())
    }
}

#[derive(Operation)]
pub struct Subtract;

impl UpdateDerived<DepRef2<'_, SomeNumber, SomeNumber>, Subtract> for SomeNumber {
    fn update(&mut self, deps: DepRef2<'_, SomeNumber, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.0.value - deps.1.value;
        Ok(())
    }
}

#[derive(Value, Hash, Default)]
pub struct AnotherNumber {
    pub value: i64,
}

#[derive(Operation)]
pub struct Cube;

impl UpdateDerived<DepRef<'_, SomeNumber>, Cube> for AnotherNumber {
    fn update(&mut self, deps: DepRef<'_, SomeNumber>) -> Result<(), EarlyExit> {
        self.value = deps.value.pow(3) as i64;
        Ok(())
    }
}

#[serial]
#[test]
#[rustfmt::skip]
fn test_resolve_graph() {
use super::multiple_dependencies::Multiply;
use super::simple_value::Square;
use depends::test_utils::{ext_reset_node_id, DiagnosticVisitor};
use depends::{
    graphviz::GraphvizVisitor, *
};
use std::rc::Rc;
unsafe {
    ext_reset_node_id();
}
// ANCHOR: complex_graph
// ANCHOR: boilerplate_setup
let a = InputNode::new(SomeNumber { value: 1 });
let b = InputNode::new(SomeNumber { value: 2 });
let c = InputNode::new(SomeNumber { value: 3 });
let d = InputNode::new(SomeNumber { value: 4 });
let e = InputNode::new(SomeNumber { value: 2 });

let a_times_b = DerivedNode::new(
    Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
    Multiply,
    SomeNumber::default(),
);

let d_minus_c = DerivedNode::new(
    Dependencies2::new(Rc::clone(&d), Rc::clone(&c)),
    Subtract,
    SomeNumber::default(),
);
// ANCHOR_END: boilerplate_setup

let d_squared = DerivedNode::new(
    Dependency::new(Rc::clone(&d)),
    Square,
    SomeNumber::default(),
);

let e_squared = DerivedNode::new(
    Dependency::new(Rc::clone(&e)),
    Square,
    SomeNumber::default(),
);

let a_times_b_plus_c_minus_d = DerivedNode::new(
    Dependencies2::new(Rc::clone(&a_times_b), Rc::clone(&d_minus_c)),
    Add,
    SomeNumber::default(),
);

let times_e_squared = DerivedNode::new(
    Dependencies2::new(Rc::clone(&a_times_b_plus_c_minus_d), Rc::clone(&e_squared)),
    Multiply,
    SomeNumber::default(),
);

let minus_d_squared = DerivedNode::new(
    Dependencies2::new(Rc::clone(&times_e_squared), Rc::clone(&d_squared)),
    // Check out the `examples` directory to see the implementation of
    // these new operations.
    Subtract,
    SomeNumber::default(),
);

let cube_and_change_type = DerivedNode::new(
    Dependency::new(Rc::clone(&minus_d_squared)),
    Cube,
    // The graph can be constructed from all sorts of types.
    AnotherNumber::default(),
);

// This visitor will track how many derived nodes have their input
// recalculated.
let mut visitor = DiagnosticVisitor::new();

let output = cube_and_change_type.resolve(&mut visitor).unwrap();
assert_eq!(output.value, -64);

// We calculated all derived nodes.
assert_eq!(visitor.recalculated.len(), 8);
// ANCHOR_END: complex_graph

drop(output);
// ANCHOR: incremental_computation_1
// Since we didn't `resolve_root` last time, clear the visitor manually.
visitor.clear();

// Updating E will only update the nodes which depend on it.
e.update(3).unwrap();

// Resolve our root node.
let output = cube_and_change_type.resolve(&mut visitor).unwrap();
assert_eq!(output.value, 1331);

// We've _visited_ all nodes.
assert_eq!(visitor.visitor.len(), 13);
// But only _recomputed_ the ones which are dependent on `e`.
// (Input nodes do not count towards this total).
assert_eq!(visitor.recalculated.len(), 4);
// ANCHOR_END: incremental_computation_1

drop(output);
visitor.clear();

// ANCHOR: incremental_computation_2
// Let's swap the values of `a` and `b`.
a.update(2).unwrap();
b.update(1).unwrap();

// The end result hasn't changed. 1 * 2 == 2 * 1.
let output = cube_and_change_type.resolve(&mut visitor).unwrap();
assert_eq!(output.value, 1331);

// Only 1 node was recalculated this time.
assert_eq!(visitor.recalculated.len(), 1);
// ANCHOR_END: incremental_computation_2
drop(output);

    let mut visitor = GraphvizVisitor::new();

    cube_and_change_type.resolve(&mut visitor).unwrap();

    println!("{}", visitor.render().unwrap());

    assert_eq!(
        visitor.render().unwrap(),
        r#"
digraph Dag {
  node_0 [label="SomeNumber"];
  node_1 [label="SomeNumber"];
  node_2 [label="SomeNumber"];
  node_3 [label="SomeNumber"];
  node_4 [label="SomeNumber"];
  node_5 [label="SomeNumber"];
  node_0 -> node_5 [label="Multiply", class="Dependencies2"];
  node_1 -> node_5 [label="Multiply", class="Dependencies2"];
  node_6 [label="SomeNumber"];
  node_3 -> node_6 [label="Subtract", class="Dependencies2"];
  node_2 -> node_6 [label="Subtract", class="Dependencies2"];
  node_7 [label="SomeNumber"];
  node_3 -> node_7 [label="Square"];
  node_8 [label="SomeNumber"];
  node_4 -> node_8 [label="Square"];
  node_9 [label="SomeNumber"];
  node_5 -> node_9 [label="Add", class="Dependencies2"];
  node_6 -> node_9 [label="Add", class="Dependencies2"];
  node_10 [label="SomeNumber"];
  node_9 -> node_10 [label="Multiply", class="Dependencies2"];
  node_8 -> node_10 [label="Multiply", class="Dependencies2"];
  node_11 [label="SomeNumber"];
  node_10 -> node_11 [label="Subtract", class="Dependencies2"];
  node_7 -> node_11 [label="Subtract", class="Dependencies2"];
  node_12 [label="AnotherNumber"];
  node_11 -> node_12 [label="Cube"];
}
    "#
        .trim()
    );

    #[allow(unused)]
    struct GraphRoot {
        #[allow(clippy::type_complexity)]
        root:
// ANCHOR: reducing_boilerplate
// Oh dear lord...
Rc<
    DerivedNode<
        Dependency<
            Rc<
                DerivedNode<
                    Dependencies2<
                        Rc<
                            DerivedNode<
                                Dependencies2<
                                    Rc<
                                        DerivedNode<
                                            Dependencies2<
                                                Rc<
                                                    DerivedNode<
                                                        Dependencies2<
                                                            Rc<InputNode<SomeNumber>>,
                                                            Rc<InputNode<SomeNumber>>,
                                                        >,
                                                        SomeNumber,
                                                        Multiply,
                                                    >,
                                                >,
                                                Rc<
                                                    DerivedNode<
                                                        Dependencies2<
                                                            Rc<InputNode<SomeNumber>>,
                                                            Rc<InputNode<SomeNumber>>,
                                                        >,
                                                        SomeNumber,
                                                        Subtract,
                                                    >,
                                                >,
                                            >,
                                            SomeNumber,
                                            Add,
                                        >,
                                    >,
                                    Rc<
                                        DerivedNode<
                                            Dependency<Rc<InputNode<SomeNumber>>>,
                                            SomeNumber,
                                            Square,
                                        >,
                                    >,
                                >,
                                SomeNumber,
                                Multiply,
                            >,
                        >,
                        Rc<
                            DerivedNode<
                                Dependency<Rc<InputNode<SomeNumber>>>,
                                SomeNumber,
                                Square,
                            >,
                        >,
                    >,
                    SomeNumber,
                    Subtract,
                >,
            >,
        >,
        AnotherNumber,
        Cube,
    >,
>
// ANCHOR_END: reducing_boilerplate
        ,}
    let _ = GraphRoot {
        root: cube_and_change_type.clone(),
    };
    visitor.clear();

#[allow(unused)]
// ANCHOR: impl_trait
struct Graph<R> {
    // Keep references to the input nodes so they can be updated.
    a: Rc<InputNode<SomeNumber>>,
    b: Rc<InputNode<SomeNumber>>,
    c: Rc<InputNode<SomeNumber>>,
    d: Rc<InputNode<SomeNumber>>,
    e: Rc<InputNode<SomeNumber>>,
    // Keep a reference to the root node so it can be resolved.
    cube_and_change_type: R
}

impl<R> Graph<R>
where
    // R must be a node which resolves to a reference to AnotherNumber.
    R: for<'a> Resolve<Output<'a> = NodeRef<'a, AnotherNumber>>,
{
    // We can only call this constructor with a root node which
    // resolves to the correct type.
    pub fn new(
        a: Rc<InputNode<SomeNumber>>,
        b: Rc<InputNode<SomeNumber>>,
        c: Rc<InputNode<SomeNumber>>,
        d: Rc<InputNode<SomeNumber>>,
        e: Rc<InputNode<SomeNumber>>,
        cube_and_change_type: R,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            cube_and_change_type,
        }
    }
}
// ANCHOR_END: impl_trait

// ANCHOR: init_impl_trait
// Create a new graph.
let graph = Graph::new(a, b, c, d, e, cube_and_change_type);

// We can now interact with the inputs and root node.
graph.b.update(-1).unwrap();

let output = graph.cube_and_change_type.resolve(&mut visitor).unwrap();
assert_eq!(output.value, -15625);
// ANCHOR_END: init_impl_trait
}
