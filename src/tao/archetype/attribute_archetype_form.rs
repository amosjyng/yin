use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Archetype representing attributes.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeArchetype {
    base: FinalNode,
}

impl Debug for AttributeArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("AttributeArchetype", self, f)
    }
}

impl From<usize> for AttributeArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for AttributeArchetype {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for AttributeArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for AttributeArchetype {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for AttributeArchetype {
    type ArchetypeForm = Archetype;
    type Form = AttributeArchetype;

    const TYPE_ID: usize = 12;
    const TYPE_NAME: &'static str = "attribute-archetype";
    const PARENT_TYPE_ID: usize = Archetype::TYPE_ID;
}

impl FormTrait for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::form::FormTrait;
    use crate::tao::initialize_kb;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(AttributeArchetype::archetype().id(), AttributeArchetype::TYPE_ID);
        assert_eq!(
            AttributeArchetype::archetype().internal_name_str(),
            Some(Rc::from(AttributeArchetype::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(AttributeArchetype::archetype().introduced_attribute_archetypes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(AttributeArchetype::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = AttributeArchetype::new();
        let concept_copy = AttributeArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = AttributeArchetype::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(AttributeArchetype::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(AttributeArchetype::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = AttributeArchetype::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
