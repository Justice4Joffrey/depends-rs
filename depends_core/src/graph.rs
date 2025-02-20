use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse2,
    punctuated::Punctuated,
    token::{Comma, Underscore},
    ItemStruct, Token,
};

use crate::{
    common::snake_case,
    get_depends_attrs,
    graphviz::{DerivedNodeDef, GraphvizGraph, GraphvizModel, ParsedGraphvizModel},
};

pub fn derive_graph(input: TokenStream) -> TokenStream {
    derive_graph_inner(input).unwrap_or_else(syn::Error::into_compile_error)
}

fn derive_graph_inner(input: TokenStream) -> syn::Result<TokenStream> {
    let ItemStruct { ident, attrs, .. } = parse2::<ItemStruct>(input).unwrap();

    let model = {
        let mut attrs = get_depends_attrs::<GraphvizModel>(&attrs)?;
        let graph = attrs.pop();
        match (graph, attrs.len()) {
            (Some(graph), 0) => graph,
            (Some(_), _) => {
                Err(syn::Error::new(
                    Span::call_site(),
                    "More than one #[depends(..)] attribute present.",
                ))?
            }
            (None, _) => {
                Err(syn::Error::new(
                    Span::call_site(),
                    "Provide a graph definition in the #[depends(..)] attribute.",
                ))?
            }
        }
    };
    let parsed: ParsedGraphvizModel = model.try_into()?;
    let checked: GraphvizGraph = parsed.try_into()?;

    let module_name = Ident::new(
        &format!("__depends_graph_{}", snake_case(&ident.to_string())),
        Span::call_site(),
    );
    let graph_ident = checked.name;
    // Unwrap is safe because we checked that the graph has at least one node.
    let root_node = checked.derived.last().cloned().unwrap();
    let root_ident = root_node.node.ident;
    let root_type = root_node.node.ty;
    let root_var_name = Ident::new(
        snake_case(root_ident.to_string().as_str()).as_str(),
        Span::call_site(),
    );
    let root_fn_name = Ident::new(
        format!("create_{}", snake_case(graph_ident.to_string().as_str())).as_str(),
        Span::call_site(),
    );
    let inputs = checked.inputs;
    let mut input_idents = vec![];
    let mut input_types = vec![];
    let mut fn_names = vec![];

    let derived = checked.derived;
    let mut derived_var_names = vec![];
    let mut derived_types = vec![];
    let mut derive_constructors = vec![];
    for input in inputs.iter() {
        let var_name = Ident::new(
            snake_case(input.ident.to_string().as_str()).as_str(),
            Span::call_site(),
        );
        let fn_name = Ident::new(
            format!("update_{}", snake_case(input.ident.to_string().as_str())).as_str(),
            Span::call_site(),
        );
        input_idents.push(var_name);
        input_types.push(input.ty.clone());
        fn_names.push(fn_name);
    }
    for derived in derived.iter() {
        let (constructor, var_name, ty) = derived_node_create(derived);
        derived_var_names.push(var_name);
        derive_constructors.push(constructor);
        derived_types.push(ty);
    }
    let constraint = quote! {
        for<'a> R: ::depends::Resolve<Output<'a> = ::std::cell::Ref<'a, ::depends::NodeState<#root_type>>> + 'a,
    };
    // Safety: Since we only ever create these `Rc`s in this private type,
    // and no-one else can access them to clone them, their reference-count
    // is constant and valid.
    // TODO: I'm not 100% sure about behaviour on drop.
    // TODO: you probably have to enforce Send as the bound on InputNode +
    Ok(quote! {
        mod #module_name {
            use super::*;

            pub struct #graph_ident<R> {
                #(#input_idents: ::std::rc::Rc<::depends::InputNode<#input_types>>,)*
                #root_var_name: R
            }

            unsafe impl<R> Send for #graph_ident<R> {}

            impl<R> #graph_ident<R>
            where
                #constraint
            {
                #[allow(clippy::too_many_arguments)]
                fn new(#(#input_idents: ::std::rc::Rc<::depends::InputNode<#input_types>>,)* #root_var_name: R) -> Self {
                    Self {
                        #(#input_idents,)*
                        #root_var_name
                    }
                }

                #(pub fn #fn_names(&self, update: <#input_types as ::depends::UpdateInput>::Update) -> ::depends::error::ResolveResult<()> {
                    self.#input_idents.update(update)
                })*
            }

            impl<R> ::depends::Resolve for #graph_ident<R>
            where
                #constraint
            {
                type Output<'a> = <R as ::depends::Resolve>::Output<'a> where Self: 'a;

                fn resolve(&self, visitor: &mut impl ::depends::Visitor) -> ::depends::error::ResolveResult<Self::Output<'_>> {
                    self.#root_var_name.resolve(visitor)
                }
            }

            impl #ident {
                #[allow(clippy::too_many_arguments)]
                pub fn #root_fn_name(
                    #(#input_idents: #input_types,)* #(#derived_var_names: #derived_types),*
                ) -> #module_name :: #graph_ident<impl for<'a> ::depends::Resolve<Output<'a> = ::std::cell::Ref<'a, ::depends::NodeState<#root_type>>>> {
                    #(let #input_idents = ::depends::InputNode::new(#input_idents);)*
                    #(#derive_constructors)*
                    #module_name :: #graph_ident::new(
                        #(#input_idents,)*
                        #root_var_name
                    )
                }
            }

            impl ::depends::GraphCreate for #ident {
                type Graph<R> = #graph_ident<R>;
            }
        }
    })
}

