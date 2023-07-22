mod common;

use std::rc::Rc;

use depends::{graphviz::GraphvizVisitor, Dependency, DerivedNode, InputNode, Resolve};

use crate::common::*;

#[test]
fn test_graphviz() {
    let a = InputNode::new(NumberValue::new(2));
    let b = InputNode::new(NumberValue::new(4));
    let c = InputNode::new(NumberValue::new(3));
    let c_sq = DerivedNode::new(
        Dependency::new(Rc::clone(&c)),
        Square,
        NumberValue::default(),
    );
    let ab = TwoNumbers::init(Rc::clone(&a), Rc::clone(&b));
    let sum_ab = DerivedNode::new(ab, Add, NumberValue::default());
    let sum_ab_c_sq = TwoNumbers::init(Rc::clone(&c_sq), Rc::clone(&sum_ab));
    let answer = DerivedNode::new(sum_ab_c_sq, Multiply, NumberValue::default());

    let mut visitor = GraphvizVisitor::new();
    {
        let result = answer.resolve(&mut visitor).unwrap();
        assert_eq!(result.value, (2 + 4) * (3 * 3));

        println!("{}", visitor.render().unwrap());
        assert_eq!(
            r#"
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
    "#
            .trim(),
            visitor.render().unwrap()
        );
    }
}
