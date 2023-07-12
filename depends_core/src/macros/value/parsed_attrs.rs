use super::{field_attrs::ValueFieldAttr, struct_attrs::ValueStructAttr, ValueAttrModel};
use crate::macros::{common::duplicate_attribute, HashLogic};

pub struct ValueParsedAttrs {
    pub hashing: Option<HashLogic>,
    pub custom_clean: Option<bool>,
}

impl TryFrom<ValueAttrModel> for ValueParsedAttrs {
    type Error = syn::Error;

    fn try_from(attrs: ValueAttrModel) -> Result<Self, Self::Error> {
        let mut this = Self {
            custom_clean: None,
            hashing: None,
        };
        for v in attrs.struct_attrs.into_iter() {
            match v {
                ValueStructAttr::Unhashable(s) => {
                    if this.hashing.is_none() {
                        this.hashing = Some(HashLogic::Unhashable);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
                ValueStructAttr::CustomClean(s) => {
                    if this.custom_clean.is_none() {
                        this.custom_clean = Some(true);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
            }
        }
        for v in attrs.field_attrs.into_iter() {
            for a in v.field_attrs {
                let ValueFieldAttr::Hash(s) = a;
                if this.hashing.is_none() {
                    this.hashing = Some(HashLogic::Field(v.ident.clone()));
                } else {
                    return Err(duplicate_attribute(s));
                }
            }
        }
        Ok(this)
    }
}
