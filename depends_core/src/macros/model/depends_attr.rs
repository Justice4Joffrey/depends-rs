use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token::Comma,
    Attribute, Meta,
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
        Ok(Self {
            attrs: input.parse_terminated(T::parse, Comma)?,
        })
    }
}

fn get_depends_attrs_res<T: Parse>(attrs: &[Attribute]) -> syn::Result<Vec<T>> {
    let mut res = Vec::new();
    for a in attrs {
        if a.path().is_ident(DEPENDS) {
            if let Meta::List(l) = &a.meta {
                let dep_attrs = parse2::<DependsAttrs<T>>(l.tokens.clone())?;
                dep_attrs.attrs.into_iter().for_each(|a| res.push(a))
            } else {
                return Err(syn::Error::new_spanned(a, "invalid attribute format"));
            }
        }
    }
    Ok(res)
}

pub fn get_depends_attrs<T: Parse>(attrs: &[Attribute]) -> Vec<T> {
    get_depends_attrs_res(attrs)
        .expect("attributes must be in the form \"#[depends(.. = .., ..)]\"")
}
