use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    spanned::Spanned,
    Data, DeriveInput, Field, Generics, Token,
};

use super::attrs::get_depends_attrs;

enum DependenciesAttr {
    RefName(Span, Ident),
}

impl Parse for DependenciesAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        match ident.to_string().as_str() {
            "ref_name" => Ok(DependenciesAttr::RefName(ident.span(), input.parse()?)),
            unknown => {
                Err(syn::Error::new(
                    ident.span(),
                    format!("Unexpected attribute {:?}", unknown),
                ))
            }
        }
    }
}

struct DependenciesParsedAttrs {
    ref_name: Option<Ident>,
}

impl TryFrom<Vec<DependenciesAttr>> for DependenciesParsedAttrs {
    type Error = syn::Error;

    fn try_from(value: Vec<DependenciesAttr>) -> Result<Self, Self::Error> {
        let mut this = Self { ref_name: None };
        for v in value.into_iter() {
            match v {
                DependenciesAttr::RefName(s, ref_name) => {
                    if this.ref_name.is_none() {
                        this.ref_name = Some(ref_name);
                    } else {
                        return Err(syn::Error::new(s, "attribute specified twice"));
                    }
                }
            }
        }
        Ok(this)
    }
}

/// In place until we figure out the dependencies generics story
fn block_generics(generics: &Generics) -> syn::Result<()> {
    if !generics.params.is_empty() {
        Err(syn::Error::new(
            generics.span(),
            "Dependencies don't currently support generics",
        ))
    } else {
        Ok(())
    }
}

pub fn derive_dependencies(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse2::<DeriveInput>(input).unwrap();
    let name = ident.to_string();

    block_generics(&generics).unwrap();

    // parse the attributes
    let ref_ident = {
        let attrs = get_depends_attrs(attrs)
            .expect("attributes must be in the form \"#[depends(... = ..., ...)]\"");
        let parsed: DependenciesParsedAttrs = attrs.try_into().unwrap();
        parsed
            .ref_name
            .unwrap_or_else(|| Ident::new(format!("{}Ref", name).as_str(), Span::call_site()))
    };

    if let Data::Struct(data) = data {
        let mut fields = TokenStream::new();
        let mut field_args = TokenStream::new();
        let mut field_new_args = TokenStream::new();
        let mut field_resolves = TokenStream::new();
        let mut field_cleans = TokenStream::new();
        let mut field_edges = TokenStream::new();
        let mut dirty_field_args = Vec::<TokenStream>::new();
        data.fields.into_iter().for_each(
            |Field {
                 attrs: _,
                 vis,
                 ident,
                 colon_token: _,
                 ty,
             }| {
                let ident = ident.expect("struct fields must be named.");
                fields.extend(quote! {
                    #vis #ident: <#ty as ::depends::core::Resolve>::Output<'a>,
                });
                field_args.extend(quote! {
                    #ident: #ty,
                });
                field_new_args.extend(quote! {
                    #ident,
                });
                field_resolves.extend(quote! {
                    #ident: self.#ident.resolve(visitor),
                });
                field_cleans.extend(quote! {
                   self.#ident.clean(visitor);
                });
                field_edges.extend(quote! {
                    visitor.mark_edge(&self.#ident);
                });
                dirty_field_args.push(quote! {
                    self.#ident.is_dirty()
                });
            },
        );

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            pub struct #ref_ident <'a> {
                #fields
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
                    #field_edges

                    #ref_ident {
                        #field_resolves
                    }
                }

                fn clean(&self, visitor: &mut impl ::depends::core::Visitor) {
                    #field_cleans
                }
            }

            impl <'a> ::depends::core::IsDirty for #ref_ident<'a> {
                fn is_dirty(&self) -> bool {
                    #(#dirty_field_args)||*
                }
            }
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
    fn test_dependencies() {
        let input = parse_quote! {
            #[diesel(some_arg)]
            struct Foo {
                #[depends(hi)]
                bar: Vec<usize>,
                baz: String,
            }
        };

        assert_snapshot!(
            "dependencies",
            format_source(derive_dependencies(input).to_string().as_str())
        );
    }

    #[test]
    fn test_dependencies_generics_ref_name() {
        let input = parse_quote! {
            #[depends(ref_name = CustomRefName)]
            struct Foo {
                bar: Vec<usize>
            }
        };

        assert_snapshot!(
            "dependencies_ref_name",
            format_source(derive_dependencies(input).to_string().as_str())
        );
    }

    #[test]
    #[should_panic]
    fn test_dependencies_generics() {
        let input = parse_quote! {
            struct Foo<T> {
                bar: Vec<usize>
            }
        };

        derive_dependencies(input);
    }

    #[test]
    #[should_panic]
    fn test_dependencies_on_enum() {
        let input = parse_quote! {
            enum Foo {
                Thing(usize)
            }
        };

        derive_dependencies(input);
    }
}
