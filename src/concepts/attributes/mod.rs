//! Contains all attribute archetypes.
mod attribute_archetype;
mod attribute_trait;
mod has_attribute_type;
mod inherits;
mod owner;
mod owner_archetype;
mod value;
mod value_archetype;

pub use attribute_archetype::Attribute;
pub use attribute_trait::AttributeTrait;
pub use has_attribute_type::HasAttributeType;
pub use inherits::Inherits;
pub use owner::Owner;
pub use owner_archetype::OwnerArchetype;
pub use value::Value;
pub use value_archetype::ValueArchetype;
