# Complex Input Nodes

Often we'll want nodes to contain complex data structures and present efficient interfaces to any dependents.

Let's take the example we used earlier to demonstrate how to clean a node:

```rust
{{#include ../../examples/src/docs/complex_value.rs:custom_clean}}
```

We can add the implementation of `UpdateInput` to this struct:

```rust
{{#include ../../examples/src/docs/complex_value.rs:update_input}}
```

We're now able to provide `Posts` to an `InputNode`.

```rust
{{#include ../../examples/src/docs/complex_value.rs:init_input_node}}
```
