use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::AttributeTrait;
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// Represents a binary relation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute {
    base: FinalNode,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Attribute", self, f)
    }
}

impl From<usize> for Attribute {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Attribute {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Attribute {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Attribute {
    type ArchetypeForm = AttributeArchetype;
    type Form = Attribute;

    const TYPE_ID: usize = 4;
    const TYPE_NAME: &'static str = "attribute";
    const PARENT_TYPE_ID: usize = Relation::TYPE_ID;
}

impl Deref for Attribute {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Attribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Attribute {}

impl From<Attribute> for Tao {
    fn from(this: Attribute) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Attribute> for Relation {
    fn from(this: Attribute) -> Relation {
        Relation::from(this.base)
    }
}

impl AttributeTrait for Attribute {
    type OwnerForm = Form;
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
        assert_eq!(Attribute::archetype().id(), Attribute::TYPE_ID);
        assert_eq!(
            Attribute::archetype().internal_name(),
            Some(Rc::from(Attribute::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Attribute::new();
        concept.set_internal_name("A");
        assert_eq!(Attribute::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Attribute::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Attribute::archetype().added_attributes(),
            vec![Value::archetype()]
        );
        assert_eq!(
            Attribute::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Attribute::new();
        let concept_copy = Attribute::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Attribute::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Attribute::archetype().owner_archetype(),
            Tao::archetype().into()
        );
        assert_eq!(
            Attribute::archetype().value_archetype(),
            Tao::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Attribute::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Attribute::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
