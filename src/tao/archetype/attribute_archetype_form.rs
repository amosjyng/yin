use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::Attribute;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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

impl ArchetypeTrait for AttributeArchetype {
    type ArchetypeForm = Archetype;
    type Form = AttributeArchetype;

    const TYPE_ID: usize = 14;
    const TYPE_NAME: &'static str = "attribute-archetype";
    const PARENT_TYPE_ID: usize = Archetype::TYPE_ID;
}

impl Deref for AttributeArchetype {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for AttributeArchetype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for AttributeArchetype {}

impl From<AttributeArchetype> for Tao {
    fn from(this: AttributeArchetype) -> Tao {
        Tao::from(this.base)
    }
}

impl From<AttributeArchetype> for Archetype {
    fn from(this: AttributeArchetype) -> Archetype {
        Archetype::from(this.base)
    }
}

impl ArchetypeFormTrait for AttributeArchetype {
    type SubjectForm = Attribute;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(
            AttributeArchetype::archetype().id(),
            AttributeArchetype::TYPE_ID
        );
        assert_eq!(
            AttributeArchetype::archetype().internal_name(),
            Some(Rc::from(AttributeArchetype::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = AttributeArchetype::new();
        concept.set_internal_name("A");
        assert_eq!(
            AttributeArchetype::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(AttributeArchetype::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(AttributeArchetype::archetype().added_attributes(), vec![]);
        assert_eq!(AttributeArchetype::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = AttributeArchetype::new();
        let concept_copy = AttributeArchetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = AttributeArchetype::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
