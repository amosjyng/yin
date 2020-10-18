//! Contains all attribute archetypes.
mod attribute_form;
mod attribute_trait;
mod has_attribute_type_form;
mod inherits_form;
mod owner_archetype_form;
mod owner_form;
mod value_archetype_form;
mod value_form;

/// Marker for attributes, for compile-time checks.
trait IsAttribute {}

pub use attribute_form::Attribute;
pub use attribute_trait::AttributeTrait;
pub use has_attribute_type_form::HasAttributeType;
pub use inherits_form::Inherits;
pub use owner_archetype_form::OwnerArchetype;
pub use owner_form::Owner;
pub use value_archetype_form::ValueArchetype;
pub use value_form::Value;
