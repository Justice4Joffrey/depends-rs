# Depends

[![Crates.io](https://img.shields.io/crates/v/depends.svg)](https://crates.io/crates/depends)
[![Documentation](https://docs.rs/depends/badge.svg)](https://docs.rs/depends/)
[![Codecov](https://codecov.io/gh/Justice4Joffrey/depends-rs/coverage.svg?branch=master)](https://codecov.io/gh/Justice4Joffrey/depends-rs)
[![Dependency status](https://deps.rs/repo/github/Justice4Joffrey/depends-rs/status.svg)](https://deps.rs/repo/github/Justice4Joffrey/depends-rs)

A library for ergonomic, performant, incremental computation between arbitrary types.

For more information, see:
- [Getting Started Guide]()
- [API Documentation](https://docs.rs/depends/)
- [Examples](https://github.com/Justice4Joffrey/depends-rs/tree/master/examples).
- [Benchmarks](https://github.com/Justice4Joffrey/depends-rs/tree/master/benches).

## Motivation

Many applications which respond to changes from multiple input sources benefit from the use of dependency graphs as code structure. By breaking complex states down in to small pieces of testable, composable logic, scaling and maintaining applications becomes much easier over time. Additionally, incremental computation allows results of previous calculations to be reused where possible, improving overall efficiency and performance.

Depends aims to present the smallest possible API surface for building minimal runtime overhead dependency graphs in Rust, whilst leveraging the compile-time guarantees of the type-system.

```rust
/// A unit of data within a graph.
#[derive(Value, Hash)]
pub struct NumberValue {
    pub value: i32,
}

// Below are input nodes, which update their internal values from
// outside the graph.
let a = InputNode::new(NumberValue::new(7));
let b = InputNode::new(NumberValue::new(6));

// Derived nodes take their value from other nodes.
let c = DerivedNode::new(
    TwoNumbers::init(Rc::clone(&a), Rc::clone(&b)),
    Multiply,
    NumberValue::default(),
);

// A visitor tracks which nodes have been visited during a resolve.
let mut visitor = HashSetVisitor::new();

// Resolve the graph on-demand to propagate changes and produce a result.
let output = c.resolve_root(&mut visitor).unwrap();
assert_eq!(output.value, 42);

// Update the value of `a` and resolve again.
a.update(70).unwrap();

// Only values which depend on `a` will be updated when the graph is next resolved.
let output = c.resolve_root(&mut visitor).unwrap();
assert_eq!(output.value, 420);
```

For more detailed examples, including (de)serialization with [Graphviz](https://graphviz.org/), see the [Getting Started Guide]().

## Current Status

This crate should be considered a Proof Of Concept and treated with reasonable amounts of scepticism.

The guarantees we would _like_ to offer, before considering this crate production-ready are:

- Determinism of output given any sequence of inputs and actions.
- Graphs cannot yield different results between calls to `resolve` without a change in input, other than when `Clean`
  has been implemented incorrectly.
- Correctness of the internal caching logic.

Feel free to experiment with the crate, apply it to your problems and pass on any feedback you have.
