use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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

impl ArchetypeTrait for Owner {
    type ArchetypeForm = AttributeArchetype;
    type Form = Owner;

    const TYPE_ID: usize = 5;
    const TYPE_NAME: &'static str = "owner";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for Owner {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Owner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Owner {}

impl From<Owner> for Tao {
    fn from(this: Owner) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Owner> for Relation {
    fn from(this: Owner) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Owner> for Attribute {
    fn from(this: Owner) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for Owner {
    type OwnerForm = Relation;
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
        assert_eq!(Owner::archetype().id(), Owner::TYPE_ID);
        assert_eq!(
            Owner::archetype().internal_name(),
            Some(Rc::from(Owner::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Owner::new();
        concept.set_internal_name("A");
        assert_eq!(Owner::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Owner::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Owner::archetype().added_attributes(), vec![]);
        assert_eq!(
            Owner::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Owner::new();
        let concept_copy = Owner::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Owner::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            Owner::archetype().owner_archetype(),
            Relation::archetype().into()
        );
        assert_eq!(
            Owner::archetype().value_archetype(),
            Tao::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = Owner::new();
        let owner_of_instance = Relation::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = Owner::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
