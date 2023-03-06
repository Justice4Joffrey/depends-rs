use proc_macro2::Span;

pub const HASH: &str = "hash";
pub const CUSTOM_CLEAN: &str = "custom_clean";
pub const UNHASHABLE: &str = "unhashable";

pub fn unexpected_attribute(attr: &str, span: Span) -> syn::Error {
    syn::Error::new(span, format!("Unexpected attribute \"{:?}\"", attr))
}

pub fn duplicate_attribute(span: Span) -> syn::Error {
    syn::Error::new(span, "Attribute specified more than once")
}
