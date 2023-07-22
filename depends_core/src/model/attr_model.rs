use syn::Ident;

/// An abstraction over struct and field-level attributes.
pub struct AttributeModel<S, F> {
    pub struct_attrs: Vec<S>,
    pub field_attrs: Vec<FieldAttrs<F>>,
}

pub struct FieldAttrs<F> {
    pub ident: Ident,
    pub field_attrs: Vec<F>,
}

impl<S, F> From<(Vec<S>, Vec<FieldAttrs<F>>)> for AttributeModel<S, F> {
    fn from((struct_attrs, field_attrs): (Vec<S>, Vec<FieldAttrs<F>>)) -> Self {
        AttributeModel {
            struct_attrs,
            field_attrs,
        }
    }
}
