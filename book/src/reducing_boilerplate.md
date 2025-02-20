# Reducing Boilerplate

Whilst it's common for dependency graph frameworks to use [Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
to combine multiple types in to a graph-like structure, Depends uses generic type-system trickery.
Specifically, [GATs](https://rust-lang.github.io/generic-associated-types-initiative/explainer/motivation.html).

Whilst this has low-level performance benefits, particularly for graphs with many edges, the cost is that types can become quite verbose.

Let's have a look at the type of the root node in the previous example:

```rust
{{#include ../../examples/src/docs/raising_the_stakes.rs:reducing_boilerplate}}
```

That's got a bit out of hand! We're experiencing the same issue that Futures have. By retaining the strict type information, the _code is now the type_. Every time we change the code, we change the type of the graph. This clearly presents a maintenance issue.

> See [Fasterthanlime](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)'s excellent in-depth article on futures for more on this particular topic.

## Impl Trait to the rescue

Thankfully, Rust has a solution for this: [impl Trait](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits).

Instead of tracking the concrete type of the root node, we only need to specify the behaviour we want it to exhibit.

Furthermore, by only explicitly storing the root and input nodes, we eliminate the need to keep track of all intermediate derived node types. This approach drastically reduces the amount of boilerplate code, simplifying our program and making it more maintainable, while still preserving the performance benefits and safety guarantees of Rust's type system.

The end result is we can create a graph to store this struct like so:

```rust
{{#include ../../examples/src/docs/raising_the_stakes.rs:impl_trait}}
```

Using that in practice then becomes:

```rust
{{#include ../../examples/src/docs/raising_the_stakes.rs:init_impl_trait}}
```

> Note we can't _create_ the graph inside one of its methods, as it must be able to _infer_ a generic parameter `R` from
> the arguments provided. A helper function
>
> `fn create_graph(...) -> Graph<impl Resolve<...>>`
>
> can be useful here.
