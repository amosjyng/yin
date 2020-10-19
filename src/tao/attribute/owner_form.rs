use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::attribute::{Attribute, AttributeTrait};
use crate::tao::{Form, FormTrait};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// The owner/source/from-node of an attribute.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Owner {
    base: FinalNode,
}

impl Debug for Owner {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Owner", self, f)
    }
}

impl From<usize> for Owner {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Owner {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Owner {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
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

impl<'a> ArchetypeTrait<'a> for Owner {
    type ArchetypeForm = AttributeArchetype;
    type Form = Owner;

    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "owner";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for Owner {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl AttributeTrait for Owner {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::attribute::{Owner, Value};
    use crate::tao::{initialize_kb, Tao};

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
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Owner::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(Owner::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
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
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Owner::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            Owner::archetype().value_archetype(),
            Tao::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Owner::individuate();
        let owner_of_instance = Tao::individuate();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Owner::individuate();
        let value_of_instance = Tao::individuate();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
