# Operations

An [Operation](https://docs.rs/depends/latest/depends/derives/derive.Operation.html) is a marker struct which is used to specify a way of transforming a [Derived Node](./derived_nodes.md) with a set of dependencies.

In Depends, you define an Operation with `#[derive(Operation)]`. For these types, you can now implement [UpdateDerived](https://docs.rs/depends/latest/depends/trait.UpdateDerived.html) to specify how to transform data in to the target.

Here is an example of an Operation that squares a number:

```rust
{{#include ../../examples/src/docs/simple_value.rs:some_number}}

{{#include ../../examples/src/docs/simple_value.rs:square}}
```

Above, we're expressing that given a `DepRef` (single dependency read-reference) to a node holding `SomeNumber`, we can pass the `Square` operation to transform the value of a [Derived Node](./derived_nodes.md) holding `SomeNumber`.

The `update` method describes how to use the dependencies (the input) to update the internal value of the derived node.

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
