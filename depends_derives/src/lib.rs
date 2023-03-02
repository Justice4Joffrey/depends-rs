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
#[proc_macro_attribute]
pub fn dependencies(args: TokenStream, input: TokenStream) -> TokenStream {
    depends_core::macros::dependencies_attr(args.into(), input.into()).into()
}
