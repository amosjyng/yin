use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::attribute::{Attribute, AttributeTrait};
use crate::tao::{Form, FormTrait};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Describes instances of an archetype as having certain types of attributes.
///
/// For example, a string may have a length of 5. But on a more meta level, that
/// means that the string has a length property or length "attribute". That's
/// where this attribute comes in.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasAttributeType {
    base: FinalNode,
}

impl Debug for HasAttributeType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasAttributeType", self, f)
    }
}

impl From<usize> for HasAttributeType {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for HasAttributeType {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for HasAttributeType {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl CommonNodeTrait for HasAttributeType {
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

impl<'a> ArchetypeTrait<'a> for HasAttributeType {
    type ArchetypeForm = AttributeArchetype;
    type Form = HasAttributeType;

    const TYPE_ID: usize = 6;
    const TYPE_NAME: &'static str = "has-attribute-type";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl FormTrait for HasAttributeType {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl AttributeTrait for HasAttributeType {
    type OwnerForm = Form;
    type ValueForm = Attribute;
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
        #[rustfmt::skip]
        assert_eq!(HasAttributeType::archetype().id(), HasAttributeType::TYPE_ID);
        assert_eq!(
            HasAttributeType::archetype().internal_name(),
            Some(Rc::new(HasAttributeType::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(HasAttributeType::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(HasAttributeType::archetype().attribute_archetypes(), vec![Owner::archetype(), Value::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasAttributeType::individuate();
        let concept_copy = HasAttributeType::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasAttributeType::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(HasAttributeType::try_from("A"), Ok(concept));
        assert!(HasAttributeType::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = HasAttributeType::individuate();
        let concept2 = HasAttributeType::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = HasAttributeType::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            HasAttributeType::archetype().owner_archetype(),
            Tao::archetype().as_archetype()
        );
        assert_eq!(
            HasAttributeType::archetype().value_archetype(),
            Attribute::archetype().as_archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasAttributeType::individuate();
        let owner_of_instance = Tao::individuate();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasAttributeType::individuate();
        let value_of_instance = Attribute::individuate();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
