use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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

impl ArchetypeTrait for Flag {
    type ArchetypeForm = Archetype;
    type Form = Flag;

    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "flag";
    const PARENT_TYPE_ID: usize = Relation::TYPE_ID;
}

impl Deref for Flag {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Flag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Flag {}

impl From<Flag> for Tao {
    fn from(this: Flag) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Flag> for Relation {
    fn from(this: Flag) -> Relation {
        Relation::from(this.base)
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
        assert_eq!(Flag::archetype().id(), Flag::TYPE_ID);
        assert_eq!(
            Flag::archetype().internal_name(),
            Some(Rc::from(Flag::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Flag::new();
        concept.set_internal_name("A");
        assert_eq!(Flag::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Flag::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Flag::archetype().added_attributes(), vec![]);
        assert_eq!(Flag::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Flag::new();
        let concept_copy = Flag::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Flag::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
