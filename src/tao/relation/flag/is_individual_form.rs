use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::flag::Flag;
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// Whether or not a concept is an individual, as opposed to an archetype.
///
/// Marking a concept as an individual will cause it to be filtered out from the
/// `parents` and `child_archetypes` functions.
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

impl ArchetypeTrait for IsIndividual {
    type ArchetypeForm = Archetype;
    type Form = IsIndividual;

    const TYPE_ID: usize = 19;
    const TYPE_NAME: &'static str = "is-individual";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for IsIndividual {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for IsIndividual {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for IsIndividual {}

impl From<IsIndividual> for Tao {
    fn from(this: IsIndividual) -> Tao {
        Tao::from(this.base)
    }
}

impl From<IsIndividual> for Relation {
    fn from(this: IsIndividual) -> Relation {
        Relation::from(this.base)
    }
}

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
            IsIndividual::archetype().internal_name(),
            Some(Rc::from(IsIndividual::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = IsIndividual::new();
        concept.set_internal_name("A");
        assert_eq!(
            IsIndividual::try_from("A").map(|c| c.id()),
            Ok(concept.id())
        );
        assert!(IsIndividual::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(IsIndividual::archetype().added_attributes(), vec![]);
        assert_eq!(
            IsIndividual::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = IsIndividual::new();
        let concept_copy = IsIndividual::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = IsIndividual::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
