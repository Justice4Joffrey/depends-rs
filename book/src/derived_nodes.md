# Derived Nodes


[Derived nodes](https://docs.rs/depends/latest/depends/struct.DerivedNode.html) calculate their internal value from other values in the graph.

Any struct which derives `Value` can be used as a Derived Node. In order to create a derived node, however, we'll need to define:

- [An Operation](./operations.md): How to transform other nodes and values in to the derived node's internal value.
- [Dependencies](./dependencies.md): The required values of the operation. Implicitly, dependencies specify _edges_ in the computation graph.

Let's look at these in more detail.
