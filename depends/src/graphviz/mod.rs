use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, HashSet},
    hash::BuildHasher,
};

use crate::{Identifiable, Visitor};

#[derive(Debug)]
struct Node {
    id: usize,
    name: &'static str,
    edges: Vec<usize>,
    operation: Option<&'static str>,
    dependency: Option<&'static str>,
}

impl Node {
    fn node_identifier(&self) -> String {
        format!("node_{}", self.id)
    }
}

/// A [Visitor] which builds a `Graphviz` representation of a given graph.
///
/// ```
/// # use std::{cell::Ref, collections::HashSet, hash::Hash, rc::Rc};
/// # use depends::{
/// #     derives::{Operation, Value},
/// #     error::{EarlyExit, ResolveResult},
/// #     DependencyEdge, DepRef2, DepRef3, Dependencies2, Dependencies3, Dependency, DerivedNode, HashValue,
/// #     InputNode, NodeHash, Resolve, DepRef, UpdateDerived, UpdateInput,
/// # };
/// # pub trait NumberLike {
/// #     fn number_value(&self) -> i32;
/// # }
/// # #[derive(Value, Default, Hash)]
/// # pub struct NumberValueI32 {
/// #     pub value: i32,
/// # }
/// # impl NumberValueI32 {
/// #     pub fn new(value: i32) -> Self {
/// #         Self { value }
/// #     }
/// # }
/// #
/// # impl UpdateInput for NumberValueI32 {
/// #     type Update = i32;
/// #
/// #     fn update_mut(&mut self, update: Self::Update) {
/// #         // Implementing this trait will provide a way for code outside of this graph to
/// #         // change its internal state. This is just a simple replace for now.
/// #         self.value = update;
/// #     }
/// # }
/// #
/// # #[derive(Value, Default, Hash)]
/// # pub struct NumberValueI8 {
/// #     pub value: i8,
/// # }
/// #
/// # impl NumberValueI8 {
/// #     pub fn new(value: i8) -> Self {
/// #         Self { value }
/// #     }
/// # }
/// #
/// # impl NumberLike for NumberValueI8 {
/// #     fn number_value(&self) -> i32 {
/// #         self.value as i32
/// #     }
/// # }
/// # impl NumberLike for NumberValueI32 {
/// #     fn number_value(&self) -> i32 {
/// #         self.value
/// #     }
/// # }
/// #
/// # impl UpdateInput for NumberValueI8 {
/// #     type Update = i8;
/// #
/// #     fn update_mut(&mut self, update: Self::Update) {
/// #         // Implementing this trait will provide a way for code outside of this graph to
/// #         // change its internal state. This is just a simple replace for now.
/// #         self.value = update;
/// #     }
/// # }
/// # #[derive(Operation)]
/// # pub struct Sum;
/// #
/// # impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Sum> for NumberValueI32 {
/// #     fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
/// #         self.value = value.0.data().number_value() + value.1.data().number_value();
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Sum>
/// # for NumberValueI32
/// # {
/// #     fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
/// #         self.value = value.0.data().number_value() + value.1.data().number_value() + value.2.data().number_value();
/// #         Ok(())
/// #     }
/// # }
/// # #[derive(Operation)]
/// # pub struct Square;
/// #
/// # impl<A: NumberLike> UpdateDerived<DepRef<'_, A>, Square> for NumberValueI32 {
/// #     fn update(&mut self, value: DepRef<'_, A>) -> Result<(), EarlyExit> {
/// #         self.value = value.data().number_value().pow(2);
/// #         Ok(())
/// #     }
/// # }
/// # #[derive(Operation)]
/// # pub struct Multiply;
/// #
/// # impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Multiply> for NumberValueI32 {
/// #     fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
/// #         self.value = value.0.data().number_value() * value.1.data().number_value();
/// #         Ok(())
/// #     }
/// # }
/// use depends::graphviz::GraphvizVisitor;
/// use depends::NodeState;
///
/// let a = InputNode::new(NumberValueI32::new(1));
/// let b = InputNode::new(NumberValueI32::new(2));
/// // Note that we can combine different types in the same graph.
/// let c = InputNode::new(NumberValueI8::new(3));
/// let d = InputNode::new(NumberValueI32::new(4));
/// let e = InputNode::new(NumberValueI32::new(5));
///
/// // Now for some derived nodes. We can't update these from outside of the
/// // graph. They are updated when their dependencies change.
///
/// // Compose a graph.
/// let c_squared = DerivedNode::new(
///     Dependency::new(Rc::clone(&c)),
///     Square,
///     NumberValueI32::default(),
/// );
/// // Sum of a and b
/// let a_plus_b = DerivedNode::new(
///     Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
///     Sum,
///     NumberValueI32::default(),
/// );
///
/// // Product of d and e
/// let d_times_e = DerivedNode::new(
///     Dependencies2::new(Rc::clone(&d), Rc::clone(&e)),
///     Multiply,
///     NumberValueI32::default(),
/// );
/// // Create another edge to node a
/// let a_plus_d_times_e = DerivedNode::new(
///     Dependencies2::new(Rc::clone(&a), Rc::clone(&d_times_e)),
///     Sum,
///     NumberValueI32::default(),
/// );
///
/// // Finally, the sum of all of the above
/// let answer = DerivedNode::new(
///     Dependencies3::new(Rc::clone(&c_squared), Rc::clone(&a_plus_b), Rc::clone(&a_plus_d_times_e)),
///     Sum,
///     NumberValueI32::default(),
/// );
///
/// let mut visitor = GraphvizVisitor::new();
///
/// // Resolve the graph with this visitor.
/// // Be sure NOT to use `resolve_root`, as this will clear the visitor's state.
/// answer.resolve(&mut visitor).unwrap();
///
/// println!("{}", visitor.render().unwrap());
/// // A Graphviz representation is now available on the visitor!
/// assert_eq!(
///     visitor.render().unwrap(),
///     r#"
/// digraph Dag {
///   node_0 [label="NumberValueI32"];
///   node_1 [label="NumberValueI32"];
///   node_2 [label="NumberValueI8"];
///   node_3 [label="NumberValueI32"];
///   node_4 [label="NumberValueI32"];
///   node_5 [label="NumberValueI32"];
///   node_2 -> node_5 [label="Square"];
///   node_6 [label="NumberValueI32"];
///   node_0 -> node_6 [label="Sum", class="Dependencies2"];
///   node_1 -> node_6 [label="Sum", class="Dependencies2"];
///   node_7 [label="NumberValueI32"];
///   node_3 -> node_7 [label="Multiply", class="Dependencies2"];
///   node_4 -> node_7 [label="Multiply", class="Dependencies2"];
///   node_8 [label="NumberValueI32"];
///   node_0 -> node_8 [label="Sum", class="Dependencies2"];
///   node_7 -> node_8 [label="Sum", class="Dependencies2"];
///   node_9 [label="NumberValueI32"];
///   node_5 -> node_9 [label="Sum", class="Dependencies3"];
///   node_6 -> node_9 [label="Sum", class="Dependencies3"];
///   node_8 -> node_9 [label="Sum", class="Dependencies3"];
/// }
/// "#
///     .trim()
/// );
/// ```
#[derive(Debug, Default)]
pub struct GraphvizVisitor {
    visitor: HashSet<usize>,
    nodes: BTreeMap<usize, Node>,
    stack: Vec<usize>,
}
// TODO: you need to name the actual dependency type, as it could be custom
//  or like Dependencies4 etc.

