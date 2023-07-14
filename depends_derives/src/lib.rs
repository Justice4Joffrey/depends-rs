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
/// Note that marking a node as `unhashable` will cause any node with an edge
/// to it to consider its dependencies dirty on every resolve.
///
/// ## Cleaning
///
/// By default, this will implement a no-op `Clean` implementation. This
/// means that nothing will be done to clean the node between resolves.
///
/// If you wish to implement `Clean` manually, you can do so by using the
/// `#[depends(custom_clean)]` attribute on the struct and providing your
/// implementation.
#[proc_macro_error]
#[proc_macro_derive(Value, attributes(depends))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_value(input.into()).into()
}

/// Mark this type as a set of dependencies for a `DerivedNode`.
///
/// This will generate 2 types:
/// - `MyTypeDep`: A generic struct which can be constructed from _any_ set of
///   nodes who's output corresponds to the fields of the base type.
/// - `MyTypeRef<'_>`: A read-only reference to all of the fields of the generic
///   type above.
///
/// Note that you will not use the type annotated by this macro directly. It
/// is merely an instruction to generate the above types.
#[proc_macro_error]
#[proc_macro_derive(Dependencies)]
pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_dependencies(input.into()).into()
}

/// Mark this type as a an `Operation`. This implements `Named` for
/// debugging, which is a requirement to implement `UpdateDerived`.
#[proc_macro_error]
#[proc_macro_derive(Operation)]
pub fn derive_operation(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_operation(input.into()).into()
}
