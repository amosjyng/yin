use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::tao::relation::Relation;
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The type of owner this attribute has. Only the most restrictive inherited
/// value will be used.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OwnerArchetype {
    base: FinalNode,
}

impl Debug for OwnerArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("OwnerArchetype", self, f)
    }
}

impl From<usize> for OwnerArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for OwnerArchetype {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for OwnerArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for OwnerArchetype {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for OwnerArchetype {
    type ArchetypeForm = AttributeArchetype;
    type Form = OwnerArchetype;

    const TYPE_ID: usize = 11;
    const TYPE_NAME: &'static str = "owner-archetype";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for OwnerArchetype {}

impl From<OwnerArchetype> for Attribute {
    fn from(this: OwnerArchetype) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for OwnerArchetype {
    type OwnerForm = Relation;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use crate::tao::relation::attribute::{Owner, Value};
    use crate::tao::{initialize_kb, Tao};
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(OwnerArchetype::archetype().id(), OwnerArchetype::TYPE_ID);
        assert_eq!(
            OwnerArchetype::archetype().internal_name_str(),
            Some(Rc::from(OwnerArchetype::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = OwnerArchetype::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            OwnerArchetype::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(OwnerArchetype::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(OwnerArchetype::archetype().added_attributes(), vec![]);
        assert_eq!(
            OwnerArchetype::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = OwnerArchetype::new();
        let concept_copy = OwnerArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = OwnerArchetype::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            OwnerArchetype::archetype().owner_archetype(),
            Relation::archetype()
        );
        assert_eq!(
            OwnerArchetype::archetype().value_archetype(),
            Tao::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = OwnerArchetype::new();
        let owner_of_instance = Relation::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = OwnerArchetype::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
