use super::Archetype;
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::HasProperty;
use crate::tao::relation::flag::Flag;
use crate::Wrapper;

/// Public trait to store eventually-automated archetype attributes in.
pub trait ArchetypeFormExtensionTrait: FormTrait + Wrapper<BaseType = FinalNode> {
    /// Get all the types of flags that this type of concept is predefined to potentially have.
    fn flags(&self) -> Vec<Archetype> {
        self.essence()
            .outgoing_nodes(HasProperty::TYPE_ID)
            .into_iter()
            .map(Archetype::from)
            .filter(|a| a.has_ancestor(Flag::archetype()))
            .collect()
    }

    /// Checks to see if this type of concept is predefined to have this as a flag.
    fn has_flag(&self, possible_type: Archetype) -> bool {
        self.essence()
            .has_outgoing(HasProperty::TYPE_ID, possible_type.essence())
    }

    /// Add a flag type to this archetype.
    fn add_flag(&mut self, attribute_type: Archetype) {
        self.essence_mut()
            .add_outgoing(HasProperty::TYPE_ID, attribute_type.essence());
    }

    /// Retrieve non-inherited flag types that are introduced by this archetype to all descendant
    /// archetypes. Flag types introduced by an ancestor do not count.
    fn added_flags(&self) -> Vec<Archetype> {
        self.essence()
            .base_wrapper()
            .outgoing_nodes(HasProperty::TYPE_ID)
            .into_iter()
            .map(|n| Archetype::from(n.id()))
            .collect()
    }
}

impl ArchetypeFormExtensionTrait for Archetype {}
impl ArchetypeFormExtensionTrait for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
    use crate::tao::form::Form;
    use crate::tao::initialize_kb;

    #[test]
    fn test_added_flags() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        assert_eq!(type1.added_flags(), Vec::<Archetype>::new());

        type1.add_flag(type2);
        assert_eq!(type1.added_flags(), vec!(type2));
    }

    #[test]
    fn test_flags_not_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_flag(type2);

        assert_eq!(type3.added_flags(), Vec::<Archetype>::new());
    }

    #[test]
    fn test_flags() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        type1.add_flag(type2);

        assert_eq!(type1.flags(), vec![type2]);
        assert!(!type1.has_flag(type1));
        assert!(type1.has_flag(type2));
    }

    #[test]
    fn test_flags_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_flag(type2);

        assert_eq!(type3.flags(), vec![type2]);
        assert!(!type3.has_flag(type1));
        assert!(type3.has_flag(type2));
    }
}
