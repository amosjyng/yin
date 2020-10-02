use crate::concepts::{Archetype, ArchetypeTrait, FormTrait};
use crate::wrappers::{debug_wrapper, BaseWrapper, CommonNodeTrait};
use std::fmt::{Debug, Formatter, Result};
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
#[derive(Copy, Clone)]
pub struct Tao {
    base: BaseWrapper,
}

impl Debug for Tao {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Tao", Box::new(self), f)
    }
}

impl Eq for Tao {}

impl PartialEq for Tao {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<usize> for Tao {
    fn from(id: usize) -> Self {
        Tao {
            base: BaseWrapper::from(id),
        }
    }
}

impl From<BaseWrapper> for Tao {
    fn from(bw: BaseWrapper) -> Self {
        Tao { base: bw }
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

impl ArchetypeTrait<Tao> for Tao {
    const TYPE_ID: usize = 0;
    const TYPE_NAME: &'static str = "Tao";

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Tao {
            base: BaseWrapper::new(),
        }
    }
}

impl FormTrait for Tao {
    fn essence(&self) -> &BaseWrapper {
        &self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Tao::archetype().id(), Tao::TYPE_ID);
        assert_eq!(
            Tao::archetype().internal_name(),
            Some(Rc::new(Tao::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Tao::individuate();
        let concept2 = Tao::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Tao::individuate();
        let concept_copy = Tao::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Tao::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}