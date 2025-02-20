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

/// Mark this type as a an `Operation`. This implements `Named` for
/// debugging, which is a requirement to implement `UpdateDerived`.
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
#[proc_macro_error]
#[proc_macro_derive(Graph, attributes(depends))]
pub fn graph(input: TokenStream) -> TokenStream {
    depends_core::derive_graph(input.into()).into()
}
