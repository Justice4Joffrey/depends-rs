use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct};

pub fn derive_operation(input: TokenStream) -> TokenStream {
    let ItemStruct {
        ident, generics, ..
    } = parse2::<ItemStruct>(input).unwrap();
    let name = ident.to_string();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::depends::core::Named for #ident #ty_generics #where_clause {
            fn name() -> &'static str {
                #name
            }
        }
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
    fn test_operation() {
        let input = parse_quote! {
            struct Operation {}
        };

        assert_snapshot!(
            "operation",
            format_source(derive_operation(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_operation_semi_colon() {
        let input = parse_quote! {
            struct Operation;
        };

        assert_snapshot!(
            "operation_semi_colon",
            format_source(derive_operation(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_operation_fields() {
        let input = parse_quote! {
            struct Operation {
                a: A,
                b: B,
            }
        };

        assert_snapshot!(
            "operation_fields",
            format_source(derive_operation(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_operation_tuple() {
        let input = parse_quote! {
            struct TupleOperation (
                A,
                B,
            );
        };

        assert_snapshot!(
            "operation_tuple",
            format_source(derive_operation(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_dependencies_enum() {
        let input = parse_quote! {
            enum Components (
                Node1 = 1;
            );
        };
        derive_operation(input);
    }
}
