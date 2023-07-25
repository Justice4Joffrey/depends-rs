# Dependencies

In most real-world applications, you'll have nodes that depend on multiple other nodes. Depends allows you to define complex dependencies using the [Dependencies](https://docs.rs/depends/latest/depends/derives/derive.Dependencies.html) derive macro.

Here's an example where an operation depends on two numbers and multiplies them:

```rust
{{#include ../../examples/src/docs/multiple_dependencies.rs:9::}}
```

By marking `TwoNumbers` with `#[derive(Dependencies)]`, you tell Depends that `TwoNumbers` is a set of dependencies. The `Dependencies` derive macro generates two new types for you: `TwoNumbersDep` and `TwoNumbersRef`.

Here's a table that shows the equivalent types for single and multiple dependencies:

|                      | Dependency\<A\>              | TwoNumbers                     |
|----------------------|------------------------------|--------------------------------|
| **Constructor**      | `Dependency::<A>::new(a: A)` | `TwoNumbers::init(a: A, b: B)` |
| **Initialised Type** | `Dependency<A>`              | `TwoNumbersDep<A, B>`          |
| **Reference Type**   | `SingleRef<'a, A>`           | `TwoNumbersRef<'a>`            |

This table describes the equivalent constructors, initialised types, and reference types for single (`Dependency<A>`) and multiple dependencies (`TwoNumbers`).
You can use `TwoNumbersRef<'a>` with any operation that requires two `NumberValue`s. This allows you to build complex, flexible dependency graphs.

## Checking Specific Dependency State

There are situations where it's useful to know which specific dependencies have caused `update_mut` to be called. For this reason, the `is_dirty` method is available on each dependency reference.

```rust
{{#include ../../examples/src/docs/checking_node_state_directly.rs:is_dirty}}
```

> The most common is example is time. It's expected that no node uses methods such as `Utc::now()`, as this is a side-effect which will result in non-deterministic behaviour.
> 
> Instead, you should 'set' the time for the graph by providing it as an [Input Node](./input_nodes.md) and creating edges to the nodes which require it.