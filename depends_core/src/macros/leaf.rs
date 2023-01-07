use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2, Data, DeriveInput, LitBool, Token,
};

use super::attrs::get_depends_attrs;

enum LeafAttr {
    CustomClean(Span, LitBool),
}

impl Parse for LeafAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        match ident.to_string().as_str() {
            "custom_clean" => Ok(LeafAttr::CustomClean(ident.span(), input.parse()?)),
            unknown => {
                Err(syn::Error::new(
                    ident.span(),
                    format!("Unexpected attribute {:?}", unknown),
                ))
            }
        }
    }
}

struct LeafParsedAttrs {
    custom_clean: Option<bool>,
}

impl TryFrom<Vec<LeafAttr>> for LeafParsedAttrs {
    type Error = syn::Error;

    fn try_from(value: Vec<LeafAttr>) -> Result<Self, Self::Error> {
        let mut this = Self { custom_clean: None };
        for v in value.into_iter() {
            match v {
                LeafAttr::CustomClean(s, value) => {
                    if this.custom_clean.is_none() {
                        this.custom_clean = Some(value.value);
                    } else {
                        return Err(syn::Error::new(s, "attribute specified twice"));
                    }
                }
            }
        }
        Ok(this)
    }
}

pub fn derive_leaf(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse2::<DeriveInput>(input).unwrap();
    let name = ident.to_string();
    let custom_clean = {
        let attrs = get_depends_attrs(attrs)
            .expect("attributes must be in the form \"#[depends(... = ..., ...)]\"");
        let parsed: LeafParsedAttrs = attrs.try_into().unwrap();
        parsed.custom_clean.unwrap_or(false)
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let clean_clause = if custom_clean {
        TokenStream::new()
    } else {
        quote! {
            impl #ty_generics ::depends::core::Clean for #ident #ty_generics #where_clause {
                fn clean(&mut self) {}
            }
        }
    };

    if let Data::Struct(_) = data {
        quote! {
            impl #impl_generics ::depends::core::Named for #ident #ty_generics #where_clause {
                fn name() -> &'static str {
                    #name
                }
            }

            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn into_leaf(self) -> ::depends::core::LeafNodeRc<Self> {
                    ::depends::core::LeafNode::new(self)
                }
            }

            #clean_clause
        }
    } else {
        panic!("This macro can only be derived for structs.");
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::macros::helpers::format_source;

    #[test]
    fn test_leaf() {
        let input: TokenStream = parse_quote! {
            #[diesel(some_arg)]
            pub struct Foo {
                #[depends(hi)]
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "leaf",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    fn test_leaf_generics() {
        let input = parse_quote! {
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "leaf_generics",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    fn test_leaf_generics_custom_clean() {
        let input = parse_quote! {
            #[depends(custom_clean = true)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "leaf_generics_custom_clean",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_leaf_on_enum() {
        let input = parse_quote! {
            enum Foo {
                Thing(usize)
            }
        };

        derive_leaf(input);
    }
}
