use crate::concepts::attributes::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// The value/target/to-node of an attribute.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    base: Attribute,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Value", self, f)
    }
}

impl From<usize> for Value {
    fn from(id: usize) -> Self {
        Self {
            base: Attribute::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for Value {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Attribute::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for Value {
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

impl<'a> ArchetypeTrait<'a, Value> for Value {
    const TYPE_ID: usize = 4;
    const TYPE_NAME: &'static str = "value";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Value {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.base.essence_mut()
    }
}

impl<'a> AttributeTrait<'a, Value> for Value {
    fn set_owner(&mut self, owner: &dyn FormTrait) {
        self.base.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.base.owner()
    }

    fn set_value(&mut self, value: &dyn FormTrait) {
        self.base.set_value(value);
    }

    fn value(&self) -> Option<Tao> {
        self.base.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Value::archetype().id(), Value::TYPE_ID);
        assert_eq!(
            Value::archetype().internal_name(),
            Some(Rc::new(Value::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Value::individuate();
        let concept_copy = Value::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Value::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Value::try_from("A"), Ok(concept));
        assert!(Value::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Value::individuate();
        let concept2 = Value::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Value::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Value::individuate();
        let owner_of_owner = Value::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Value::individuate();
        let value_of_owner = Value::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
