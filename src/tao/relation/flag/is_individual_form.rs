use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::flag::Flag;
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Whether or not a concept is an individual, as opposed to an archetype.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsIndividual {
    base: FinalNode,
}

impl Debug for IsIndividual {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("IsIndividual", self, f)
    }
}

impl From<usize> for IsIndividual {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for IsIndividual {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for IsIndividual {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for IsIndividual {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for IsIndividual {
    type ArchetypeForm = Archetype;
    type Form = IsIndividual;

    const TYPE_ID: usize = 16;
    const TYPE_NAME: &'static str = "is-individual";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for IsIndividual {}

impl From<IsIndividual> for Flag {
    fn from(this: IsIndividual) -> Flag {
        Flag::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Owner;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(IsIndividual::archetype().id(), IsIndividual::TYPE_ID);
        assert_eq!(
            IsIndividual::archetype().internal_name_str(),
            Some(Rc::from(IsIndividual::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(IsIndividual::archetype().added_attributes(), vec![]);
        #[rustfmt::skip]
        assert_eq!(IsIndividual::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = IsIndividual::new();
        let concept_copy = IsIndividual::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = IsIndividual::new();
        concept.set_internal_name_str("A");
        #[rustfmt::skip]
        assert_eq!(IsIndividual::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(IsIndividual::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = IsIndividual::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
