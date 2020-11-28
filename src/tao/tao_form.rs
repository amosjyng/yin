use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::Form;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// The root node of all knowledge.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tao {
    base: FinalNode,
}

impl Debug for Tao {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Tao", self, f)
    }
}

impl From<usize> for Tao {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Tao {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Tao {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Tao {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Tao {
    type ArchetypeForm = Archetype;
    type Form = Form;

    const TYPE_ID: usize = 0;
    const TYPE_NAME: &'static str = "tao";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
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
        assert_eq!(Tao::archetype().id(), Tao::TYPE_ID);
        assert_eq!(
            Tao::archetype().internal_name_str(),
            Some(Rc::from(Tao::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Tao::archetype().added_attributes(), vec![]);
        assert_eq!(Tao::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Tao::new();
        let concept_copy = Tao::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Tao::new();
        concept.set_internal_name_str("A");
        assert_eq!(Tao::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Tao::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Tao::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
