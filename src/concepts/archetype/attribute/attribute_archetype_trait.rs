use crate::concepts::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use crate::concepts::FormTrait;
use crate::node_wrappers::FinalNode;

/// Represents types of attributes. Attributes should implement this in addition to
/// `ArchetypeTrait`.
///
/// Note that there is a `ArchetypeFormTrait` combining the `ArchetypeTrait` and
/// `FormTrait` into one, but no `AttributeArchetypeFormTrait` doing the same for
/// `AttributeArchetypeTrait` and `AttributeTrait`. This is because an `AttributeArchetype` is not
/// an attribute itself and therefore cannot implement `AttributeTrait`.
///
/// As for the combination of `AttributeArchetypeTrait` and `FormTrait`, the main purpose of
/// having a `ArchetypeFormTrait` was to allow for specialization of function return values. For
/// example, individuating an `AttributeArchetype` should produce another `AttributeArchetype`
/// instead of a generic `Archetype`, and this allows for that. If you look at the functions
/// introduced by `AttributeArchetype`, however, you will find that the owner and value archetypes
/// are very dependent on the attribute itself. This is fine when you have local knowledge, which
/// is why `AttributeArchetypeTrait` allows for specialization when implementing for a specific
/// attribute. But `AttributeArchetype` cannot do this specialization, and it also does not
/// introduce any additional descendant-related functions that are not already covered by
/// `ArchetypeFormTrait`, so it doesn't make sense to create a `AttributeArchetypeFormTrait` that
/// does nothing.
///
/// If you look closely at `ArchetypeFormTrait`, you will find that it too is pretty coarse-grained
/// in its specialization. It uses `Archetype` and `Tao` for everything until you get to
/// `Attribute`, at which point `AttributeArchetype` adds its specialization. Even its
/// specialization does not extend to functions that are not related to descendants -- for example,
/// `ArchetypeFormTrait::introduced_attribute_types` returns `AttributeArchetype`'s regardless of
/// what the `A` parameter is, precisely because no matter what type you are, your attributes will
/// always be attributes, and the type of an attribute will always be guaranteed to be an
/// `AttributeArchetype`, but will not be guaranteed to be anything more specific in general.
///
///  * `A` type parameter: ArchetypeForm representing the attribute's type
///  * `F` type parameter: Form representing the attribute itself
///  * `OA` type parameter: Archetype representing the attribute owner's type
///  * `OF` type parameter: Form representing the attribute owner
///  * `VA` type parameter: Archetype representing the attribute value's type
///  * `VF` type parameter: Form representing the attribute value
pub trait AttributeArchetypeTrait<
    'a,
    A: ArchetypeFormTrait<'a, A, F> + ArchetypeTrait<'a, A> + FormTrait + From<FinalNode>,
    F: ArchetypeTrait<'a, F> + FormTrait + From<FinalNode>,
>: ArchetypeTrait<'a, F>
{
    /// Get the AttributeArchetype for this type of Attribute.
    ///
    /// *Note*: this will be replaced in a future version of Yin with type parameterization in
    /// `ArchetypeTrait`
    fn attribute_archetype() -> A {
        A::from(Self::TYPE_ID)
    }

    /// Create a subtype of the archetype represented by this AttributeArchetype instance.
    ///
    /// *Note*: this will be replaced in a future version of Yin with type parameterization in
    /// `ArchetypeTrait`
    fn individuate_as_attribute_archetype() -> A {
        A::from(Self::individuate_with_parent(Self::TYPE_ID).id())
    }
}
