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
/// # use std::{collections::HashSet, hash::Hash, rc::Rc};
/// #
/// # use depends::{
/// #     Dependency, HashValue, Resolve, UpdateDerived, UpdateInput,
/// #     NodeHash, InputNode, DerivedNode, TargetMut, SingleRef,
/// #     error::{EarlyExit, ResolveResult},
/// #     derives::{Dependencies, Value, Operation},
/// # };
/// #
/// # #[derive(Value, Default, Hash, Debug)]
/// # pub struct NumberValue {
/// #     value: i32,
/// # }
/// #
/// # impl UpdateInput for NumberValue {
/// #     type Update = i32;
/// #
/// #     fn update_mut(&mut self, input: Self::Update) {
/// #         self.value = input;
/// #     }
/// # }
/// # #[derive(Operation)]
/// # struct Add;
/// #
/// # impl UpdateDerived for Add {
/// #     type Input<'a> = TwoNumbersRef<'a> where Self: 'a;
/// #     type Target<'a> = TargetMut<'a, NumberValue> where Self: 'a;
/// #
/// #     fn update_derived(
/// #         TwoNumbersRef { left, right }: TwoNumbersRef<'_>,
/// #         mut target: TargetMut<'_, NumberValue>,
/// #     ) -> Result<(), EarlyExit> {
/// #         target.value = left.value + right.value;
/// #         Ok(())
/// #     }
/// # }
/// #
/// # #[derive(Operation)]
/// # struct Square;
/// #
/// # impl UpdateDerived for Square{
/// #     type Input<'a> = SingleRef<'a, NumberValue> where Self: 'a;
/// #     type Target<'a> = TargetMut<'a, NumberValue> where Self: 'a;
/// #
/// #     fn update_derived(
/// #         input: Self::Input<'_>,
/// #         mut target: TargetMut<'_, NumberValue>,
/// #     ) -> Result<(), EarlyExit> {
/// #         target.value = input.value.pow(2);
/// #         Ok(())
/// #     }
/// # }
/// #
/// # #[derive(Dependencies)]
/// # pub struct TwoNumbers {
/// #     left: NumberValue,
/// #     right: NumberValue,
/// # }
/// #
/// use depends::graphviz::GraphvizVisitor;
///
/// // Compose a graph.
/// let left = InputNode::new(NumberValue::default());
/// let left_squared = DerivedNode::new(
///     Dependency::new(Rc::clone(&left)),
///     Square,
///     NumberValue::default(),
/// );
/// let right = InputNode::new(NumberValue::default());
/// let two_numbers = TwoNumbers::init(left_squared, right);
/// let sum = DerivedNode::new(two_numbers, Add, NumberValue::default());
/// let sum_squared = DerivedNode::new(Dependency::new(sum), Square, NumberValue::default());
///
/// let mut visitor = GraphvizVisitor::new();
///
/// // Resolve the graph with this visitor.
/// // Be sure NOT to use `resolve_root`, as this will clear the visitor's state.
/// sum_squared.resolve(&mut visitor).unwrap();
///
/// println!("{}", visitor.render().unwrap());
/// // A Graphviz representation is now available on the visitor!
/// assert_eq!(
///     visitor.render().unwrap(),
///     r#"
/// digraph Dag {
///   node_0 [label="NumberValue"];
///   node_1 [label="NumberValue"];
///   node_0 -> node_1 [label="Square"];
///   node_2 [label="NumberValue"];
///   node_3 [label="NumberValue"];
///   node_1 -> node_3 [label="Add", class="TwoNumbersDep"];
///   node_2 -> node_3 [label="Add", class="TwoNumbersDep"];
///   node_4 [label="NumberValue"];
///   node_3 -> node_4 [label="Square"];
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
                        .unwrap_or_else(String::new);
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
