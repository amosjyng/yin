use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use crate::tao::form::{Form, FormTrait};
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// Represents patterns found across an entire class of concepts.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Archetype {
    base: FinalNode,
}

impl Debug for Archetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Archetype", self, f)
    }
}

impl From<usize> for Archetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Archetype {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Archetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for Archetype {
    type ArchetypeForm = Archetype;
    type Form = Archetype;

    const TYPE_ID: usize = 13;
    const TYPE_NAME: &'static str = "archetype";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl Deref for Archetype {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Archetype {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for Archetype {}

impl From<Archetype> for Tao {
    fn from(this: Archetype) -> Tao {
        Tao::from(this.base)
    }
}

impl ArchetypeFormTrait for Archetype {
    type SubjectForm = Form;
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
        assert_eq!(Archetype::archetype().id(), Archetype::TYPE_ID);
        assert_eq!(
            Archetype::archetype().internal_name(),
            Some(Rc::from(Archetype::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Archetype::new();
        concept.set_internal_name("A");
        assert_eq!(Archetype::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Archetype::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Archetype::archetype().added_attributes(), vec![]);
        assert_eq!(Archetype::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Archetype::new();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Archetype::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }
}
