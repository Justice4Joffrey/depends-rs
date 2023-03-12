use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse2, Data, DeriveInput, Field};

use super::{parsed_attrs::DependeeParsedAttrs, DependeeAttrModel};
use crate::macros::{model::get_depends_attrs, FieldAttrs, HashLogic};

pub fn derive_dependee(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        data,
        generics,
    } = parse2::<DeriveInput>(input).unwrap();
    let data = if let Data::Struct(data) = data {
        data
    } else {
        panic!("This macro can only be derived for structs.");
    };
    let name = ident.to_string();

    let (node_ident, dependencies_ty, custom_clean, hash_logic) = {
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

        let model = DependeeAttrModel {
            struct_attrs,
            field_attrs,
        };
        let parsed: DependeeParsedAttrs = model.try_into().unwrap();
        (
            parsed
                .node_name
                .unwrap_or_else(|| Ident::new(format!("{}Node", name).as_str(), Span::call_site())),
            parsed.dependencies.expect("missing \"dependencies\""),
            parsed.custom_clean.unwrap_or(false),
            parsed.hashing.unwrap_or(HashLogic::Struct),
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

    let hash_value_clause = hash_logic.to_tokens();

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
                        data: ::std::cell::RefCell::new(::depends::core::NodeState::new_dependee(data)),
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
                use ::std::ops::DerefMut;

                visitor.touch(self);
                if visitor.visit(self) {
                    let input = self.dependencies.resolve(visitor);
                    if input.is_dirty() {
                        let mut node_state = self.data.borrow_mut();
                        node_state.clean();
                        node_state.deref_mut().update_mut(input);
                        node_state.update_node_hash(&mut visitor.hasher());
                    }
                }
                visitor.leave(self);
                self.data.borrow()
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
    fn test_dependee() {
        let input: TokenStream = parse_quote! {
            #[depends(node_name = SomeName, dependencies = SomeType)]
            #[diesel(some_arg)]
            pub struct Foo {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
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
    #[ignore]
    fn test_dependee_generics_custom_clean() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>, custom_clean)]
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
    #[ignore]
    fn test_dependee_unhashable() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>, unhashable)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee_generics_unhashable",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_dependee_single_dependency() {
        let input = parse_quote! {
            #[depends(dependencies = Dependency<Rc<SomeNode<Bar>>>, custom_clean)]
            struct Foo<T> {
                bar: Vec<usize>
            }
        };
        assert_snapshot!(
            "dependee_single_dependency",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    #[ignore]
    fn test_dependee_hashable_field() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>)]
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                this_number: usize
            }
        };
        assert_snapshot!(
            "dependee_hashable_field",
            format_source(derive_dependee(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_dependee_hash_declared_multiple() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>, unhashable)]
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                this_number: usize
            }
        };
        derive_dependee(input);
    }

    #[test]
    #[should_panic]
    fn test_dependee_hash_declared_multiple_fields() {
        let input = parse_quote! {
            #[depends(dependencies = SomeDeps<D, E>)]
            struct Foo<T> {
                bar: Vec<usize>,
                #[depends(hash)]
                this_number: usize,
                #[depends(hash)]
                another_number: usize
            }
        };
        derive_dependee(input);
    }
}
