mod common;

use std::rc::Rc;

use depends::{
    core::{Dependency, DerivedNode, InputNode, Resolve},
    graphviz::GraphvizVisitor,
};

use crate::common::*;

#[test]
fn test_graphviz() {
    let a = InputNode::new(NumberValue::new(2));
    let b = InputNode::new(NumberValue::new(4));
    let c = InputNode::new(NumberValue::new(3));
    let c_sq = DerivedNode::new(
        Dependency::new(Rc::clone(&c)),
        square,
        NumberValue::default(),
    );
    let ab = TwoNumbers::init(Rc::clone(&a), Rc::clone(&b));
    let sum_ab = DerivedNode::new(ab, add, NumberValue::default());
    let sum_ab_c_sq = TwoNumbers::init(Rc::clone(&c_sq), Rc::clone(&sum_ab));
    let answer = DerivedNode::new(sum_ab_c_sq, multiply, NumberValue::default());

    let mut visitor = GraphvizVisitor::new();
    {
        let result = answer.resolve(&mut visitor).unwrap();
        assert_eq!(result.value, (2 + 4) * (3 * 3));

        assert_eq!(
            r#"
digraph G {
  0[label="NumberValue"];
  1[label="NumberValue"];
  2[label="NumberValue"];
  3[label="NumberValue"];
  2 -> 3;
  4[label="NumberValue"];
  0 -> 4;
  1 -> 4;
  5[label="NumberValue"];
  3 -> 5;
  4 -> 5;
}
    "#
            .trim(),
            visitor.render().unwrap()
        );
    }
}
