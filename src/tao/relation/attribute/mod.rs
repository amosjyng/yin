//! Relations between two forms.

pub mod has_property;

mod attribute_form;
mod attribute_trait;
mod default_value_form;
mod inherits_form;
mod meta_form_form;
mod owner_archetype_form;
mod owner_form;
mod value_archetype_form;
mod value_form;

pub use attribute_form::Attribute;
pub use attribute_trait::AttributeTrait;
pub use default_value_form::DefaultValue;
pub use inherits_form::Inherits;
pub use meta_form_form::MetaForm;
pub use owner_archetype_form::OwnerArchetype;
pub use owner_form::Owner;
pub use value_archetype_form::ValueArchetype;
pub use value_form::Value;
