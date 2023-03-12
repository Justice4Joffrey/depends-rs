use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Data, DeriveInput, Field};

use super::parsed_attrs::LeafParsedAttrs;
use crate::macros::{
    leaf::LeafAttrModel,
    model::{get_depends_attrs, FieldAttrs},
    HashLogic,
};

pub fn derive_leaf(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse2::<DeriveInput>(input).unwrap();

    let data = if let Data::Struct(data) = data {
        data
    } else {
        panic!("This macro can only be derived for structs.");
    };

    let name = ident.to_string();
    let (custom_clean, hashing) = {
        let struct_attrs = get_depends_attrs(&attrs);
        let field_attrs: Vec<_> = data
            .fields
            .iter()
            .map(|Field { attrs, ident, .. }| {
                FieldAttrs {
                    ident: ident
                        .as_ref()
                        .expect("Only structs with named fields are supported.")
                        .clone(),
                    field_attrs: get_depends_attrs(attrs),
                }
            })
            .collect();

        let model = LeafAttrModel {
            struct_attrs,
            field_attrs,
        };
        let parsed: LeafParsedAttrs = model.try_into().unwrap();
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

    quote! {
        impl #impl_generics ::depends::core::Named for #ident #ty_generics #where_clause {
            fn name() -> &'static str {
                #name
            }
        }

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn into_leaf(self) -> ::std::rc::Rc<::depends::core::LeafNode<Self>> {
                ::depends::core::LeafNode::new(self)
            }
        }

        impl #impl_generics ::depends::core::HashValue for #ident #ty_generics #where_clause {
            fn hash_value(&self, hasher: &mut impl ::std::hash::Hasher) -> ::depends::core::NodeHash {
                #hash_value_clause
            }
        }

        #clean_clause
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::macros::helpers::format_source;

    #[test]
    #[ignore]
    fn test_leaf() {
        let input: TokenStream = parse_quote! {
            #[diesel(some_arg)]
            pub struct Foo {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "leaf",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
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
    #[ignore]
    fn test_leaf_hashed() {
        let input = parse_quote! {
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                number: usize,
            }
        };
        assert_snapshot!(
            "leaf_hashed_attr",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_leaf_unhashable() {
        let input = parse_quote! {
            #[depends(unhashable)]
            struct Foo<T> {
                bar: Vec<usize>,
                number: usize,
            }
        };
        assert_snapshot!(
            "leaf_unhashable",
            format_source(derive_leaf(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_leaf_generics_custom_clean() {
        let input = parse_quote! {
            #[depends(custom_clean)]
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
    fn test_leaf_multi_hash_attr_args() {
        let input = parse_quote! {
            #[depends(unhashable)]
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                number: usize,
            }
        };
        derive_leaf(input);
    }

    #[test]
    #[should_panic]
    fn test_leaf_multi_attrs() {
        let input = parse_quote! {
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                number: usize,
                #[depends(hash)]
                another: usize,
            }
        };
        derive_leaf(input);
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
