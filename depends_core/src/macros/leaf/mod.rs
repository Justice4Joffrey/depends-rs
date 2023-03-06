mod derive;
mod field_attrs;
mod parsed_attrs;
mod struct_attrs;

pub use derive::derive_leaf;

use self::{field_attrs::LeafFieldAttr, struct_attrs::LeafStructAttr};
use super::AttributeModel;

type LeafAttrModel = AttributeModel<LeafStructAttr, LeafFieldAttr>;
