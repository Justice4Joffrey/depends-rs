use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, spanned::Spanned, Field, ItemStruct};

use super::parsed_attrs::ValueParsedAttrs;
use crate::macros::{
    model::{get_depends_attrs, FieldAttrs},
    value::ValueAttrModel,
    HashLogic,
};

pub fn derive_value(input: TokenStream) -> TokenStream {
    derive_value_inner(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn derive_value_inner(input: TokenStream) -> syn::Result<TokenStream> {
    let ItemStruct {
        ident,
        fields,
        generics,
        attrs,
        ..
    } = parse2::<ItemStruct>(input)?;

    let name = ident.to_string();
    let (custom_clean, hashing) = {
        let struct_attrs = get_depends_attrs(&attrs)?;
        let field_attrs: syn::Result<Vec<_>> = fields
            .iter()
            .map(|Field { attrs, ident, .. }| {
                Ok(FieldAttrs {
                    ident: ident
                        .as_ref()
                        .ok_or_else(|| {
                            syn::Error::new(
                                ident.span(),
                                "Only structs with named fields are supported.",
                            )
                        })?
                        .clone(),
                    field_attrs: get_depends_attrs(attrs)?,
                })
            })
            .collect();
        let field_attrs = field_attrs?;

        let model = ValueAttrModel {
            struct_attrs,
            field_attrs,
        };
        let parsed: ValueParsedAttrs = model.try_into()?;
        (
            parsed.custom_clean.unwrap_or(false),
            parsed.hashing.unwrap_or(HashLogic::Struct),
        )
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

    let hash_value_clause = hashing.to_tokens();

    Ok(quote! {
        impl #impl_generics ::depends::core::Named for #ident #ty_generics #where_clause {
            fn name() -> &'static str {
                #name
            }
        }

        impl #impl_generics ::depends::core::HashValue for #ident #ty_generics #where_clause {
            fn hash_value(&self, hasher: &mut impl ::std::hash::Hasher) -> ::depends::core::NodeHash {
                use ::std::hash::Hash;
                #hash_value_clause
            }
        }

        #clean_clause
    })
}

#[cfg(all(test, not(miri), not(target_os = "windows")))]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::macros::helpers::format_source;

    #[test]
    fn test_input() {
        let input: TokenStream = parse_quote! {
            #[diesel(some_arg)]
            pub struct Foo {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "value",
            format_source(derive_value(input).to_string().as_str())
        );
    }

    #[test]
    fn test_input_hashed() {
        let input = parse_quote! {
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                number: usize,
            }
        };
        assert_snapshot!(
            "value_hashed_attr",
            format_source(derive_value(input).to_string().as_str())
        );
    }

    #[test]
    fn test_input_unhashable() {
        let input = parse_quote! {
            #[depends(unhashable)]
            struct Foo<T> {
                bar: Vec<usize>,
                number: usize,
            }
        };
        assert_snapshot!(
            "value_unhashable",
            format_source(derive_value(input).to_string().as_str())
        );
    }

    #[test]
    fn test_input_generics_custom_clean() {
        let input = parse_quote! {
            #[depends(custom_clean)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "value_generics_custom_clean",
            format_source(derive_value(input).to_string().as_str())
        );
    }
}
