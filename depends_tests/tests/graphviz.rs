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
digraph G {
  0[label="NumberValue"];
  1[label="NumberValue"];
  2[label="NumberValue"];
  3[label="NumberValue"];
  2 -> 3[label="Square"];
  4[label="NumberValue"];
  0 -> 4[label="Add"];
  1 -> 4[label="Add"];
  5[label="NumberValue"];
  3 -> 5[label="Multiply"];
  4 -> 5[label="Multiply"];
}
    "#
            .trim(),
            visitor.render().unwrap()
        );
    }
}
