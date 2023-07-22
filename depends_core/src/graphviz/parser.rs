use std::collections::HashMap;

use proc_macro2::{extra::DelimSpan, Ident};
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token,
    token::Comma,
    LitStr, Token,
};

mod kw {
    syn::custom_keyword!(digraph);
}

pub struct GraphvizModel {
    #[allow(unused)]
    digraph: kw::digraph,
    pub name: Ident,
    #[allow(unused)]
    brace_token: token::Brace,
    pub vertex_or_edges: Punctuated<VertexOrEdge, Token![;]>,
}

impl Parse for GraphvizModel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let digraph = input.parse()?;
        let name = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let vertex_or_edges = content.parse_terminated(VertexOrEdge::parse, Token![;])?;
        Ok(Self {
            digraph,
            name,
            brace_token,
            vertex_or_edges,
        })
    }
}

pub enum VertexOrEdge {
    Vertex(Vertex),
    Edge(Edge),
}

impl VertexOrEdge {
    pub fn attribute_map(&self) -> HashMap<String, LitStr> {
        let attributes = match self {
            Self::Vertex(vertex) => &vertex.attributes,
            Self::Edge(edge) => &edge.attributes,
        };
        attributes
            .iter()
            .map(|attr| (attr.ident.to_string(), attr.lit_str.clone()))
            .collect()
    }
}

impl Parse for VertexOrEdge {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![-]) {
            let edge = input.parse()?;
            Ok(Self::Edge(edge))
        } else {
            let vertex = input.parse()?;
            Ok(Self::Vertex(vertex))
        }
    }
}

pub struct Vertex {
    pub ident: Ident,
    #[allow(unused)]
    bracket_token: token::Bracket,
    pub attributes: Punctuated<Attribute, Comma>,
    pub attr_span: DelimSpan,
}

impl Parse for Vertex {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let content;
        let bracket_token = bracketed!(content in input);
        let attr_span = bracket_token.span;
        let attributes = content.parse_terminated(Attribute::parse, Comma)?;
        Ok(Self {
            ident,
            bracket_token,
            attributes,
            attr_span,
        })
    }
}

pub struct Edge {
    pub from: Ident,
    #[allow(unused)]
    dash: Token![-],
    #[allow(unused)]
    gt: Token![>],
    pub to: Ident,
    #[allow(unused)]
    bracket_token: token::Bracket,
    pub attributes: Punctuated<Attribute, Comma>,
    pub attr_span: DelimSpan,
}

impl Parse for Edge {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let from = input.parse()?;
        let dash = input.parse()?;
        let gt = input.parse()?;
        let to = input.parse()?;
        let content;
        let bracket_token = bracketed!(content in input);
        let attr_span = bracket_token.span;
        let attributes = content.parse_terminated(Attribute::parse, Comma)?;
        Ok(Self {
            from,
            dash,
            gt,
            to,
            bracket_token,
            attributes,
            attr_span,
        })
    }
}

pub struct Attribute {
    pub ident: Ident,
    #[allow(unused)]
    eq: Token![=],
    pub lit_str: LitStr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let eq = input.parse()?;
        let lit_str = input.parse()?;
        Ok(Self { ident, eq, lit_str })
    }
}
