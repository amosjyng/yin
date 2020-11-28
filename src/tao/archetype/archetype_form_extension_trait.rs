use super::Archetype;
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::has_property::HasFlag;
use crate::Wrapper;

/// Public trait to store eventually-automated archetype attributes in.
pub trait ArchetypeFormExtensionTrait: FormTrait + Wrapper<BaseType = FinalNode> {
    /// Get all the types of flags that this type of concept is predefined to potentially have.
    fn flags(&self) -> Vec<Archetype> {
        self.essence()
            .outgoing_nodes(HasFlag::TYPE_ID)
            .into_iter()
            .map(Archetype::from)
            .collect()
    }

    /// Checks to see if this type of concept is predefined to have this as a flag.
    fn has_flag(&self, possible_type: Archetype) -> bool {
        self.essence()
            .has_outgoing(HasFlag::TYPE_ID, possible_type.essence())
    }

    /// Add a flag type to this archetype.
    fn add_flag(&mut self, attribute_type: Archetype) {
        self.essence_mut()
            .add_outgoing(HasFlag::TYPE_ID, attribute_type.essence());
    }

    /// Retrieve non-inherited flag types that are introduced by this archetype to all descendant
    /// archetypes. Flag types introduced by an ancestor do not count.
    fn added_flags(&self) -> Vec<Archetype> {
        self.essence()
            .base_wrapper()
            .outgoing_nodes(HasFlag::TYPE_ID)
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
    use crate::tao::relation::attribute::Attribute;
    use crate::tao::relation::flag::Flag;
    use crate::tao::Tao;

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
    fn test_added_flags_not_inherited() {
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
    fn test_flag_equivalents() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Tao::archetype().individuate_as_archetype();
        type1.add_flag(type2);

        assert_eq!(type1.flags(), vec![type2]);
        assert_eq!(type1.added_flags(), vec![type2]);
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

    #[test]
    fn test_flags_no_attributes() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let flag_type = Flag::archetype().individuate_as_archetype();
        let attr_type = Attribute::archetype().individuate_as_archetype();
        form_type.add_flag(flag_type);
        form_type.add_attribute(attr_type);

        assert_eq!(form_type.flags(), vec![flag_type]);
        assert_eq!(form_type.added_flags(), vec![flag_type]);
    }
}
