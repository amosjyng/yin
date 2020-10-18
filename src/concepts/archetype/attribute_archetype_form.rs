use super::{Archetype, ArchetypeFormTrait};
use crate::concepts::attributes::Attribute;
use crate::concepts::{ArchetypeTrait, FormTrait};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Represents an archetype representing attributes.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeArchetype {
    base: FinalNode,
}

impl Debug for AttributeArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("AttributeArchetype", self, f)
    }
}

impl From<usize> for AttributeArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for AttributeArchetype {
    fn from(fw: FinalNode) -> Self {
        Self { base: fw }
    }
}

impl<'a> TryFrom<&'a str> for AttributeArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl CommonNodeTrait for AttributeArchetype {
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

impl<'a> ArchetypeTrait<'a, AttributeArchetype> for AttributeArchetype {
    const TYPE_ID: usize = 9;
    const TYPE_NAME: &'static str = "AttributeArchetype";
    const PARENT_TYPE_ID: usize = Archetype::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: FinalNode::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for AttributeArchetype {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeFormTrait<'a, AttributeArchetype, Attribute> for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::{Owner, Value};
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
        let type1 = Attribute::individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        let type2_instance = type2.individuate_as_form();
        assert_eq!(type1.individuals(), vec![type1_instance, type2_instance]);
        assert_eq!(type2.individuals(), vec![type2_instance]);
    }

    #[test]
    fn test_individuals_not_self() {
        initialize_kb();
        let childless_type =
            AttributeArchetype::from(*Attribute::individuate_as_archetype().essence());
        assert_eq!(childless_type.individuals(), Vec::<Attribute>::new())
    }

    #[test]
    fn test_child_archetypes() {
        initialize_kb();
        let type1 = Attribute::individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        assert_eq!(type1.child_archetypes(), vec![type2, type3]);
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Attribute::individuate_as_archetype();
        let type2 = Attribute::individuate_as_archetype();
        assert_eq!(type1.introduced_attribute_types(), Vec::<Archetype>::new());

        type1.add_attribute_type(type2);
        assert_eq!(type1.introduced_attribute_types(), vec!(type2));
    }

    #[test]
    fn test_attribute_types_not_inherited() {
        initialize_kb();
        let mut type1 = Attribute::individuate_as_archetype();
        let type2 = Attribute::individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);

        assert_eq!(type3.introduced_attribute_types(), Vec::<Archetype>::new());
    }
}
