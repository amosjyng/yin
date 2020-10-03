use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, CommonNodeTrait, FinalWrapper};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Represents an archetype from which various individual nodes can be derived.
#[derive(Copy, Clone)]
pub struct Archetype {
    base: FinalWrapper,
}

impl Debug for Archetype {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Archetype", Box::new(self), f)
    }
}

impl Eq for Archetype {}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<usize> for Archetype {
    fn from(id: usize) -> Self {
        Archetype {
            base: FinalWrapper::from(id),
        }
    }
}

impl From<FinalWrapper> for Archetype {
    fn from(fw: FinalWrapper) -> Self {
        Archetype { base: fw }
    }
}

impl CommonNodeTrait for Archetype {
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

impl ArchetypeTrait<Archetype> for Archetype {
    const TYPE_ID: usize = 1;
    const TYPE_NAME: &'static str = "Archetype";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Archetype {
            base: FinalWrapper::new(),
        }
    }
}

impl FormTrait for Archetype {
    fn essence(&self) -> &FinalWrapper {
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
        assert_eq!(Archetype::archetype().id(), Archetype::TYPE_ID);
        assert_eq!(
            Archetype::archetype().internal_name(),
            Some(Rc::new(Archetype::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Archetype::individuate();
        let concept2 = Archetype::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Archetype::individuate();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
