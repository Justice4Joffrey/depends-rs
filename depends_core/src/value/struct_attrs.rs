use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

use crate::common::{unexpected_attribute, CUSTOM_CLEAN, UNHASHABLE};

pub enum ValueStructAttr {
    Unhashable(Span),
    CustomClean(Span),
}

impl Parse for ValueStructAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            UNHASHABLE => Ok(Self::Unhashable(ident.span())),
            CUSTOM_CLEAN => Ok(Self::CustomClean(ident.span())),
            unknown => Err(unexpected_attribute(unknown, ident.span())),
        }
    }
}
