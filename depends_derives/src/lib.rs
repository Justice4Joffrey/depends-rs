use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Implement necessary traits for making this type a valid value in either
/// an `InputNode` or `DerivedNode`.
///
/// ## Hashing
///
/// By default, this will assume the type implements `Hash`. If it doesn't,
/// you must either mark a field you wish to use as a hash with
/// `#[depends(hash)]`, or mark the type itself as `#[depends(unhashable)]`.
///
/// > Note that marking a node as `unhashable` will cause any node with an edge
/// > to it to consider its dependencies dirty on every resolve. In the vast
/// > majority of cases you should hash a field instead.
///
/// ## Cleaning
///
/// By default, this will implement a no-op `Clean` implementation. This
/// means that nothing will be done to clean the node between resolves.
///
/// If you wish to implement `Clean` manually, you can do so by using the
/// `#[depends(custom_clean)]` attribute on the struct and providing your
/// implementation.
///
/// > Any transient state (such as 'the things which have changed since the
/// > last resolve') _must_ be cleared up in this method.
#[proc_macro_error]
#[proc_macro_derive(Value, attributes(depends))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    depends_core::derive_value(input.into()).into()
}

/// Mark this type as a set of dependencies for a `DerivedNode`.
///
/// > For a single dependency, see the `Dependency` type.
///
/// This will generate 2 types:
/// - `MyTypeDep`: A generic struct which can be constructed from _any_ set of
///   nodes who's output corresponds to the fields of the base type.
/// - `MyTypeRef<'_>`: A read-only reference to all of the fields of the generic
///   type above.
///
/// Note that you will not use the type annotated by this macro directly. It
/// is merely an instruction to generate the above types.
///
/// # Usage
/// ```ignore
/// # use depends_derives::Dependencies;
/// // Use this type to express the _types_ of output to depend on. It won't
/// // actually be _used_ at runtime. See below.
/// #[derive(Dependencies)]
/// struct TwoNumbers {
///    left: NumberValue,
///    right: NumberValue,
/// }
///
/// // Generates:
/// // A generic struct which can be constructed from _any_ nodes who's
/// // output is a [NumberValue].
/// // TwoNumbersDep<A, B> {
/// //   left: A,
/// //   right: B,
/// // }
/// // A read-only reference to all of the fields of the generic type above.
/// // TwoNumbersRef<'a> {
/// //   left: Ref<'a, NumberValue>,
/// //   right: Ref<'a, NumberValue>,
/// // }
/// ```
#[proc_macro_error]
#[proc_macro_derive(Dependencies)]
pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    depends_core::derive_dependencies(input.into()).into()
}

/// Mark this type as a an `Operation`. This implements `Named` for
/// debugging, which is a requirement to implement `UpdateDerived`.
///
/// # Usage
/// ```ignore
/// # use depends_core::execution::{SingleRef, TargetMut, UpdateDerived};
/// # use depends_core::execution::error::EarlyExit;
/// # use depends_derives::Operation;
/// #[derive(Operation)]
/// struct Square;
///
/// // Note [TwoNumbersRef] is generated by the [Dependencies] macro, and
/// // represents read-references to all of its fields.
/// impl UpdateDerived for Square {
///     // [SingleRef] is used for a single dependency.
///     // If you have more than one, you must use a struct which derives
///     // [Dependencies].
///     type Input<'a> = SingleRef<'a> where Self: 'a;
///     // [TargetMut] is short-hand for a mutable reference to the target.
///     // For this [Operation], the target is a [NumberValue].
///     type Target<'a> = TargetMut<'a, NumberValue> where Self: 'a;
///
///     fn update_derived(
///         value: SingleRef<'_>,
///         mut target: TargetMut<'_, NumberValue>,
///     ) -> Result<(), EarlyExit> {
///         target.value = input.value.pow(2);
///         // Returning [EarlyExit] will cause the graph to stop resolving.
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(Operation)]
pub fn derive_operation(input: TokenStream) -> TokenStream {
    depends_core::derive_operation(input.into()).into()
}

#[cfg(feature = "graphviz")]
/// Automatically generate graph construction code from a Graphviz graph
/// definition.
///
/// > Note that the ordering of edge definitions is important. For any given
/// > `Dependencies` type, edges must be defined in the Graphviz specification
/// > in the order they appear in the `struct`.
///
/// Like futures in Rust, the type of a given graph is dependent on the
/// code itself. Whilst two graphs can be thought of as 'Resolving to a
/// type T', the actual type of the graph itself depends on exactly which
/// nodes are constructed to produce that value.
///
/// This macro will also safely implement `Send` for the graph definition.
/// Since `Rc` types within the graph cannot be accessed, it is safe to
/// send the graph to another thread.
///
/// # Usage
///
/// Provide a DAG definition in the Graphviz format. Each node must have a
/// `label` attribute which corresponds to the name of the node type in
/// scope.
///
/// For nodes which have multiple dependencies, a `class` attribute must be
/// provided which corresponds to the `Dependencies` type.
/// ```ignore
/// # use depends_core::execution::HashSetVisitor;
/// # use depends_derives::Graph;
/// #[derive(Graph)]
/// #[depends(
///   digraph Dag {
///     node_0 [label="NumberValue"];
///     node_1 [label="NumberValue"];
///     node_0 -> node_1 [label="Square"];
///     node_2 [label="NumberValue"];
///     node_3 [label="NumberValue"];
///     node_1 -> node_3 [label="Add", class="TwoNumbersDep"];
///     node_2 -> node_3 [label="Add", class="TwoNumbersDep"];
///     node_4 [label="NumberValue"];
///     node_3 -> node_4 [label="Square"];
///   }
/// )]
/// struct MyGraphBuilder;
///
/// // create a graph with some initial values.
/// let graph = MyGraphBuilder::create_dag(
///     NumberValue::new(7),
///     NumberValue::new(6),
///     NumberValue::default(),
///     NumberValue::default(),
///     NumberValue::default(),
/// );
///
/// // methods to update input nodes are generated.
/// graph.update_node_0(70).unwrap();
///
/// let mut visitor = HashSetVisitor::new();
/// graph.resolve(&mut visitor).unwrap();
/// ```
#[proc_macro_error]
#[proc_macro_derive(Graph, attributes(depends))]
pub fn graph(input: TokenStream) -> TokenStream {
    depends_core::derive_graph(input.into()).into()
}
