use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::flag::Flag;
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Marks a property as not behing inherited.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nonhereditary {
    base: FinalNode,
}

impl Debug for Nonhereditary {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Nonhereditary", self, f)
    }
}

impl From<usize> for Nonhereditary {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Nonhereditary {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Nonhereditary {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Nonhereditary {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Nonhereditary {
    type ArchetypeForm = Archetype;
    type Form = Nonhereditary;

    const TYPE_ID: usize = 16;
    const TYPE_NAME: &'static str = "nonhereditary";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl FormTrait for Nonhereditary {}

impl From<Nonhereditary> for Flag {
    fn from(this: Nonhereditary) -> Flag {
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
        assert_eq!(Nonhereditary::archetype().id(), Nonhereditary::TYPE_ID);
        assert_eq!(
            Nonhereditary::archetype().internal_name_str(),
            Some(Rc::from(Nonhereditary::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Nonhereditary::new();
        concept.set_internal_name_str("A");
        assert_eq!(
            Nonhereditary::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(Nonhereditary::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Nonhereditary::archetype().added_attributes(), vec![]);
        assert_eq!(
            Nonhereditary::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Nonhereditary::new();
        let concept_copy = Nonhereditary::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Nonhereditary::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
