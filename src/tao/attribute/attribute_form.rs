use super::AttributeTrait;
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::{FormTrait, Tao};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Represents either a unary or binary relation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute {
    /// Wrapper that this abstraction is based on.
    pub base: FinalNode,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Attribute", self, f)
    }
}

impl From<usize> for Attribute {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Attribute {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Attribute {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl CommonNodeTrait for Attribute {
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

impl<'a> ArchetypeTrait<'a> for Attribute {
    type ArchetypeForm = AttributeArchetype;
    type Form = Attribute;

    const TYPE_ID: usize = 2;
    const TYPE_NAME: &'static str = "Attribute";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Attribute {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl AttributeTrait for Attribute {
    type OwnerForm = Tao;
    type ValueForm = Tao;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::{ArchetypeFormTrait, AttributeArchetype};
    use crate::tao::attribute::{Owner, Value};
    use crate::tao::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Attribute::archetype().id(), Attribute::TYPE_ID);
        assert_eq!(
            Attribute::archetype().internal_name(),
            Some(Rc::new(Attribute::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_attributes_specified() {
        initialize_kb();
        // todo: use ::attribute() after yang generates that
        assert_eq!(
            Attribute::archetype().introduced_attribute_types(),
            vec![
                AttributeArchetype::from(Owner::TYPE_ID),
                AttributeArchetype::from(Value::TYPE_ID)
            ]
        );
        assert_eq!(
            Attribute::archetype().attribute_archetypes(),
            vec![
                AttributeArchetype::from(Owner::TYPE_ID),
                AttributeArchetype::from(Value::TYPE_ID)
            ]
        );
        assert_eq!(Attribute::archetype().owner_archetype(), Tao::archetype());
        assert_eq!(Attribute::archetype().value_archetype(), Tao::archetype());
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Attribute::individuate();
        let concept_copy = Attribute::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Attribute::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Attribute::try_from("A"), Ok(concept));
        assert!(Attribute::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Attribute::individuate();
        let concept2 = Attribute::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Attribute::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut attr_instance = Attribute::individuate();
        let owner_of_attr = Attribute::individuate();
        attr_instance.set_owner(&owner_of_attr.ego_death());
        assert_eq!(attr_instance.owner(), Some(owner_of_attr.ego_death()));
        assert_eq!(attr_instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut attr_instance = Attribute::individuate();
        let value_of_attr = Attribute::individuate();
        attr_instance.set_value(&value_of_attr.ego_death());
        assert_eq!(attr_instance.owner(), None);
        assert_eq!(attr_instance.value(), Some(value_of_attr.ego_death()));
    }
}
