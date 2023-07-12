use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, HashSet},
    hash::BuildHasher,
};

use crate::core::{Identifiable, Visitor};

#[derive(Debug)]
struct Node {
    id: usize,
    name: &'static str,
    edges: BTreeSet<usize>,
}

/// A [Visitor] which builds a `Graphviz` representation of a given graph.
///
/// ```
/// # use std::{collections::HashSet, hash::Hash, rc::Rc};
/// #
/// # use depends::{
/// #     core::{
/// #         Dependency, HashValue, Resolve, UpdateInput,
/// #         NodeHash, InputNode, DerivedNode, TargetMut,
/// #         error::ResolveResult
/// #     },
/// #     derives::{Dependencies, Value},
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
/// # fn add(
/// #     TwoNumbersRef { left, right }: TwoNumbersRef<'_>,
/// #     mut target: TargetMut<'_, NumberValue>,
/// # ) -> ResolveResult<()> {
/// #     target.value = left.value + right.value;
/// #     Ok(())
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
/// let right = InputNode::new(NumberValue::default());
/// let two_numbers = TwoNumbers::init(left, right);
/// let sum = DerivedNode::new(two_numbers, add, NumberValue::default());
///
/// let mut visitor = GraphvizVisitor::new();
///
/// // Resolve the graph with this visitor.
/// // Be sure NOT to use `resolve_root`, as this will clear the visitor's state.
/// sum.resolve(&mut visitor).unwrap();
///
/// // A Graphviz representation is now available on the visitor!
/// assert_eq!(
///     visitor.render().unwrap(),
///     r#"
/// digraph G {
///   0[label="NumberValue"];
///   1[label="NumberValue"];
///   2[label="NumberValue"];
///   0 -> 2;
///   1 -> 2;
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
            lines.push(String::from("digraph G {"));
            self.nodes.values().for_each(|n| {
                lines.push(format!("  {}[label=\"{}\"];", n.id, n.name));
                n.edges.iter().for_each(|c| {
                    lines.push(format!("  {} -> {};", c, n.id));
                });
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

    fn touch<N>(&mut self, node: &N)
    where
        N: Identifiable,
    {
        self.stack.push(node.id());
        self.nodes.entry(node.id()).or_insert_with(|| {
            Node {
                id: node.id(),
                name: N::name(),
                edges: BTreeSet::default(),
            }
        });
    }

    fn leave<N>(&mut self, node: &N)
    where
        N: Identifiable,
    {
        let last = self.stack.pop().unwrap();
        assert_eq!(last, node.id());
        if let Some(parent) = self.stack.last() {
            self.nodes.get_mut(parent).map(|n| n.edges.insert(last));
        }
    }

    fn hasher(&self) -> Self::Hasher {
        self.visitor.hasher().build_hasher()
    }
}
