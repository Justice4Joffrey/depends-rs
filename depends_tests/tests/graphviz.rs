mod common;
use depends::{core::Resolve, graphviz::GraphvizVisitor};

#[test]
fn test_graphviz() {
    let graph = common::my_graph();
    let mut visitor = GraphvizVisitor::new();
    graph.answer.resolve(&mut visitor);

    assert_eq!(
        r#"
digraph G {
  0[label="NumberInput"];
  1[label="NumberInput"];
  2[label="NumberInput"];
  3[label="Sum"];
  0 -> 3;
  1 -> 3;
  4[label="Multiply"];
  0 -> 4;
  2 -> 4;
  5[label="Answer"];
  3 -> 5;
  4 -> 5;
}
"#
        .trim(),
        visitor.render().unwrap()
    );
}
