use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse2, spanned::Spanned, Field, Fields, GenericParam, Generics, ItemStruct, Lifetime,
    LifetimeParam, TypeParam,
};

/// In place until we figure out the dependencies generics story
fn block_generics(generics: &Generics) -> syn::Result<()> {
    if !generics.params.is_empty() {
        Err(syn::Error::new(
            generics.span(),
            "dependencies don't support generics",
        ))
    } else {
        Ok(())
    }
}

pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    let ItemStruct {
        vis,
        ident,
        generics,
        fields,
        ..
    } = parse2::<ItemStruct>(input).unwrap();

    // Generics are created for derived types, so it's not clear how we
    // _could_ support them.
    block_generics(&generics).unwrap();
    let name = ident.to_string();
    let ref_ident = Ident::new(format!("{}Ref", name).as_str(), Span::call_site());
    let dep_ident = Ident::new(format!("{}Dep", name).as_str(), Span::call_site());
    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        panic!("must be a struct with named fields");
    };
    // kind of arbitrary, but we don't have infinite alphabet and this is
    // already a large number of dependencies.
    if fields.len() < 2 {
        panic!("dependencies must have at least 2 fields");
    }
    if fields.len() > 16 {
        panic!("dependencies only supports structs with up to 16 fields");
    }

    let lifetime = LifetimeParam::new(Lifetime::new("'a", Span::call_site()));

    let mut new_fields = TokenStream::new();
    // use all fields in a dead_code block to prevent clippy from complaining
    let mut unused_fields = Vec::<TokenStream>::new();
    let mut ref_fields = TokenStream::new();
    let mut field_args = TokenStream::new();
    let mut field_new_args = TokenStream::new();
    let mut field_resolves = TokenStream::new();
    let mut dirty_field_args = Vec::<TokenStream>::new();
    let mut generics = Generics::default();
    let mut where_clauses = Vec::<TokenStream>::new();

    fields
        .into_iter()
        .zip('A'..='Z')
        .for_each(|(Field { vis, ident, ty, .. }, gen_ident)| {
            let ident = ident.expect("struct fields must be named.");
            let gen_ident = Ident::new(gen_ident.to_string().as_str(), Span::call_site());
            let new_type = quote! { ::depends::core::DepRef<#lifetime, ::std::cell::Ref<#lifetime, ::depends::core::NodeState<#ty>>> };
            generics
                .params
                .push(GenericParam::Type(TypeParam::from(gen_ident.clone())));
            new_fields.extend(quote! {
                #vis #ident: ::depends::core::Dependency<::std::rc::Rc<#gen_ident>>,
            });
            unused_fields.push(quote! {
                let _ = self.#ident;
            });
            ref_fields.extend(quote! {
                #vis #ident: #new_type,
            });
            field_args.extend(quote! {
                #ident: ::std::rc::Rc<#gen_ident>,
            });
            field_new_args.extend(quote! {
                #ident: ::depends::core::Dependency::new(#ident),
            });
            field_resolves.extend(quote! {
                #ident: self.#ident.resolve(visitor)?,
            });
            dirty_field_args.push(quote! {
                self.#ident.is_dirty()
            });
            where_clauses.push(quote! {
                for<#lifetime> #gen_ident: ::depends::core::Resolve<Output<#lifetime> = ::std::cell::Ref<#lifetime, ::depends::core::NodeState<#ty>>> + #lifetime
            });
        });

    let mut ref_generics = Generics::default();
    ref_generics
        .params
        .push(GenericParam::Lifetime(lifetime.clone()));

    quote! {
        #vis struct #dep_ident #generics {
            #new_fields
        }

        #vis struct #ref_ident #ref_generics {
            #ref_fields
        }

        impl #ident
        {
            pub fn init #generics(#field_args) -> #dep_ident #generics
            where
                #(#where_clauses),*
            {
                #dep_ident {
                    #field_new_args
                }
            }

            #[allow(dead_code)]
            fn __unused(&self) {
                #(#unused_fields);*
            }
        }

        impl #generics ::depends::core::Resolve for #dep_ident #generics
            where
                #(#where_clauses),*
        {
            type Output<#lifetime> = #ref_ident #ref_generics where Self: #lifetime;

            fn resolve(&self, visitor: &mut impl ::depends::core::Visitor) -> ::depends::core::error::ResolveResult<Self::Output<'_>> {
                Ok(#ref_ident {
                    #field_resolves
                })
            }
        }

        impl ::depends::core::IsDirty for #ref_ident <'_> {
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
            format_source(derive_dependencies(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_dependencies_no_fields() {
        let input = parse_quote! {
            struct Components {
            }
        };
        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_one_field() {
        let input = parse_quote! {
            struct Components {
                node1: Node1,
            }
        };
        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_tuple() {
        let input = parse_quote! {
            struct Components (
                Node1,
            );
        };
        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_enum() {
        let input = parse_quote! {
            enum Components (
                Node1 = 1;
            );
        };
        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_too_many_fields() {
        let input = parse_quote! {
            struct Components {
                node1: Node1,
                node2: Node2,
                node3: Node3,
                node4: Node4,
                node5: Node5,
                node6: Node6,
                node7: Node7,
                node8: Node8,
                node9: Node9,
                node10: Node10,
                node11: Node11,
                node12: Node12,
                node13: Node13,
                node14: Node14,
                node15: Node15,
                node16: Node16,
                node17: Node17,
            }
        };
        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_generics() {
        let input = parse_quote! {
            struct Components<A, B>{
                node1: A,
                node2: B,
                node3: Node3,
            }
        };
        derive_dependencies(input);
    }
}
