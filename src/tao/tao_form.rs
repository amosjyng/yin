use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::Form;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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

impl ArchetypeTrait for Tao {
    type ArchetypeForm = Archetype;
    type Form = Form;

    const TYPE_ID: usize = 0;
    const TYPE_NAME: &'static str = "tao";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl Deref for Tao {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Tao {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
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
            Tao::archetype().internal_name(),
            Some(Rc::from(Tao::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Tao::new();
        concept.set_internal_name("A");
        assert_eq!(Tao::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Tao::try_from("B").is_err());
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
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Tao::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
