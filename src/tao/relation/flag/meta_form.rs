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

/// Marks a property as meta.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Meta {
    base: FinalNode,
}

impl Debug for Meta {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Meta", self, f)
    }
}

impl From<usize> for Meta {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Meta {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Meta {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Meta {
    type ArchetypeForm = Archetype;
    type Form = Meta;

    const TYPE_ID: usize = 17;
    const TYPE_NAME: &'static str = "meta";
    const PARENT_TYPE_ID: usize = Flag::TYPE_ID;
}

impl Deref for Meta {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Meta {}

impl From<Meta> for Tao {
    fn from(this: Meta) -> Tao {
        Tao::from(this.base)
    }
}

impl From<Meta> for Relation {
    fn from(this: Meta) -> Relation {
        Relation::from(this.base)
    }
}

impl From<Meta> for Flag {
    fn from(this: Meta) -> Flag {
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
        assert_eq!(Meta::archetype().id(), Meta::TYPE_ID);
        assert_eq!(
            Meta::archetype().internal_name(),
            Some(Rc::from(Meta::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Meta::new();
        concept.set_internal_name("A");
        assert_eq!(Meta::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Meta::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Meta::archetype().added_attributes(), vec![]);
        assert_eq!(Meta::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Meta::new();
        let concept_copy = Meta::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Meta::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
