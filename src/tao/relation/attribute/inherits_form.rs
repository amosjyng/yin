use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Describes the owner as inheriting all attributes of the value.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inherits {
    base: FinalNode,
}

impl Debug for Inherits {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Inherits", self, f)
    }
}

impl From<usize> for Inherits {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Inherits {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Inherits {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Inherits {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Inherits {
    type ArchetypeForm = AttributeArchetype;
    type Form = Inherits;

    const TYPE_ID: usize = 5;
    const TYPE_NAME: &'static str = "inherits";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for Inherits {}

impl AttributeTrait for Inherits {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::form::FormTrait;
    use crate::tao::relation::attribute::{Owner, Value};
    use crate::tao::{initialize_kb, Tao};
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Inherits::archetype().id(), Inherits::TYPE_ID);
        assert_eq!(
            Inherits::archetype().internal_name(),
            Some(Rc::new(Inherits::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Inherits::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(Inherits::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Inherits::individuate();
        let concept_copy = Inherits::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Inherits::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Inherits::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Inherits::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Inherits::individuate();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Inherits::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            Inherits::archetype().value_archetype(),
            Tao::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Inherits::individuate();
        let owner_of_instance = Tao::individuate();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Inherits::individuate();
        let value_of_instance = Tao::individuate();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
