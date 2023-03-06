use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

use crate::macros::common::{unexpected_attribute, CUSTOM_CLEAN, UNHASHABLE};

pub enum LeafStructAttr {
    Unhashable(Span),
    CustomClean(Span),
}

impl Parse for LeafStructAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            UNHASHABLE => Ok(Self::Unhashable(ident.span())),
            CUSTOM_CLEAN => Ok(Self::CustomClean(ident.span())),
            unknown => Err(unexpected_attribute(unknown, ident.span())),
        }
    }
}
