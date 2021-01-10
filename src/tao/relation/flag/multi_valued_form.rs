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

/// Marks an attribute as having multiple possible values for the same owner.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiValued {
    base: FinalNode,
}

impl Debug for MultiValued {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("MultiValued", self, f)
    }
}

impl From<usize> for MultiValued {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for MultiValued {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for MultiValued {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for MultiValued {
    type ArchetypeForm = Archetype;
    type Form = MultiValued;

    const TYPE_ID: usize = 18;
    const TYPE_NAME: &'static str = "multi-valued";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for MultiValued {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MultiValued {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for MultiValued {}

impl From<MultiValued> for Tao {
    fn from(this: MultiValued) -> Tao {
        Tao::from(this.base)
    }
}

impl From<MultiValued> for Relation {
    fn from(this: MultiValued) -> Relation {
        Relation::from(this.base)
    }
}

impl From<MultiValued> for Flag {
    fn from(this: MultiValued) -> Flag {
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
        assert_eq!(MultiValued::archetype().id(), MultiValued::TYPE_ID);
        assert_eq!(
            MultiValued::archetype().internal_name(),
            Some(Rc::from(MultiValued::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = MultiValued::new();
        concept.set_internal_name("A");
        assert_eq!(MultiValued::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(MultiValued::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(MultiValued::archetype().added_attributes(), vec![]);
        assert_eq!(
            MultiValued::archetype().attributes(),
            vec![Owner::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = MultiValued::new();
        let concept_copy = MultiValued::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = MultiValued::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
