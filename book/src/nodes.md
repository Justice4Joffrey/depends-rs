# Nodes

In a graph, a node represents a discrete unit of data or computation. In the context of the Depends library, nodes hold and manage a [Value](./values.md) of a specific type. They can either be:

- [Input Nodes](./input_nodes.md): Nodes which derive their `Value`'s state from outside the graph.
- [Derived Nodes](./derived_nodes.md): Nodes which derive their `Value`'s state from other nodes they depend on.
