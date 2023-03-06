use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    token::Eq,
    Ident, Type,
};

use crate::macros::common::{unexpected_attribute, CUSTOM_CLEAN, UNHASHABLE};

pub enum DependeeStructAttr {
    NodeName(Span, Ident),
    Dependencies(Span, Type),
    Unhashable(Span),
    CustomClean(Span),
}

impl Parse for DependeeStructAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "node_name" => {
                Ok({
                    input.parse::<Eq>()?;
                    Self::NodeName(ident.span(), input.parse()?)
                })
            }
            "dependencies" => {
                Ok({
                    input.parse::<Eq>()?;
                    Self::Dependencies(ident.span(), input.parse()?)
                })
            }
            UNHASHABLE => Ok(Self::Unhashable(ident.span())),
            CUSTOM_CLEAN => Ok(Self::CustomClean(ident.span())),
            unknown => Err(unexpected_attribute(unknown, ident.span())),
        }
    }
}
