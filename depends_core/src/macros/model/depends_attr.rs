use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::{Comma, Paren},
    Attribute,
};

/// The attribute outer token.
const DEPENDS: &str = "depends";

/// Extracts an attribute token containing multiple attribute declarations.
///
/// e.g.
///
/// #[depends(a = 1, b)]
/// #[depends(c)]
///
/// becomes
///
/// vec![a = 1, b, c]
struct DependsAttrs<T> {
    attrs: Punctuated<T, Comma>,
}

impl<T: Parse> Parse for DependsAttrs<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _: Paren = parenthesized!(content in input);
        Ok(Self {
            attrs: content.parse_terminated(T::parse)?,
        })
    }
}

fn get_depends_attrs_res<T: Parse>(attrs: &[Attribute]) -> syn::Result<Vec<T>> {
    let mut res = Vec::new();
    for a in attrs {
        if a.path.is_ident(DEPENDS) {
            let dep_attrs = parse2::<DependsAttrs<T>>(a.tokens.clone())?;
            dep_attrs.attrs.into_iter().for_each(|a| res.push(a))
        }
    }
    Ok(res)
}

pub fn get_depends_attrs<T: Parse>(attrs: &[Attribute]) -> Vec<T> {
    get_depends_attrs_res(attrs)
        .expect("attributes must be in the form \"#[depends(.. = .., ..)]\"")
}
