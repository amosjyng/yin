use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::has_property::HasProperty;
use crate::tao::relation::attribute::AttributeTrait;
use crate::tao::relation::Relation;
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Describes instances of an archetype as generally having values set for this
/// attribute.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct HasAttribute {
    base: FinalNode,
}

impl Debug for HasAttribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("HasAttribute", self, f)
    }
}

impl From<usize> for HasAttribute {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for HasAttribute {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for HasAttribute {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for HasAttribute {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for HasAttribute {
    type ArchetypeForm = AttributeArchetype;
    type Form = HasAttribute;

    const TYPE_ID: usize = 10;
    const TYPE_NAME: &'static str = "has-attribute";
    const PARENT_TYPE_ID: usize = HasProperty::TYPE_ID;
}

impl FormTrait for HasAttribute {}

impl From<HasAttribute> for HasProperty {
    fn from(this: HasAttribute) -> HasProperty {
        HasProperty::from(this.base)
    }
}

impl AttributeTrait for HasAttribute {
    type OwnerForm = Form;
    type ValueForm = Relation;
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
        assert_eq!(HasAttribute::archetype().id(), HasAttribute::TYPE_ID);
        assert_eq!(
            HasAttribute::archetype().internal_name_str(),
            Some(Rc::from(HasAttribute::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = HasAttribute::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            HasAttribute::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(HasAttribute::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(HasAttribute::archetype().added_attributes(), vec![]);
        assert_eq!(
            HasAttribute::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = HasAttribute::new();
        let concept_copy = HasAttribute::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = HasAttribute::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }

    #[test]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            HasAttribute::archetype().owner_archetype(),
            Tao::archetype()
        );
        assert_eq!(
            HasAttribute::archetype().value_archetype(),
            Relation::archetype()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = HasAttribute::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = HasAttribute::new();
        let value_of_instance = Relation::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
