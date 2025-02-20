//! # Depends
//!
//! A library for ergonomic, performant, incremental computation between
//! arbitrary types.
//!
//! For more information, see:
//! - [Getting Started Guide](https://justice4joffrey.github.io/depends-rs)
//! - [Examples](https://github.com/Justice4Joffrey/depends-rs/tree/master/examples)
//! - [Benchmarks](https://github.com/Justice4Joffrey/depends-rs/tree/master/benches)
//!
//! ## Motivation
//!
//! Many applications which respond to changes from multiple input sources
//! benefit from the use of dependency graphs as code structure. By breaking
//! complex states down in to small pieces of testable, composable logic,
//! scaling and maintaining applications becomes much easier over time.
//! Additionally, incremental computation allows results of previous
//! calculations to be reused where possible, improving overall efficiency
//! and performance.
//!
//! Depends aims to present the smallest possible API surface for building
//! minimal runtime-overhead dependency graphs in Rust, whilst leveraging
//! the compile-time guarantees of the type-system.
//!
//! ```
//! # use std::cell::{Ref, RefMut};
//! # use std::{
//! #     hash::{Hash, Hasher},
//! # };
//! # use std::collections::HashSet;
//! # use std::rc::Rc;
//! #
//! # use depends::error::{EarlyExit, ResolveResult};
//! # use depends::{Dependencies2, DependencyEdge, DepRef2, HashSetVisitor, NodeState, DepRef};
//! # use depends::{
//! #     DerivedNode, InputNode, Resolve, UpdateDerived, UpdateInput,
//! #     derives::{Operation, Value},
//! # };
//! # #[derive(Operation)]
//! # struct Multiply;
//! # impl UpdateDerived<DepRef2<'_, i64, i32>, Multiply> for i64 {
//! #    fn update(
//! #        &mut self,
//! #        deps: DepRef2<'_, i64, i32>,
//! #    ) -> Result<(), EarlyExit> {
//! #        *self = deps.0.data().value() * (*deps.1.data().value()as i64);
//! #        Ok(())
//! #    }
//! # }
//! // Below are input nodes, which are nodes which take new values from
//! // outside the graph.
//! // It's not common to use primitives, but they make for a simple example.
//! let a = InputNode::new(7_i64);
//! let b = InputNode::new(6_i32);
//!
//! // Derived nodes take their value from other nodes (either input or
//! // derived). Note that we can combine _any_ type of node, providing
//! // they're compatible with the dependencies (`TwoNumbers`) and operation
//! // (`Multiply`).
//! let c = DerivedNode::new(
//!     Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
//!     Multiply,
//!     0_i64,
//! );
//!
//! // A visitor tracks which nodes have been visited during a resolve.
//! let mut visitor = HashSetVisitor::new();
//!
//! // Resolve the graph!
//! // `resolve_root` will clear the visitor before returning, readying it
//! // for the next resolution.
//! // This can fail if there are cycles in the graph or an existing read
//! // reference is being held.
//! assert_eq!(c.resolve_root(&mut visitor).unwrap().value().clone(), 42);
//!
//! // Nodes which have an edge to dependencies which are updated between
//! // resolves will recalculate their state on-demand. Others will return
//! // a cached value. This is known as incremental computation, and can
//! // vastly improve performance of complex calculations.
//! a.update(70).unwrap();
//!
//! // Any dependent values will be updated next time the graph is resolved.
//! assert_eq!(c.resolve_root(&mut visitor).unwrap().value().clone(), 420);
//! ```

mod execution;
pub use execution::*;

pub mod derives {
    //! Derive macros for `depends`.
    pub use depends_derives::*;
}

/// Visualisation tool for graphs.
#[cfg(feature = "graphviz")]
pub mod graphviz;
