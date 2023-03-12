use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse2, spanned::Spanned, Field, Fields, Generics, ItemStruct};

/// In place until we figure out the dependencies generics story
fn block_generics(generics: &Generics) -> syn::Result<()> {
    if !generics.params.is_empty() {
        Err(syn::Error::new(
            generics.span(),
            "dependencies don't currently support generics",
        ))
    } else {
        Ok(())
    }
}

pub fn dependencies_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("dependencies attribute does not support arguments")
    }
    let ItemStruct {
        vis,
        ident,
        generics,
        fields,
        ..
    } = parse2::<ItemStruct>(input).unwrap();

    // until we figure out how to resolve dependencies without propagating
    // throughout the graph, block
    block_generics(&generics).unwrap();
    let name = ident.to_string();
    let ref_ident = Ident::new(format!("{}Ref", name).as_str(), Span::call_site());
    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("must be a struct with named fields");
    };

    let mut new_fields = TokenStream::new();
    let mut ref_fields = TokenStream::new();
    let mut field_args = TokenStream::new();
    let mut field_new_args = TokenStream::new();
    let mut field_resolves = TokenStream::new();
    let mut dirty_field_args = Vec::<TokenStream>::new();
    fields.into_iter().for_each(|Field { vis, ident, ty, .. }| {
        let ident = ident.expect("struct fields must be named.");
        let new_type = quote! { Dependency<Rc<#ty>> };
        new_fields.extend(quote! {
            #vis #ident: #new_type,
        });
        ref_fields.extend(quote! {
            #vis #ident: <#new_type as ::depends::core::Resolve>::Output<'a>,
        });
        field_args.extend(quote! {
            #ident: Rc<#ty>,
        });
        field_new_args.extend(quote! {
            #ident: Dependency::new(#ident),
        });
        field_resolves.extend(quote! {
            #ident: self.#ident.resolve(visitor),
        });
        dirty_field_args.push(quote! {
            self.#ident.is_dirty()
        });
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #vis struct #ident {
            #new_fields
        }

        #vis struct #ref_ident <'a> {
            #ref_fields
        }

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn new(#field_args) -> Self {
                Self {
                    #field_new_args
                }
            }
        }

        impl #impl_generics ::depends::core::Resolve for #ident #ty_generics #where_clause {
            type Output<'a> = #ref_ident<'a> where Self: 'a;

            fn resolve(&self, visitor: &mut impl ::depends::core::Visitor) -> Self::Output<'_> {
                #ref_ident {
                    #field_resolves
                }
            }
        }

        impl <'a> ::depends::core::IsDirty for #ref_ident<'a> {
            fn is_dirty(&self) -> bool {
                #(#dirty_field_args)||*
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
    fn test_dependencies() {
        let input = parse_quote! {
            struct Components {
                node1: Node1,
                node2: Node2,
                node3: Node3,
            }
        };

        assert_snapshot!(
            "dependencies",
            format_source(
                dependencies_attr(TokenStream::new(), input)
                    .to_string()
                    .as_str()
            )
        );
    }

    #[should_panic]
    #[test]
    fn test_dependencies_rejects_attrs() {
        let input = parse_quote! {
            struct Components {
                node1: Node1,
                node2: Node2,
                node3: Node3,
            }
        };

        dependencies_attr(quote! {thing = 1}, input);
    }
}
