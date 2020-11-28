use crate::node_wrappers::{CommonNodeTrait, FinalNode};
use crate::tao::form::{Form, FormExtension, FormTrait};
use std::convert::TryFrom;

/// Implement for static access to archetype metadata and typed individuation (individuation
/// through the archetype will return a more generic result than might be desired).
pub trait ArchetypeTrait<'a>: From<usize> + From<FinalNode> + TryFrom<&'a str> + Ord {
    /// The Form that will be used to reason about this node and its children as archetypes and
    /// subtypes.
    type ArchetypeForm: ArchetypeTrait<'a> + FormTrait;
    /// The Form that will be used to reason about this node's leaves as individuals. Unless you
    /// are the Tao, this should be the same as the type that `ArchetypeTrait` is being implemented
    /// on.
    type Form: ArchetypeTrait<'a> + FormTrait;

    /// The ID for this archetype.
    const TYPE_ID: usize;

    /// The name of this archetype.
    const TYPE_NAME: &'static str;

    /// The default parent this archetype inherits from. Every archetype should have at least one
    /// parent, so that it doesn't live in a separate universe of its own. This helps enforce that,
    /// since allocations are not allowed in Rust constants.
    const PARENT_TYPE_ID: usize;

    /// The incarnation of this archetype as a form.
    fn archetype() -> Self::ArchetypeForm {
        Self::ArchetypeForm::from(Self::TYPE_ID)
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
    fn new() -> Self::Form {
        let result = Self::Form::from(FinalNode::new_with_inheritance(Self::TYPE_ID));
        // todo: require FormExtension on Self::Form after it's implemented everywhere
        Form::from(result.id()).mark_individual();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{AttributeTrait, Owner};
    use crate::tao::relation::flag::Flag;
    use crate::tao::relation::Relation;

    #[test]
    fn test_new_node_inheritance() {
        initialize_kb();
        let owner = Owner::new();
        assert_eq!(owner.owner(), None);

        let my_flag = Flag::new();
        // todo: use as_relation() once yang generates that
        let my_flag_rel = Relation::from(my_flag.id());
        Owner::from(Owner::TYPE_ID).set_owner(&my_flag_rel);
        assert_eq!(owner.owner(), Some(my_flag_rel));
    }
}
