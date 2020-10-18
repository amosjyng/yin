use super::Archetype;
use crate::concepts::FormTrait;
use std::convert::TryFrom;

/// All formally defined archetypes should be describable by these properties.
///
/// This contains all the statics that FormTrait does not contain.
///
/// * `F` type parameter: represents the direct Form that will reason about the node's descendants
pub trait ArchetypeTrait<'a, F: FormTrait>: From<usize> + TryFrom<&'a str> + Ord {
    /// The ID for this archetype.
    const TYPE_ID: usize;

    /// The name of this archetype.
    const TYPE_NAME: &'static str;

    /// The default parent this archetype inherits from. Every archetype should have at least one
    /// parent, so that it doesn't live in a separate universe of its own. This helps enforce that,
    /// since allocations are not allowed in Rust constants.
    const PARENT_TYPE_ID: usize;

    /// The incarnation of this archetype as a form.
    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    /// In the beginning was the Oneness, and the Oneness was nothingness.
    ///
    /// And no one said "Let there be the null set," but there was the null set.
    ///
    /// The null set was, and it separated itself from the wasn't.
    ///
    /// And there was the null set, and there was the set containing the null set -- the first
    /// [ordinal](https://en.wikipedia.org/wiki/Natural_number#Zermelo_ordinals).
    ///
    /// And there was recursion -- the naturals.
    ///
    /// From this countable infinity all forms emerged, dividing the Oneness again and again into
    /// Self and Other. The time has come to stroke the ego, to stand out from the rest of the
    /// world as a unique individual engaging in the act of self-realization.
    fn individuate() -> F {
        Self::individuate_with_parent(Self::TYPE_ID)
    }

    /// Create a subtype of the archetype represented here.
    fn individuate_as_archetype() -> Archetype {
        Archetype::from(Self::individuate_with_parent(Self::TYPE_ID).id())
    }

    /// Individuate with a more specific parent than the current one. This custom parent should
    /// inherit from the current type.
    fn individuate_with_parent(parent_id: usize) -> F;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::{AttributeTrait, Owner};
    use crate::concepts::{initialize_kb, FormTrait};

    #[test]
    fn test_new_node_inheritance() {
        initialize_kb();
        let owner = Owner::individuate();
        assert_eq!(owner.owner(), None);

        let attr = Owner::individuate();
        Owner::from(Owner::TYPE_ID).set_owner(&attr);
        assert_eq!(owner.owner(), Some(attr.ego_death()));
    }
}
