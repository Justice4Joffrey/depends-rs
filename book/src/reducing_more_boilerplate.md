# Reducing More Boilerplate

Writing code to manually construct a dependency graph, connecting nodes to their dependencies, can quickly become tedious and error-prone, particularly for larger graphs. As shown in the example, each node and connection must be explicitly defined and linked, leading to a significant amount of boilerplate code.

```rust
{{#include ../../examples/src/docs/raising_the_stakes.rs:boilerplate_setup}}

...
```

## Graphviz Deserialization

To simplify this process, there's the [Graph](https://docs.rs/depends/0.9.0/depends/derives/derive.Graph.html) derive
macro. This takes a Graphviz definition of the graph, and generates the code to construct it.

```rust
{{#include ../../examples/src/docs/reducing_more_boilerplate.rs:graph_creator}}
```

The macro generates the type `MyDag<R>`, similarly to the previous example. This graph can be created by passing the
initial values for each of the nodes to the method attached to `DagCreator`.

```rust
{{#include ../../examples/src/docs/reducing_more_boilerplate.rs:graph_creator_use}}
```

## Send-ability

Another benefit of using the `Graph` derive macro is that it will safely implement `Send` with `unsafe`
for the given graph. This is a requirement for use in most async environments, such as [Tokio](https://tokio.rs/).

> Despite the fact that it uses `Rc` and `RefCell` internally, the `Graph` is safe to `Send` because it gives no access
> to the `Rc` types created, and moves all of them at once when being sent to another thread.

An example of this can be seen below:

```rust
{{#include ../../examples/src/docs/reducing_more_boilerplate.rs:send_graph}}
```
