# Hashing

All values in a dependency graph must provide a hash (a unique identifier) for their state, or indicate they _can't_ be
hashed. This is used to determine if whether a value has changed since the last time it was observed.

There are three ways to define a value's hashing behaviour:

**1. Default:** Requires the type to implement the [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) trait.

```rust
{{#include ../../examples/src/docs/hashing.rs:default_hashing}}
```

**2. Custom Hash:** Annotate a field with the `#[depends(hash)]` attribute to manually manage the hashing behaviour.

```rust
{{#include ../../examples/src/docs/hashing.rs:custom_hashing}}
```

**3. Unhashable:** Mark values that can't be hashed with the `#[depends(unhashable)]` attribute. This type will always
appear dirty to any dependents, causing them to always recalculate their own state.

```rust
{{#include ../../examples/src/docs/hashing.rs:no_hashing}}
```

> **It's unlikely you'll need to use the `unhashable` attribute and this can greatly reduce the efficiency of
> computations. Most nodes can use a custom hash field instead.**

