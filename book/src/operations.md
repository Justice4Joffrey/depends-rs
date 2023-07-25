# Operations

An [Operation](https://docs.rs/depends/latest/depends/derives/derive.Operation.html) describes the dependencies of a [Derived Node](./derived_nodes.md), and how those dependencies are used to update the internal [Value](./values.md).

In Depends, you define an Operation by implementing the [UpdateDerived](https://docs.rs/depends/latest/depends/trait.UpdateDerived.html) trait for a struct. This struct, marked with `#[derive(Operation)]`, symbolizes the operation itself.

Here is an example of an Operation that squares a number:

```rust
{{#include ../../examples/src/docs/simple_value.rs:some_number}}

{{#include ../../examples/src/docs/simple_value.rs:square}}
```

`Square` is an `Operation` that takes a `SingleRef` to `SomeNumber` as input, and a `TargetMut` of `SomeNumber` as its target.

The `update_derived` method describes how to use the dependencies (the input) to update the internal value of the derived node (the target).

This operation will take a number and square it. In practice, operations can be any function that transforms the inputs into a new state for the target.

## Early Exit

For some graphs, it may be desirable to exit early from an operation. This can be achieved by returning `Err(EarlyExit)` from the `update_derived` method.

```rust
{{#include ../../examples/src/docs/early_exit.rs:early_exit}}
```

This is particularly useful if you want to short-circuit a costly computation when it's clear that the result is no longer relevant.

> Early exit will be triggered by the first value which returns an `Err`, therefore ordering is important.
>
> Be aware that nodes _after_ the node which prompts the exit will not receive data during the execution, and will miss
> any transient state (that which will be [cleaned](./cleaning.md)) which is cleared up by the time they are next
> updated.
