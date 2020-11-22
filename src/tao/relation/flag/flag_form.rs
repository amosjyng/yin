use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::Relation;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Represents a unary relation.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flag {
    base: FinalNode,
}

impl Debug for Flag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Flag", self, f)
    }
}

impl From<usize> for Flag {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Flag {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Flag {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Flag {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Flag {
    type ArchetypeForm = Archetype;
    type Form = Flag;

    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "flag";
    const PARENT_TYPE_ID: usize = Relation::TYPE_ID;
}

impl FormTrait for Flag {}

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
        assert_eq!(Flag::archetype().id(), Flag::TYPE_ID);
        assert_eq!(
            Flag::archetype().internal_name_str(),
            Some(Rc::from(Flag::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Flag::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Flag::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Flag::new();
        let concept_copy = Flag::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Flag::new();
        concept.set_internal_name_str("A");
        assert_eq!(Flag::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Flag::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Flag::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