fn derived_node_create(derived: &DerivedNodeDef) -> (TokenStream, Ident, Ident) {
    let ident = derived.node.ident.clone();
    let ty = derived.node.ty.clone();
    let operation = derived.operation.clone();
    let mut deps = Vec::with_capacity(derived.incoming.len());
    // This is probably not the best way of generating `_, _, _, ..` * n
    let mut generics = Punctuated::<Token![_], Comma>::new();
    derived.incoming.iter().for_each(|n| {
        deps.push(n.ident.clone());
        generics.push(Underscore::default());
    });

    let deps_init = if let Some(dep_type) = derived.dependency_type.as_ref() {
        quote! {
            {
                let d = #dep_type::< #generics >::new(#(::std::rc::Rc::clone(& #deps)),*);
                d
            }
        }
    } else {
        // There is only ever one
        quote! {
            #(::depends::Dependency::new(::std::rc::Rc::clone(& #deps)))*
        }
    };
    (
        quote! {
            let #ident = ::depends::DerivedNode::new(
                #deps_init,
                #operation,
                #ident
            );
        },
        ident,
        ty,
    )
}

#[cfg(all(test, not(miri)))]
mod tests {
    use insta::assert_snapshot;
    use syn::parse_quote;

    use super::*;
    use crate::helpers::format_source;

    #[test]
    fn test_graph() {
        let input = parse_quote! {
            #[depends(
                digraph Dag {
                   node_0 [label="Comments"];
                   node_1 [label="Posts"];
                   node_2 [label="Likes"];
                   node_3 [label="CommentsToPosts"];
                   node_0 -> node_3 [label="CommentPostIds"];
                   node_4 [label="PostScoresQuery"];
                   node_0 -> node_4 [label="UpdatePostScoresQuery", class="Dependencies4"];
                   node_1 -> node_4 [label="UpdatePostScoresQuery", class="Dependencies4"];
                   node_2 -> node_4 [label="UpdatePostScoresQuery", class="Dependencies4"];
                   node_3 -> node_4 [label="UpdatePostScoresQuery", class="Dependencies4"];
                }
            )]
            struct Components {
                node1: Node1,
                node2: Node2,
                node3: Node3,
            }
        };

        assert_snapshot!(
            "graph",
            format_source(derive_graph(input).to_string().as_str())
        );
    }
}
