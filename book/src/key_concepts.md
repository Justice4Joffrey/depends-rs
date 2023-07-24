# Key Concepts

Depends is built around several key concepts from graph theory and functional programming. Understanding these concepts will help you make the most of the library.

## Directed Acyclic Graph (DAG)

A [Directed Acyclic Graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph), or DAG, is a concept from graph theory. It's a collection of nodes and directed edges, where each edge has an orientation (it goes from one node to another), and there are no cycles (you cannot start at a node, follow the directed edges, and return to the starting node).

In Depends, the computation graph you define is a DAG. Each node in the graph represents a piece of data, and each edge represents a transformation of data from one node to another.

## Input Nodes and Derived Nodes

The nodes in the Depends DAG are of two types:

- **Input Nodes:** These are the leaves of the graph. They represent data inputs that come from outside the graph. These are often values provided by the user or from some external source.
- **Derived Nodes:** These nodes represent data that's computed within the graph. A derived node's value is a function of the values of its dependencies (the nodes that have edges leading to it).

<p align="center">
  <img src="./assets/visualisation.svg" />
</p>

The `Root` node is a special derived node where the graph's output is read from. Graphs should only ever be resolved from the root node.

## Purity and Determinism

A function is said to be pure if it always produces the same output when given the same input, and it has no side effects. That is, it doesn't alter any state outside the function or depend on any state that could change between function calls.

In Depends, it's essential that each transformation is a pure and deterministic function of its dependencies. This is what allows Depends to safely perform incremental computation. When the dependencies of a node haven't changed, Depends knows it doesn't need to re-compute that node. This guarantee of purity is a key to Depends's efficiency.
