# Input Nodes

In the context of a computation graph, [Input Nodes](https://docs.rs/depends/latest/depends/struct.InputNode.html)
are nodes that have no dependencies. These are commonly referred to as leaf nodes and their values are sourced from
outside the graph.

## UpdateInput

To facilitate a [Value](./values.md) being used in an input node we need to implement the
[`UpdateInput`](https://docs.rs/depends/latest/depends/trait.UpdateInput.html) trait.

The simplest implementation can be seen below:

```rust
{{#include ../../examples/src/docs/simple_value.rs:update_input}}
```

It's now possible to create an `InputNode` from this data and interact with it by passing in updates:

```rust
{{#include ../../examples/src/docs/simple_value.rs:create_simple_input}}
```
