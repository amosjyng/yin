use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

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

impl ArchetypeTrait for Relation {
    type ArchetypeForm = Archetype;
    type Form = Relation;

    const TYPE_ID: usize = 2;
    const TYPE_NAME: &'static str = "relation";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl Deref for Relation {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Relation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Relation {}

impl From<Relation> for Tao {
    fn from(this: Relation) -> Tao {
        Tao::from(this.base)
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
        assert_eq!(Relation::archetype().id(), Relation::TYPE_ID);
        assert_eq!(
            Relation::archetype().internal_name(),
            Some(Rc::from(Relation::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Relation::new();
        concept.set_internal_name("A");
        assert_eq!(Relation::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Relation::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(
            Relation::archetype().added_attributes(),
            vec![Owner::archetype()]
        );
        assert_eq!(Relation::archetype().attributes(), vec![Owner::archetype()]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Relation::new();
        let concept_copy = Relation::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Relation::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
