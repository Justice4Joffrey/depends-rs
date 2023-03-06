use syn::{Ident, Type};

use super::{field_attrs::DependeeFieldAttr, struct_attrs::DependeeStructAttr, DependeeAttrModel};
use crate::macros::{common::duplicate_attribute, HashLogic};

pub struct DependeeParsedAttrs {
    pub node_name: Option<Ident>,
    pub dependencies: Option<Type>,
    pub hashing: Option<HashLogic>,
    pub custom_clean: Option<bool>,
}

impl TryFrom<DependeeAttrModel> for DependeeParsedAttrs {
    type Error = syn::Error;

    fn try_from(attrs: DependeeAttrModel) -> Result<Self, Self::Error> {
        let mut this = DependeeParsedAttrs {
            custom_clean: None,
            hashing: None,
            node_name: None,
            dependencies: None,
        };
        for v in attrs.struct_attrs.into_iter() {
            match v {
                DependeeStructAttr::Unhashable(s) => {
                    if this.hashing.is_none() {
                        this.hashing = Some(HashLogic::Unhashable);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
                DependeeStructAttr::CustomClean(s) => {
                    if this.custom_clean.is_none() {
                        this.custom_clean = Some(true);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
                DependeeStructAttr::NodeName(s, ident) => {
                    if this.node_name.is_none() {
                        this.node_name = Some(ident);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
                DependeeStructAttr::Dependencies(s, typ) => {
                    if this.dependencies.is_none() {
                        this.dependencies = Some(typ);
                    } else {
                        return Err(duplicate_attribute(s));
                    }
                }
            }
        }
        for v in attrs.field_attrs.into_iter() {
            for a in v.field_attrs {
                let DependeeFieldAttr::Hash(s) = a;
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
