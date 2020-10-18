use crate::concepts::attributes::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// The type of value this attribute has. Only the most restrictive inherited
/// value will be used.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueArchetype {
    base: Attribute,
}

impl Debug for ValueArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("ValueArchetype", self, f)
    }
}

impl From<usize> for ValueArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: Attribute::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for ValueArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Attribute::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for ValueArchetype {
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

impl<'a> ArchetypeTrait<'a, ValueArchetype> for ValueArchetype {
    const TYPE_ID: usize = 8;
    const TYPE_NAME: &'static str = "value-archetype";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for ValueArchetype {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.base.essence_mut()
    }
}

impl<'a> AttributeTrait<'a, ValueArchetype> for ValueArchetype {
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
        assert_eq!(ValueArchetype::archetype().id(), ValueArchetype::TYPE_ID);
        assert_eq!(
            ValueArchetype::archetype().internal_name(),
            Some(Rc::new(ValueArchetype::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = ValueArchetype::individuate();
        let concept_copy = ValueArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = ValueArchetype::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(ValueArchetype::try_from("A"), Ok(concept));
        assert!(ValueArchetype::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = ValueArchetype::individuate();
        let concept2 = ValueArchetype::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = ValueArchetype::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = ValueArchetype::individuate();
        let owner_of_owner = ValueArchetype::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = ValueArchetype::individuate();
        let value_of_owner = ValueArchetype::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
