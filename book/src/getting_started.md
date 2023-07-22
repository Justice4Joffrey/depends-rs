# Getting Started

## Directed Acyclic Graphs

Central to the Depends crate is the concept of a _directed acyclic graph_ (DAG). A DAG is a tree-like structure where
each node has zero or more _parents_ and zero or more _children_. A node with no parents is called a _root_ node, and

## Defining a Node

A node in a dependency graph must:

- Provide a hash value for its state (or indicate it _can't_ be hashed).
- Declare how to clean any transient state.

## Value

The easiest way to make a struct compatible with the Depends dependency graph is with the `#[derive(Value)]`
attribute.

```rust
use depends::derives::Value;

#[derive(Value, Hash)]
struct YourStruct {
    // ... your fields go here
}
```

The default behaviour of this type is to:

- Hash all fields.
- Not clean any transient state.
