## Values

A `Value` is a stateful unit of data held within a dependency graph node. During execution, values will be prompted to recalculate their state if their dependencies change _their_ values.

A `Value` is required to declare how it can be:

- [Hashed](./hashing.md): Provide a single unique integer for its state (or indicate it _can't_ be hashed).
- [Cleaned](./cleaning.md): Declare how to clean any transient state.

Use the `#[derive(Value)]` macro to make a struct compatible with the dependency graph.

```rust
{{#include ../../examples/src/docs/getting_started_value.rs::}}
```

The default behaviour of this type is to:

- Hash all fields.
- Not clean any transient state.
