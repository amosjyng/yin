use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The default value of a data structure.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultValue {
    base: FinalNode,
}

impl Debug for DefaultValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("DefaultValue", self, f)
    }
}

impl From<usize> for DefaultValue {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for DefaultValue {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for DefaultValue {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for DefaultValue {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for DefaultValue {
    type ArchetypeForm = AttributeArchetype;
    type Form = DefaultValue;

    const TYPE_ID: usize = 23;
    const TYPE_NAME: &'static str = "default-value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for DefaultValue {}

impl From<DefaultValue> for Attribute {
    fn from(this: DefaultValue) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for DefaultValue {
    type OwnerForm = Form;
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
        assert_eq!(DefaultValue::archetype().id(), DefaultValue::TYPE_ID);
        assert_eq!(
            DefaultValue::archetype().internal_name_str(),
            Some(Rc::from(DefaultValue::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = DefaultValue::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            DefaultValue::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(DefaultValue::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(DefaultValue::archetype().added_attributes(), vec![]);
        assert_eq!(
            DefaultValue::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = DefaultValue::new();
        let concept_copy = DefaultValue::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = DefaultValue::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            DefaultValue::archetype().owner_archetype(),
            Tao::archetype()
        );
        assert_eq!(
            DefaultValue::archetype().value_archetype(),
            Tao::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = DefaultValue::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = DefaultValue::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
