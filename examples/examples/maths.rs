use std::rc::Rc;

use depends::{
    Dependencies2, Dependencies3, Dependency, DerivedNode, HashSetVisitor, InputNode, Resolve,
};
// Check out the source code here.
use examples::maths::*;

// (As imported in graph.rs)
#[allow(unused)]
fn main() {
    // Create some input nodes. These are nodes we can update from outside of
    // the graph.
    let a = InputNode::new(NumberValueI32::new(1));
    let b = InputNode::new(NumberValueI32::new(2));
    // Note that we can combine different types in the same graph.
    let c = InputNode::new(NumberValueI8::new(3));
    let d = InputNode::new(NumberValueI32::new(4));
    let e = InputNode::new(NumberValueI32::new(5));

    // Now for some derived nodes. We can't update these from outside of the
    // graph. They are updated when their dependencies change.

    // C squared
    let g = DerivedNode::new(
        Dependency::new(Rc::clone(&c)),
        Square,
        NumberValueI32::default(),
    );
    // Sum of a and b
    let h = DerivedNode::new(
        Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
        Sum,
        NumberValueI32::default(),
    );

    // Product of d and e
    let i = DerivedNode::new(
        Dependencies2::new(Rc::clone(&d), Rc::clone(&e)),
        Multiply,
        NumberValueI32::default(),
    );
    // Create another edge to node a
    let j = DerivedNode::new(
        Dependencies2::new(Rc::clone(&a), Rc::clone(&i)),
        Sum,
        NumberValueI32::default(),
    );

    // Finally, the sum of all of the above
    let answer = DerivedNode::new(
        Dependencies3::new(Rc::clone(&g), Rc::clone(&h), Rc::clone(&j)),
        Sum,
        NumberValueI32::default(),
    );

    // We can render the graph to Graphviz format
    // let mut visitor = GraphvizVisitor::new();
    // answer.resolve(&mut visitor).unwrap();
    // println!("{}", visitor.render().unwrap());

    // We can now resolve the graph. This will update all of the derived
    // nodes
    let mut visitor = HashSetVisitor::new();
    {
        // This can fail if there are cycles in the graph or an existing read
        // reference is being held.
        let res = answer.resolve_root(&mut visitor).unwrap();
        println!("Answer 1: {}", res.value);
        assert_eq!(res.value, (3_i32.pow(2)) + (1 + 2) + (1 + (4 * 5)));
    }
    {
        // If we update input nodes, any nodes which depend on them will be
        // re-resolved. Those which don't will return their cached value.
        a.update(10).unwrap();
        b.update(12).unwrap();
        let res = answer.resolve_root(&mut visitor).unwrap();
        println!("Answer 2: {}", res.value);
        assert_eq!(res.value, (3_i32.pow(2)) + (10 + 12) + (10 + (4 * 5)));
    }
    {
        // The graph contains multiple types whose edges are type-checked.
        // Therefore, only valid graphs can be constructed.
        c.update(5).unwrap();
        let res = answer.resolve_root(&mut visitor).unwrap();
        println!("Answer 3: {}", res.value);
        assert_eq!(res.value, (5_i32.pow(2)) + (10 + 12) + (10 + (4 * 5)));
    }
}
