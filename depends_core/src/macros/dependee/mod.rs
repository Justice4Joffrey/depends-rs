mod derive;

mod field_attrs;
mod parsed_attrs;
mod struct_attrs;

pub use derive::derive_dependee;
use field_attrs::DependeeFieldAttr;
use struct_attrs::DependeeStructAttr;

use crate::macros::model::AttributeModel;

type DependeeAttrModel = AttributeModel<DependeeStructAttr, DependeeFieldAttr>;
