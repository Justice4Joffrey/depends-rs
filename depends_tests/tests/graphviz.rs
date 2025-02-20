use std::rc::Rc;

use depends::{
    graphviz::GraphvizVisitor, Dependencies2, Dependencies3, Dependency, DerivedNode, InputNode,
    Resolve,
};
use examples::maths::*;

#[test]
fn test_graphviz() {
    let a = InputNode::new(NumberValueI32::new(4));
    let b = InputNode::new(NumberValueI32::new(5));
    let c = InputNode::new(NumberValueI8::new(6));
    let d = InputNode::new(NumberValueI8::new(7));

    let c_sq = DerivedNode::new(
        Dependency::new(Rc::clone(&c)),
        Square,
        NumberValueI32::default(),
    );
    let product_ab = DerivedNode::new(
        Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
        Multiply,
        NumberValueI32::default(),
    );

    let answer = DerivedNode::new(
        Dependencies3::new(Rc::clone(&product_ab), Rc::clone(&c_sq), Rc::clone(&d)),
        Sum,
        NumberValueI32::default(),
    );

    let mut visitor = GraphvizVisitor::new();
    {
        let result = answer.resolve(&mut visitor).unwrap();
        assert_eq!(result.value, (4 * 5) + (6 * 6) + 7);

        println!("{}", visitor.render().unwrap());
        assert_eq!(
            r#"
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
    "#
            .trim(),
            visitor.render().unwrap()
        );
    }
}
