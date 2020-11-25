use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::has_property::HasProperty;
use crate::tao::relation::attribute::AttributeTrait;
use crate::tao::relation::Relation;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Describes instances of an archetype as generally having values set for this
/// flag. Does not describe whether the value for the flag is true or false.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasFlag {
    base: FinalNode,
}

impl Debug for HasFlag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasFlag", self, f)
    }
}

impl From<usize> for HasFlag {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for HasFlag {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for HasFlag {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for HasFlag {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for HasFlag {
    type ArchetypeForm = AttributeArchetype;
    type Form = HasFlag;

    const TYPE_ID: usize = 9;
    const TYPE_NAME: &'static str = "has-flag";
    const PARENT_TYPE_ID: usize = HasProperty::TYPE_ID;
}

impl FormTrait for HasFlag {}

impl AttributeTrait for HasFlag {
    type OwnerForm = Form;
    type ValueForm = Relation;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    #[rustfmt::skip]
    use crate::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use crate::tao::form::FormTrait;
    use crate::tao::relation::attribute::{Owner, Value};
    use crate::tao::{initialize_kb, Tao};
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(HasFlag::archetype().id(), HasFlag::TYPE_ID);
        assert_eq!(
            HasFlag::archetype().internal_name_str(),
            Some(Rc::from(HasFlag::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(HasFlag::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(HasFlag::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasFlag::new();
        let concept_copy = HasFlag::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasFlag::new();
        concept.set_internal_name_str("A");
        assert_eq!(HasFlag::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(HasFlag::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = HasFlag::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            HasFlag::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            HasFlag::archetype().value_archetype(),
            Relation::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasFlag::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasFlag::new();
        let value_of_instance = Relation::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
