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
            "Dependencies can't be defined with generics.",
        ))
    } else {
        Ok(())
    }
}

pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    derive_dependencies_inner(input).unwrap_or_else(syn::Error::into_compile_error)
}

/// Limit on the number of edges to a node. This is arbitrary.
const MAX_FIELDS: usize = 26;

fn derive_dependencies_inner(input: TokenStream) -> syn::Result<TokenStream> {
    let ItemStruct {
        vis,
        ident,
        generics,
        fields,
        ..
    } = parse2::<ItemStruct>(input)?;

    // Generics are created for derived types, so it's not clear how we
    // _could_ support them.
    block_generics(&generics)?;
    let name = ident.to_string();
    let ref_ident = Ident::new(format!("{name}Ref").as_str(), Span::call_site());
    let dep_name = format!("{name}Dep");
    let dep_ident = Ident::new(&dep_name, Span::call_site());
    let fields = if let Fields::Named(f) = fields {
        f.named
    } else {
        return Err(syn::Error::new(
            Span::call_site(),
            "Must be a struct with named fields.",
        ));
    };
    // kind of arbitrary, but we don't have infinite alphabet and this is
    // already a large number of dependencies.
    if fields.len() < 2 {
        return Err(syn::Error::new(Span::call_site(), "Dependencies must have at least 2 fields. Use `depends::Dependency` for a single dependency."));
    }
    if fields.len() > MAX_FIELDS {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("Dependencies only supports structs with up to {MAX_FIELDS} fields."),
        ));
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
    let mut rc_types = Vec::<TokenStream>::new();
    let mut names = Vec::<Ident>::new();
    let mut where_clauses = Vec::<TokenStream>::new();

    for (Field { vis, ident, ty, .. }, gen_ident) in fields.into_iter().zip('A'..='Z') {
        let ident_span = ident.span();
        let ident =
            ident.ok_or_else(|| syn::Error::new(ident_span, "Struct fields must be named."))?;
        let gen_ident = Ident::new(gen_ident.to_string().as_str(), Span::call_site());
        let new_type = quote! { ::depends::DepRef<#lifetime, ::std::cell::Ref<#lifetime, ::depends::NodeState<#ty>>> };
        rc_types.push(quote! {
           ::std::rc::Rc<#gen_ident>
        });
        new_fields.extend(quote! {
            #vis #ident: ::depends::Dependency<::std::rc::Rc<#gen_ident>>,
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
            #ident: ::depends::Dependency::new(#ident),
        });
        field_resolves.extend(quote! {
            #ident: self.#ident.resolve(visitor)?,
        });
        dirty_field_args.push(quote! {
            self.#ident.is_dirty()
        });
        where_clauses.push(quote! {
                for<#lifetime> #gen_ident: ::depends::Resolve<Output<#lifetime> = ::std::cell::Ref<#lifetime, ::depends::NodeState<#ty>>> + #lifetime
            });
        generics
            .params
            .push(GenericParam::Type(TypeParam::from(gen_ident)));
        names.push(ident);
    }

    let mut ref_generics = Generics::default();
    ref_generics
        .params
        .push(GenericParam::Lifetime(lifetime.clone()));

    Ok(quote! {
        #vis struct #dep_ident #generics {
            #new_fields
        }

        impl #generics ::depends::Named for #dep_ident #generics {
            fn name() -> &'static str {
                #dep_name
            }
        }

        #vis struct #ref_ident #ref_generics {
            #ref_fields
        }

        impl #ident
        {
            #[allow(clippy::too_many_arguments)]
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

        impl #generics From<( #(#rc_types),* )> for #dep_ident #generics
        where
            #(#where_clauses),*
        {
            fn from(( #(#names),* ): ( #(#rc_types),* )) -> Self {
                Self {
                    #field_new_args
                }
            }
        }

        impl #generics ::depends::Resolve for #dep_ident #generics
            where
                #(#where_clauses),*
        {
            type Output<#lifetime> = #ref_ident #ref_generics where Self: #lifetime;

            fn resolve(&self, visitor: &mut impl ::depends::Visitor) -> ::depends::error::ResolveResult<Self::Output<'_>> {
                use ::depends::Named;
                visitor.touch_dependency_group(Self::name());
                Ok(#ref_ident {
                    #field_resolves
                })
            }
        }

        impl ::depends::IsDirty for #ref_ident <'_> {
            fn is_dirty(&self) -> bool {
                #(#dirty_field_args)||*
            }
        }
    })
}

#[cfg(all(test, not(miri)))]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::helpers::format_source;

    #[test]
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
}