impl GraphvizVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render the visited graph to Graphviz DOT format. Returns [Option::None]
    /// if no graph has been visited.
    pub fn render(&self) -> Option<String> {
        if self.nodes.is_empty() {
            None
        } else {
            let mut lines = Vec::new();
            lines.push(String::from("digraph Dag {"));
            self.nodes.values().for_each(|n| {
                lines.push(format!("  {} [label=\"{}\"];", n.node_identifier(), n.name));
                // TODO: it would be nice to make this type-system enforced
                //  at the moment, it's not guaranteed by the type system
                //  that nodes.len() == 0 iff operation.is_none()
                //  this would likely be done by splitting `touch` in to
                //  `touch_input` and `touch_derived`
                if let Some(op) = n.operation {
                    let class = n
                        .dependency
                        .map(|d| format!(", class=\"{}\"", d))
                        .unwrap_or_default();
                    let edge_label = format!("[label=\"{}\"{}]", op, class);
                    n.edges.iter().for_each(|c| {
                        lines.push(format!(
                            "  {} -> {} {};",
                            self.nodes[c].node_identifier(),
                            n.node_identifier(),
                            edge_label
                        ));
                    });
                }
            });
            lines.push(String::from("}"));
            Some(lines.join("\n"))
        }
    }
}

impl Visitor for GraphvizVisitor {
    type Hasher = DefaultHasher;

    fn visit<N>(&mut self, node: &N) -> bool
    where
        N: Identifiable,
    {
        self.visitor.visit(node)
    }

    fn clear(&mut self) {
        self.visitor.clear();
        self.nodes.clear();
    }

    fn touch<N>(&mut self, node: &N, operation: Option<&'static str>)
    where
        N: Identifiable,
    {
        self.stack.push(node.id());
        self.nodes.entry(node.id()).or_insert_with(|| {
            Node {
                id: node.id(),
                name: N::name(),
                edges: Vec::default(),
                operation,
                dependency: None,
            }
        });
    }

    fn touch_dependency_group(&mut self, dep: &'static str) {
        let last = self.stack.last().unwrap();
        if let Some(n) = self.nodes.get_mut(last) {
            n.dependency = Some(dep);
        }
    }

    fn leave<N>(&mut self, node: &N)
    where
        N: Identifiable,
    {
        let last = self.stack.pop().unwrap();
        assert_eq!(last, node.id());
        if let Some(parent) = self.stack.last() {
            if let Some(n) = self.nodes.get_mut(parent) {
                n.edges.push(last)
            }
        }
    }

    fn hasher(&self) -> Self::Hasher {
        self.visitor.hasher().build_hasher()
    }
}
