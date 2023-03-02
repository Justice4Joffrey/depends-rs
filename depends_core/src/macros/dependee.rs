use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2, Data, DeriveInput, LitBool, Token, Type,
};

use super::attrs::get_depends_attrs;

// TODO: field attr #[depends(hash_this)]

enum DependeeAttr {
    NodeName(Span, Ident),
    Dependencies(Span, Type),
    CustomClean(Span, LitBool),
}

impl Parse for DependeeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        match ident.to_string().as_str() {
            "node_name" => Ok(DependeeAttr::NodeName(ident.span(), input.parse()?)),
            "dependencies" => Ok(DependeeAttr::Dependencies(ident.span(), input.parse()?)),
            "custom_clean" => Ok(DependeeAttr::CustomClean(ident.span(), input.parse()?)),
            unknown => {
                Err(syn::Error::new(
                    ident.span(),
                    format!("Unexpected attribute {:?}", unknown),
                ))
            }
        }
    }
}

struct DependeeParsedAttrs {
    node_name: Option<Ident>,
    dependencies: Option<Type>,
    custom_clean: Option<bool>,
}

impl TryFrom<Vec<DependeeAttr>> for DependeeParsedAttrs {
    type Error = syn::Error;

    fn try_from(value: Vec<DependeeAttr>) -> Result<Self, Self::Error> {
        let mut this = Self {
            node_name: None,
            dependencies: None,
            custom_clean: None,
        };
        for v in value.into_iter() {
            match v {
                DependeeAttr::Dependencies(s, deps) => {
                    if this.dependencies.is_none() {
                        this.dependencies = Some(deps);
                    } else {
                        return Err(syn::Error::new(s, "attribute specified twice"));
                    }
                }
                DependeeAttr::NodeName(s, node) => {
                    if this.node_name.is_none() {
                        this.node_name = Some(node);
                    } else {
                        return Err(syn::Error::new(s, "attribute specified twice"));
                    }
                }
                DependeeAttr::CustomClean(s, value) => {
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

pub fn derive_dependee(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        data,
        generics,
    } = parse2::<DeriveInput>(input).unwrap();
    let name = ident.to_string();

    // parse the attributes
    let (node_ident, dependencies_ty, custom_clean) = {
        let attrs = get_depends_attrs(attrs)
            .expect("attributes must be in the form \"#[depends(... = ..., ...)]\"");
        let parsed: DependeeParsedAttrs = attrs.try_into().unwrap();
        (
            parsed
                .node_name
                .unwrap_or_else(|| Ident::new(format!("{}Node", name).as_str(), Span::call_site())),
            parsed.dependencies.expect("missing \"dependencies\""),
            parsed.custom_clean.unwrap_or(false),
        )
    };

    let clean_clause = if custom_clean {
        TokenStream::new()
    } else {
        quote! {
            impl #generics ::depends::core::Clean for #ident {
                fn clean(&mut self) {}
            }
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let turbo_ident = {
        let turbo = ty_generics.as_turbofish();
        quote! {
            #ident #turbo
        }
    };

    // TODO: generics aren't _actually_ supported. Dependencies must at this
    //  point in time have concrete types.
    //  It's possible in future (perhaps), that a set of dependencies can
    //  have a set of generic outputs. But importantly, these can't be tied
    //  to the type itself, as otherwise generics propagate throughout the
    //  graph implementation.
    //  So, the solution is _probably_ to specify a node by its _input_ NOT
    //  it's dependencies. Who knows. This is all very complicated.

    if let Data::Struct(_data) = data {
        quote! {
            #vis struct #node_ident #generics {
                dependencies: #dependencies_ty,
                data: ::std::cell::RefCell<::depends::core::NodeState<#ident #generics>>,
                id: usize
            }

            impl #impl_generics #node_ident #ty_generics #where_clause {
                pub fn new(dependencies: #dependencies_ty, data: #ident #ty_generics) -> ::std::rc::Rc<#node_ident #ty_generics> {
                    Self::new_with_id(dependencies, data, ::depends::core::next_node_id())
                }

                pub fn new_with_id(dependencies: #dependencies_ty, data: #ident #ty_generics, id: usize) -> ::std::rc::Rc<#node_ident #ty_generics> {
                    ::std::rc::Rc::new(
                        #node_ident {
                            dependencies,
                            data: ::std::cell::RefCell::new(::depends::core::NodeState::new(data)),
                            id
                        }
                    )
                }
            }

            impl #impl_generics ::depends::core::Identifiable for #node_ident #ty_generics #where_clause {
                fn id(&self) -> usize {
                    self.id
                }
            }

            impl #impl_generics #ident #ty_generics #where_clause {
                pub fn into_node(self, dependencies: #dependencies_ty) -> ::std::rc::Rc<#node_ident #ty_generics> {
                    #node_ident::new(
                        dependencies,
                        self,
                    )
                }
            }

            impl #impl_generics ::depends::core::Named for #ident #ty_generics #where_clause {
                fn name() -> &'static str {
                    #name
                }
            }

            impl #impl_generics ::depends::core::Named for #node_ident #ty_generics #where_clause {
                fn name() -> &'static str {
                    #turbo_ident ::name()
                }
            }

            impl #impl_generics ::depends::core::Depends for #ident #ty_generics #where_clause {
                type Input<'a>
                = <#dependencies_ty as ::depends::core::Resolve>::Output<'a>
                    where
                        Self: 'a;
            }

            impl #impl_generics ::depends::core::Resolve for #node_ident #ty_generics #where_clause {
                type Output<'a> = ::std::cell::Ref<'a, ::depends::core::NodeState<#ident>> where Self: 'a;

                fn resolve(&self, visitor: &mut impl ::depends::core::Visitor) -> Self::Output<'_> {
                    use ::depends::core::{IsDirty, Clean};

                    visitor.touch(self);
                    if visitor.visit(self) {
                        let input = self.dependencies.resolve(visitor);
                        if input.is_dirty() {
                            let mut node_state = self.data.borrow_mut();
                            node_state.clean();
                            node_state.data_mut().update_mut(input);
                            node_state.update_node_hash();
                        }
                    }
                    visitor.leave(self);
                    self.data.borrow()
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
    fn test_dependee() {
        let input: TokenStream = parse_quote! {
            #[depends(node_name = SomeName, dependencies = SomeType)]
            #[diesel(some_arg)]
            pub struct Foo {
                #[depends(hi)]
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    fn test_dependee_generics() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee_generics",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    fn test_dependee_generics_custom_clean() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>, custom_clean = true)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee_generics_custom_clean",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    fn test_dependee_single_dependency() {
        let input = parse_quote! {
            #[depends(dependencies = Dependency<Rc<SomeNode<Bar>>>, custom_clean = true)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee_single_dependency",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }
}
