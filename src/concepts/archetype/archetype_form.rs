use super::ArchetypeFormTrait;
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Represents an archetype from which various individual nodes can be derived.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Archetype {
    base: FinalNode,
}

impl Debug for Archetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Archetype", self, f)
    }
}

impl From<usize> for Archetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Archetype {
    fn from(fw: FinalNode) -> Self {
        Self { base: fw }
    }
}

impl<'a> TryFrom<&'a str> for Archetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl CommonNodeTrait for Archetype {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a, Archetype> for Archetype {
    const TYPE_ID: usize = 1;
    const TYPE_NAME: &'static str = "Archetype";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: FinalNode::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for Archetype {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeFormTrait<'a, Archetype, Tao> for Archetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::archetype::attribute::{AttributeArchetype, AttributeArchetypeTrait};
    use crate::concepts::attributes::{Attribute, Owner, Value};
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Archetype::archetype().id(), Archetype::TYPE_ID);
        assert_eq!(
            Archetype::archetype().internal_name(),
            Some(Rc::new(Archetype::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Archetype::individuate();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Archetype::try_from("A"), Ok(concept));
        assert!(Archetype::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Archetype::individuate();
        let concept2 = Archetype::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn test_individuation() {
        initialize_kb();
        let type1 = Owner::individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        assert!(type1.has_ancestor(Owner::archetype()));
        assert!(!type1.has_ancestor(Value::archetype()));
        assert!(type1_instance.has_ancestor(type1));
        assert!(type1_instance.has_ancestor(Owner::archetype()));
        assert!(!type1_instance.has_ancestor(Value::archetype()));
    }

    #[test]
    fn test_individuals() {
        initialize_kb();
        let type1 = Tao::individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        let type2_instance = type2.individuate_as_form();
        assert_eq!(type1.individuals(), vec![type1_instance, type2_instance]);
        assert_eq!(type2.individuals(), vec![type2_instance]);
    }

    #[test]
    fn test_individuals_not_self() {
        initialize_kb();
        let childless_type = Tao::individuate_as_archetype();
        assert_eq!(childless_type.individuals(), Vec::<Tao>::new())
    }

    #[test]
    fn test_child_archetypes() {
        initialize_kb();
        let type1 = Tao::individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        assert_eq!(type1.child_archetypes(), vec![type2, type3]);
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Tao::individuate_as_archetype();
        let type2 = Attribute::individuate_as_attribute_archetype();
        assert_eq!(
            type1.introduced_attribute_types(),
            Vec::<AttributeArchetype>::new()
        );

        type1.add_attribute_type(type2);
        assert_eq!(type1.introduced_attribute_types(), vec!(type2));
    }

    #[test]
    fn test_attribute_types_not_inherited() {
        initialize_kb();
        let mut type1 = Tao::individuate_as_archetype();
        let type2 = Attribute::individuate_as_attribute_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);

        assert_eq!(
            type3.introduced_attribute_types(),
            Vec::<AttributeArchetype>::new()
        );
    }
}
