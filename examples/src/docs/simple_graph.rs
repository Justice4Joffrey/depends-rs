use serial_test::serial;

#[serial]
#[test]
#[rustfmt::skip]
fn test_resolve_graph() {
use std::rc::Rc;
use depends::{graphviz::GraphvizVisitor, *};
use depends::test_utils::ext_reset_node_id;
use crate::docs::{
    multiple_dependencies::Multiply,
    simple_value::SomeNumber,
};
unsafe {
    ext_reset_node_id();
}
// ANCHOR: simple_graph
let input_1 = InputNode::new(SomeNumber { value: 6 });
let input_2 = InputNode::new(SomeNumber { value: 7 });

// Derived is the root node of this graph.
let derived = DerivedNode::new(
    Dependencies2::new(Rc::clone(&input_1), Rc::clone(&input_2)),
    Multiply,
    SomeNumber::default(),
);

// We'll need to create a visitor to resolve the graph. This keeps track
// of which nodes have been visited during depth-first traversal.
let mut visitor = GraphvizVisitor::new();

{
    // Let's put output in a scope so that all shared-references are dropped.
    let output = derived.resolve_root(&mut visitor).unwrap();
    assert_eq!(output.value, 42);
}
// Update one of the inputs.
input_1.update(60).unwrap();
{
    let output = derived.resolve_root(&mut visitor).unwrap();
    // The output should be updated on-demand.
    assert_eq!(output.value, 420);
}
// ANCHOR_END: simple_graph

// ANCHOR: graphviz
// Use the special GraphvizVisitor to track the graph structure.
let mut visitor = GraphvizVisitor::new();

// Use `resolve` to _avoid_ clearing the visitor.
derived.resolve(&mut visitor).unwrap();

assert_eq!(
    visitor.render().unwrap(),
    r#"
digraph Dag {
  node_0 [label="SomeNumber"];
  node_1 [label="SomeNumber"];
  node_2 [label="SomeNumber"];
  node_0 -> node_2 [label="Multiply", class="Dependencies2"];
  node_1 -> node_2 [label="Multiply", class="Dependencies2"];
}
"#
    .trim()
);
// ANCHOR_END: graphviz
}
