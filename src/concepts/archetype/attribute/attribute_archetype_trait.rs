use crate::concepts::archetype::ArchetypeTrait;
use crate::concepts::attributes::AttributeTrait;

/// Represents types of attributes. Attributes should implement this in addition to
/// `ArchetypeTrait`.
pub trait AttributeArchetypeTrait<'a, A: From<usize>, F: AttributeTrait<'a, F>>:
    ArchetypeTrait<'a, F>
{
    /// Get the AttributeArchetype for this type of Attribute.
    ///
    /// todo: replace this with type parameterization for `ArchetypeTrait`
    fn attribute_archetype() -> A {
        A::from(Self::TYPE_ID)
    }

    /// Create a subtype of the archetype represented by this AttributeArchetype instance.
    ///
    /// todo: replace this with type parameterization for `ArchetypeTrait`
    fn individuate_as_attribute_archetype() -> A {
        A::from(Self::individuate_with_parent(Self::TYPE_ID).id())
    }
}
