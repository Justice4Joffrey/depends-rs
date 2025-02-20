# Dependencies

In most real-world applications, you'll have nodes that depend on multiple other nodes. Depends allows you to define complex dependencies using `Dependencies` tuples.

Here's an example where an operation depends on two numbers and multiplies them:

```rust
{{#include ../../examples/src/docs/multiple_dependencies.rs:9::}}
```

You can connect up to 16 dependencies to a single node by using the approriate `DependenciesN` type.

## Checking Specific Dependency State

There are situations where it's useful to know which specific dependencies have caused `update_mut` to be called. For this reason, the `is_dirty` method is available on each dependency reference.

```rust
{{#include ../../examples/src/docs/checking_node_state_directly.rs:is_dirty}}
```

> The most common is example is time. It's expected that no node uses methods such as `Utc::now()`, as this is a side-effect which will result in non-deterministic behaviour.
>
> Instead, you should 'set' the time for the graph by providing it as an [Input Node](./input_nodes.md) and creating edges to the nodes which require it.
