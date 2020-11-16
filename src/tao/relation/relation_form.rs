use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::Tao;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Links any number of nodes together.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Relation {
    base: FinalNode,
}

impl Debug for Relation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Relation", self, f)
    }
}

impl From<usize> for Relation {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Relation {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Relation {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Relation {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Relation {
    type ArchetypeForm = Archetype;
    type Form = Relation;

    const TYPE_ID: usize = 11;
    const TYPE_NAME: &'static str = "relation";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Relation {}

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
        assert_eq!(Relation::archetype().id(), Relation::TYPE_ID);
        assert_eq!(
            Relation::archetype().internal_name(),
            Some(Rc::new(Relation::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        #[rustfmt::skip]
        assert_eq!(Relation::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Relation::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Relation::individuate();
        let concept_copy = Relation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Relation::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Relation::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Relation::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Relation::individuate();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
