use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::tao::relation::Relation;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Describes instances of an archetype as having certain other properties.
///
/// For example, a string may have a length of 5. But on a more meta level, that
/// means that the string has a length property or length "attribute". That's
/// where this attribute comes in.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasProperty {
    base: FinalNode,
}

impl Debug for HasProperty {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasProperty", self, f)
    }
}

impl From<usize> for HasProperty {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for HasProperty {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for HasProperty {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for HasProperty {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for HasProperty {
    type ArchetypeForm = AttributeArchetype;
    type Form = HasProperty;

    const TYPE_ID: usize = 8;
    const TYPE_NAME: &'static str = "has-property";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for HasProperty {}

impl AttributeTrait for HasProperty {
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
        assert_eq!(HasProperty::archetype().id(), HasProperty::TYPE_ID);
        assert_eq!(
            HasProperty::archetype().internal_name_str(),
            Some(Rc::from(HasProperty::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(HasProperty::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(HasProperty::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasProperty::new();
        let concept_copy = HasProperty::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasProperty::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(HasProperty::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(HasProperty::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = HasProperty::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            HasProperty::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            HasProperty::archetype().value_archetype(),
            Relation::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasProperty::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasProperty::new();
        let value_of_instance = Relation::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
