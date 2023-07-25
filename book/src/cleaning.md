# Cleaning

By default, values in the Depends dependency graph do not perform any cleanup between graph calculations. This means any
transient state being tracked will remain.

If you need to perform cleanup on a value, you can specify the `#[depends(custom_clean)]` attribute.

```rust
{{#include ../../examples/src/docs/complex_value.rs:custom_clean}}
```

When this attribute is used, you are required to implement the [Clean](https://docs.rs/depends/latest/depends/trait.Clean.html)
trait for the struct. The `Clean` trait contains a single method, `clean`, which should be implemented to reset any transient state being tracked by your struct.

This enables you to manually control the cleanup process between computations, ensuring that your transient state is
always correctly managed.

> **Correct cleanup is vital to maintain the accuracy and efficiency of your computations.**
