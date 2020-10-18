mod archetype_form;
mod archetype_form_trait;
mod archetype_trait;
mod attribute_archetype_form;

/// Marker for archetypes, for compile-time checks.
pub trait IsArchetype {}

pub use archetype_form::Archetype;
pub use archetype_form_trait::ArchetypeFormTrait;
pub use archetype_trait::ArchetypeTrait;
pub use attribute_archetype_form::AttributeArchetype;
