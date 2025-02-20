use depends::{derives::Graph, Dependencies2, Dependencies3, HashSetVisitor, Resolve};
use examples::maths::*;

#[derive(Graph)]
#[depends(
    digraph Dag {
        node_0 [label="NumberValueI32"];
        node_1 [label="NumberValueI32"];
        node_2 [label="NumberValueI8"];
        node_3 [label="NumberValueI8"];
        node_4 [label="NumberValueI32"];
        node_2 -> node_4 [label="Square"];
        node_5 [label="NumberValueI32"];
        node_0 -> node_5 [label="Multiply", class="Dependencies2"];
        node_1 -> node_5 [label="Multiply", class="Dependencies2"];
        node_6 [label="NumberValueI32"];
        node_5 -> node_6 [label="Sum", class="Dependencies3"];
        node_4 -> node_6 [label="Sum", class="Dependencies3"];
        node_3 -> node_6 [label="Sum", class="Dependencies3"];
    }
)]
struct GraphCreator;

#[test]
fn test_graphviz_from_spec() {
    let graph = GraphCreator::create_dag(
        NumberValueI32::new(2),
        NumberValueI32::new(3),
        NumberValueI8::new(4),
        NumberValueI8::new(5),
        NumberValueI32::default(),
        NumberValueI32::default(),
        NumberValueI32::default(),
    );
    let mut visitor = HashSetVisitor::new();
    let result = graph.resolve_root(&mut visitor).unwrap();

    assert_eq!(result.value, (2 * 3) + (4 * 4) + 5);
}
