use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// The value/target/to-node of an attribute.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    base: FinalNode,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Value", self, f)
    }
}

impl From<usize> for Value {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Value {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Value {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Value {
    type ArchetypeForm = AttributeArchetype;
    type Form = Value;

    const TYPE_ID: usize = 6;
    const TYPE_NAME: &'static str = "value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for Value {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Value {}

impl From<Value> for Tao {
    fn from(this: Value) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Value> for Relation {
    fn from(this: Value) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Value> for Attribute {
    fn from(this: Value) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for Value {
    type OwnerForm = Attribute;
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
        assert_eq!(Value::archetype().id(), Value::TYPE_ID);
        assert_eq!(
            Value::archetype().internal_name(),
            Some(Rc::from(Value::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Value::new();
        concept.set_internal_name("A");
        assert_eq!(Value::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Value::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Value::archetype().added_attributes(), vec![]);
        assert_eq!(
            Value::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Value::new();
        let concept_copy = Value::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Value::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Value::archetype().owner_archetype(),
            Attribute::archetype().into()
        );
        assert_eq!(
            Value::archetype().value_archetype(),
            Tao::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Value::new();
        let owner_of_instance = Attribute::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Value::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
