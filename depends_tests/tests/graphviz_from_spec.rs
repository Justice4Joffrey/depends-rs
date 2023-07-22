mod common;

use common::*;
use depends::{derives::Graph, HashSetVisitor, Resolve};

#[derive(Graph)]
#[depends(
    digraph Dag {
        node_0 [label="NumberValue"];
        node_1 [label="NumberValue"];
        node_2 [label="NumberValue"];
        node_3 [label="NumberValue"];
        node_2 -> node_3 [label="Square"];
        node_4 [label="NumberValue"];
        node_0 -> node_4 [label="Add", class="TwoNumbersDep"];
        node_1 -> node_4 [label="Add", class="TwoNumbersDep"];
        node_5 [label="NumberValue"];
        node_3 -> node_5 [label="Multiply", class="TwoNumbersDep"];
        node_4 -> node_5 [label="Multiply", class="TwoNumbersDep"];
    }
)]
struct GraphCreator;

#[test]
fn test_graphviz_from_spec() {
    let graph = GraphCreator::create_dag(
        NumberValue::new(2),
        NumberValue::new(4),
        NumberValue::new(3),
        NumberValue::default(),
        NumberValue::default(),
        NumberValue::default(),
    );
    let mut visitor = HashSetVisitor::new();
    let result = graph.resolve_root(&mut visitor).unwrap();

    assert_eq!(result.value, (2 + 4) * (3 * 3));
}
