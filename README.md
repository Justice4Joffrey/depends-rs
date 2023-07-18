# Depends

[![Crates.io](https://img.shields.io/crates/v/depends.svg)](https://crates.io/crates/depends)
[![Documentation](https://docs.rs/depends/badge.svg)](https://docs.rs/depends/)
[![Codecov](https://codecov.io/gh/Justice4Joffrey/depends-rs/coverage.svg?branch=master)](https://codecov.io/gh/Justice4Joffrey/depends-rs)
[![Dependency status](https://deps.rs/repo/github/Justice4Joffrey/depends-rs/status.svg)](https://deps.rs/repo/github/Justice4Joffrey/depends-rs)

A library for ergonomic, performant, incremental computation between arbitrary types.

See [benchmarks](https://github.com/Justice4Joffrey/depends-rs/tree/master/benches).

## Why would I want that?

Most applications rely on some core logic which must respond to external
events. Often, the logic to transform each event in to an action is
straightforward, but as the application scales, many hard to reason-with
situations emerge from the combinatorial explosion of states.

Dependency graphs are an excellent code architecture to tame complexity in
such scenarios.

```rust
/// A unit of data within a graph.
#[derive(Value, Default, Hash)]
pub struct NumberValue {
    pub value: i32,
}

impl NumberValue {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

// By implementing UpdateInput, we can change the value of this node from
// outside of the graph.
impl UpdateInput for NumberValue {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        // Implementing this trait will provide a way for code outside of this graph to
        // change its internal state. This is just a simple replace for now.
        self.value = update;
    }
}

/// Any [DerivedNode] must state its dependencies. If there are more than one,
/// this must be wrapped in a struct which derived [Dependencies] as shown.
/// Otherwise, it can be passed to a [DerivedNode] wrapped with the
/// [Dependency] type.
#[derive(Dependencies)]
pub struct TwoNumbers {
    left: NumberValue,
    right: NumberValue,
}

/// We must define the transformations for each derived node.
/// Given a set of inputs and a target value, describe how to update the
/// state. In this example, it's just a simple multiplication.
#[derive(Operation)]
struct Multiply;

// Note [TwoNumbersRef] is generated by the [Dependencies] macro, and
// represents read-references to all of its fields.
impl UpdateDerived for Multiply {
    type Input<'a> = TwoNumbersRef<'a> where Self: 'a;
    type Target<'a> = TargetMut<'a, NumberValue> where Self: 'a;

    fn update_derived(
        TwoNumbersRef { left, right }: TwoNumbersRef<'_>,
        mut target: TargetMut<'_, NumberValue>,
    ) -> Result<(), EarlyExit> {
        target.value = left.value * right.value;
        Ok(())
    }
}

// Compose a graph!
// Below are input nodes, which are nodes which take new values from
// outside the graph.
let a = InputNode::new(NumberValue::new(7));
let b = InputNode::new(NumberValue::new(6));

// Derived nodes take their value from other nodes (either input or
// derived). Note that we can combine _any_ type of node, not just
// [NumberValue]s.
let c = DerivedNode::new(
    TwoNumbers::init(Rc::clone(&a), Rc::clone(&b)),
    Multiply,
    NumberValue::default(),
);

// A visitor tracks which nodes have been visited during a resolve.
let mut visitor = HashSet::<usize>::new();

// Resolve the graph!
// `resolve_root` will clear the visitor before returning, readying it
// for the next resolution.
// This can fail if there are cycles in the graph or an existing read
// reference is being held.
let res = c.resolve_root(&mut visitor).unwrap();
assert_eq!(res.value, 42);

// Nodes which have an edge to dependencies which are updated between
// resolves will recalculate their state on-demand. Others will return
// a cached value. This is known as incremental computation, and can
// vastly improve performance of complex calculations.
a.update(70).unwrap();

// Any dependent values will be updated next time the graph is resolved.
let res = c.resolve_root(&mut visitor).unwrap();
assert_eq!(res.value, 420);
```

Clearly, to implement a simple multiplication problem, a dependency graph is
overkill. However, for more complex problems, where many inputs can change
and the output is a combination of many transformations on that input (and
derivations of it), `depends` can help you produce scalable, performant,
testable code out of the box.

## Graphviz

### Rendering

Any graph built using `depends` can be converted to a [Graphviz](https://graphviz.org/) representation
by passing a `GraphvizVisitor` (this requires the feature `graphviz`).

```rust
let mut visitor = GraphvizVisitor::new();
graph.resolve(&mut visitor).unwrap();
assert_eq!(
    graph.render().unwrap(),
    r#"
digraph Dag {
  node_0 [label="NumberValue"];
  node_1 [label="NumberValue"];
  node_0 -> node_1 [label="Square"];
  node_2 [label="NumberValue"];
  node_3 [label="NumberValue"];
  node_1 -> node_3 [label="Add", class="TwoNumbersDep"];
  node_2 -> node_3 [label="Add", class="TwoNumbersDep"];
  node_4 [label="NumberValue"];
  node_3 -> node_4 [label="Square"];
}
"#);
```

The graph in the above example is rendered below.

![Alt text](./product.svg)

### Interpreting

It's also possible to _build_ strongly-typed graphs in `depends` from a Graphviz definition.

This has the advantage of safely implementing `Send` for the graph type, despite the fact that internally it uses `Rc`s.
```rust
// `label` is the name of the type in scope to use as a node.
// `class` is the name of the dependency type in scope (required iff there are multiple edges to a node).
#[derive(Graph)]
#[depends(
  digraph Dag {
    node_0 [label="NumberValue"];
    node_1 [label="NumberValue"];
    node_0 -> node_1 [label="Square"];
    node_2 [label="NumberValue"];
    node_3 [label="NumberValue"];
    node_1 -> node_3 [label="Add", class="TwoNumbersDep"];
    node_2 -> node_3 [label="Add", class="TwoNumbersDep"];
    node_4 [label="NumberValue"];
    node_3 -> node_4 [label="Square"];
  }
)]
struct MyGraphBuilder;

// create a graph with some initial values.
let graph = MyGraphBuilder::create_dag(
    NumberValue::new(7),
    NumberValue::new(6),
    NumberValue::default(),
    NumberValue::default(),
    NumberValue::default(),
);
let mut visitor = HashSet::new();
graph.resolve(&mut visitor).unwrap();
graph.update_node_0(70).unwrap();
graph.resolve().unwrap();
```

## Current Status

This crate should be considered a Proof Of Concept only and treated with huge amounts of scepticism and distrust.

The guarantees we would _like_ to offer, before considering this crate production-worthy are:

- Determinism of output given any sequence of inputs and actions.
- Graphs cannot yield different results between calls to `resolve` without a change in input, other than when `Clean`
  has been implemented incorrectly.
- Correctness of the internal caching logic.

Feel free to experiment with the crate, apply it to your problems and pass on any feedback you have.

## Examples

You can run the examples from `examples/` with:

```

cd examples
cargo run --example <example_name>

```
