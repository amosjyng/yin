use crate::concepts::attributes::{Attribute, AttributeTrait};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// The owner/source/from-node of an attribute.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Owner {
    base: Attribute,
}

impl Debug for Owner {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Owner", self, f)
    }
}

impl From<usize> for Owner {
    fn from(id: usize) -> Self {
        Self {
            base: Attribute::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for Owner {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        Attribute::try_from(name).map(|a| Self { base: a })
    }
}

impl CommonNodeTrait for Owner {
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

impl<'a> ArchetypeTrait<'a, Owner> for Owner {
    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "owner";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: Attribute::individuate_with_parent(parent_id),
        }
    }
}

impl FormTrait for Owner {
    fn essence(&self) -> &FinalNode {
        self.base.essence()
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        self.base.essence_mut()
    }
}

impl<'a> AttributeTrait<'a, Owner> for Owner {
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
        assert_eq!(Owner::archetype().id(), Owner::TYPE_ID);
        assert_eq!(
            Owner::archetype().internal_name(),
            Some(Rc::new(Owner::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Owner::individuate();
        let concept_copy = Owner::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Owner::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Owner::try_from("A"), Ok(concept));
        assert!(Owner::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Owner::individuate();
        let concept2 = Owner::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Owner::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Owner::individuate();
        let owner_of_owner = Owner::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Owner::individuate();
        let value_of_owner = Owner::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }
}
