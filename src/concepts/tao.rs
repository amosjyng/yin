use crate::concepts::{Archetype, ArchetypeTrait, FormTrait};
use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// The Tao that can be made into a `struct` is not the eternal Tao.
///
/// The name that can be put into a `String` is not the eternal name.
///
/// The unlabeled: ones and zeroes that ground this digital world.
///
/// The labels: documentation that forms ten thousand abstractions.
///
/// ***
///
/// (What's that, I could've just called this the "root" node? But where's the *fun* in that? Next
/// you're going to tell me not to GPL this motherfucker.)
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
    fn from(bw: FinalNode) -> Self {
        Self { base: bw }
    }
}

impl<'a> TryFrom<&'a str> for Tao {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl CommonNodeTrait for Tao {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a, Tao> for Tao {
    const TYPE_ID: usize = 0;
    const TYPE_NAME: &'static str = "Tao";
    // It seems fitting, albeit meaningless, to make the Tao inherit its own properties.
    const PARENT_TYPE_ID: usize = Self::TYPE_ID;

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: FinalNode::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for Tao {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Tao::archetype().id(), Tao::TYPE_ID);
        assert_eq!(
            Tao::archetype().internal_name(),
            Some(Rc::new(Tao::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Tao::individuate();
        let concept_copy = Tao::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Tao::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Tao::try_from("A"), Ok(concept));
        assert!(Tao::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Tao::individuate();
        let concept2 = Tao::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Tao::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
