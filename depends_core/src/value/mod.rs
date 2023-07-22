mod derive;
mod field_attrs;
mod parsed_attrs;
mod struct_attrs;

pub use derive::derive_value;

use self::{field_attrs::ValueFieldAttr, struct_attrs::ValueStructAttr};
use super::AttributeModel;

type ValueAttrModel = AttributeModel<ValueStructAttr, ValueFieldAttr>;
