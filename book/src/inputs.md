# Inputs

In the context of a computation graph, [Input Nodes](https://docs.rs/depends/0.9.0/depends/struct.InputNode.html)
are nodes that have no dependencies. These are commonly referred to as leaf nodes and their values are sourced from
outside of the graph.

Some node types can be used both as input nodes and derived nodes (more on that later).

## UpdateInput

To facilitate this node taking data from outside the graph, we need to implement the
[`UpdateInput`](https://docs.rs/depends/latest/depends/trait.UpdateInput.html) trait.

The simplest implementation can be seen below:

```rust
{{#include ../../examples/src/docs/simple_value.rs::}}
```

### A More Complex Example

Let's take the example we used before to demonstrate how to clean a node:

```rust
{{#include ../../examples/src/docs/complex_value.rs:5:38}}
```

We can add the implementation of `UpdateInput` to this struct:

```rust
{{#include ../../examples/src/docs/complex_value.rs:39:54}}
```

We're now able to provide `Posts` to an `InputNode`.

```rust
{{#include ../../examples/src/docs/complex_value.rs:58:68}}
```

