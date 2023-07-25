# Overview

Depends is a powerful Rust library for efficient, flexible incremental computation between arbitrary data.

The goal of the library is to provide the _smallest_ possible API to produce high-performance, reliable, declarative
dependency graph structures.

## Key Features

Depends is:

- **Efficient:** Designed to only recompute data affected by changes, significantly increasing performance.
- **Flexible:** Easily establish dependencies on any arbitrary Rust types.
- **Declarative:** Serialize run-time graphs to/from Graphviz specifications.
- **Testable:** Build complex applications by composing small, testable units of logic.

This library is built with versatility in mind, ideal for domains where data from a variety of inputs frequently changes and
computations need to be updated accordingly.
