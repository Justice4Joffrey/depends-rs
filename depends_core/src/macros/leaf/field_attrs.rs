use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

use crate::macros::common::{unexpected_attribute, HASH};

pub enum LeafFieldAttr {
    Hash(Span),
}

impl Parse for LeafFieldAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            HASH => Ok(Self::Hash(ident.span())),
            unknown => Err(unexpected_attribute(unknown, ident.span())),
        }
    }
}
