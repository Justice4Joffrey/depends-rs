# Depends

[![Crates.io](https://img.shields.io/crates/v/depends.svg)](https://crates.io/crates/depends)
[![Documentation](https://docs.rs/depends/badge.svg)](https://docs.rs/depends/)
[![Codecov](https://codecov.io/gh/Justice4Joffrey/depends-rs/coverage.svg?branch=master)](https://codecov.io/gh/Justice4Joffrey/depends-rs)
[![Dependency status](https://deps.rs/repo/github/Justice4Joffrey/depends-rs/status.svg)](https://deps.rs/repo/github/Justice4Joffrey/depends-rs)

A library for ergonomic, performant, incremental computation between arbitrary types.

For more information, see:

- [Getting Started Guide](https://justice4joffrey.github.io/depends-rs)
- [API Documentation](https://docs.rs/depends/)
- [Examples](https://github.com/Justice4Joffrey/depends-rs/tree/master/examples)
- [Benchmarks](https://github.com/Justice4Joffrey/depends-rs/tree/master/benches)

## Motivation

Many applications which respond to changes from multiple input sources benefit from the use of dependency graphs as code structure. By breaking complex states down in to small pieces of testable, composable logic, scaling and maintaining applications becomes much easier over time. Additionally, incremental computation allows results of previous calculations to be reused where possible, improving overall efficiency and performance.

Depends aims to present the smallest possible API surface for building minimal runtime-overhead dependency graphs in Rust, whilst leveraging the compile-time guarantees of the type-system.

```rust
// Below are input nodes, which are nodes which take new values from
// outside the graph.
// It's not common to use primitives, but they make for a simple example.
let a = InputNode::new(7_i64);
let b = InputNode::new(6_i32);

// Derived nodes take their value from other nodes (either input or
// derived). Note that we can combine _any_ type of node, providing
// we've defined an operation (Multiply) for a set of dependencies (Dependencies2).
let c = DerivedNode::new(
    Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
    Multiply,
    0_i64,
);

// A visitor tracks which nodes have been visited during a resolve.
let mut visitor = HashSetVisitor::new();

// Resolve the graph!
// `resolve_root` will clear the visitor before returning, readying it
// for the next resolution.
// This can fail if there are cycles in the graph or an existing read
// reference is being held.
assert_eq!(
    c.resolve_root(&mut visitor).unwrap().value().clone(),
    42
);

// Nodes which have an edge to dependencies which are updated between
// resolves will recalculate their state on-demand. Others will return
// a cached value. This is known as incremental computation, and can
// vastly improve performance of complex calculations.
a.update(70).unwrap();

// Any dependent values will be updated next time the graph is resolved.
assert_eq!(
    c.resolve_root(&mut visitor).unwrap().value().clone(),
    420
);
```

For more detailed examples, including (de)serialization with [Graphviz](https://graphviz.org/), see the [Getting Started Guide](https://justice4joffrey.github.io/depends-rs).

## Current Status

This crate should be considered a Proof Of Concept and treated with reasonable amounts of scepticism.

The guarantees we would _like_ to offer, before considering this crate production-ready are:

- Determinism of output given any sequence of inputs and actions.
- Graphs cannot yield different results between calls to `resolve` without a change in input, other than when `Clean`
  has been implemented incorrectly.
- Correctness of the internal caching logic.

Feel free to experiment with the crate, apply it to your problems and pass on any feedback you have.
