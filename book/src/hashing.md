# Hashing

All nodes in a dependency graph must provide a hash (a unique identifier) for their state, or indicate they _can't_ be
hashed. This is used to determine if a node's state has changed since the last time the graph was calculated.

There are three ways to define a node's hashing behaviour:

**1. Default:** Requires the node to implement the [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)) trait.

```rust
{{#include ../../examples/src/docs/hashing.rs:3:8}}
```

**2. Custom Hash:** Annotate a field with the `#[depends(hash)]` attribute to manually manage the hashing behaviour.

```rust
{{#include ../../examples/src/docs/hashing.rs:9:17}}
```

**3. Unhashable:** Mark nodes that can't be hashed with the `#[depends(unhashable)]` attribute. This marks the node as
always dirty, causing dependent nodes to always be recalculated.

```rust
{{#include ../../examples/src/docs/hashing.rs:18:24}}
```

> **It's unlikely you'll need to use the `unhashable` attribute and this can greatly reduce the efficiency of
> computations. Most nodes can use a custom hash field instead.**

