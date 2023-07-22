use std::collections::HashSet;

use depends::{derives::Graph, Resolve};

use crate::maths::*;

mod maths;

// Connecting graph nodes and tracking the complex types can be tedious!
// Fortunately, we can auto-generate some of the boilerplate using the
// [Graph] macro.
//
// This has the added benefit of providing a safe `Send` implementation for
// the graph, which is useful for threaded/async environments.
//
// Note that a graph is definitely _not_ `Sync`, seeing as it uses `RefCell`
// for interior mutability.
#[derive(Graph)]
#[depends(
    digraph MyDag {
        node_0 [label="NumberValue"];
        node_1 [label="NumberValue"];
        node_2 [label="OtherNumberValue"];
        node_3 [label="NumberValue"];
        node_4 [label="NumberValue"];
        node_5 [label="NumberValue"];
        node_2 -> node_5 [label="Square"];
        node_6 [label="NumberValue"];
        node_0 -> node_6 [label="Add", class="TwoNumbersDep"];
        node_1 -> node_6 [label="Add", class="TwoNumbersDep"];
        node_7 [label="NumberValue"];
        node_3 -> node_7 [label="Multiply", class="TwoNumbersDep"];
        node_4 -> node_7 [label="Multiply", class="TwoNumbersDep"];
        node_8 [label="NumberValue"];
        node_0 -> node_8 [label="Add", class="TwoNumbersDep"];
        node_7 -> node_8 [label="Add", class="TwoNumbersDep"];
        node_9 [label="NumberValue"];
        node_5 -> node_9 [label="AddThree", class="ThreeNumbersDep"];
        node_6 -> node_9 [label="AddThree", class="ThreeNumbersDep"];
        node_8 -> node_9 [label="AddThree", class="ThreeNumbersDep"];
    }
)]
struct GraphCreator {}

fn main() {
    // Provide initial values for all nodes.
    let graph = GraphCreator::create_my_dag(
        NumberValue::new(4),
        NumberValue::new(5),
        NumberValue::new(1),
        NumberValue::new(2),
        OtherNumberValue::new(3),
        NumberValue::default(),
        NumberValue::default(),
        NumberValue::default(),
        NumberValue::default(),
        NumberValue::default(),
    );

    let mut visitor = HashSet::<usize>::new();
    {
        let res = graph.resolve_root(&mut visitor).unwrap();
        println!("Answer 1: {}", res.value);
        assert_eq!(res.value, (3_i32.pow(2)) + (1 + 2) + (1 + (4 * 5)));
    }

    // We can move the graph to other threads, despite the fact that it holds
    // `Rc`s. This is safe because they are private and never mutated
    // until dropped.
    let (graph, mut visitor) = std::thread::spawn(|| {
        // The graph provides a safe API for updating all input nodes.
        graph.update_node_0(10).unwrap();
        graph.update_node_1(12).unwrap();
        {
            let res = graph.resolve_root(&mut visitor).unwrap();
            println!("Answer 2: {}", res.value);
            assert_eq!(res.value, (3_i32.pow(2)) + (10 + 12) + (10 + (4 * 5)));
        }
        (graph, visitor)
    })
    .join()
    .unwrap();

    {
        graph.update_node_2(5).unwrap();
        let res = graph.resolve_root(&mut visitor).unwrap();
        println!("Answer 3: {}", res.value);
        assert_eq!(res.value, (5_i32.pow(2)) + (10 + 12) + (10 + (4 * 5)));
    }
}
