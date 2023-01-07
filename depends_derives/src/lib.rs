use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(Leaf, attributes(depends))]
pub fn derive_leaf(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_leaf(input.into()).into()
}

#[proc_macro_error]
#[proc_macro_derive(Dependee, attributes(depends))]
pub fn derive_dependee(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_dependee(input.into()).into()
}

#[proc_macro_error]
#[proc_macro_derive(Dependencies, attributes(depends))]
pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    depends_core::macros::derive_dependencies(input.into()).into()
}
