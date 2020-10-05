use crate::concepts::attributes::Inherits;
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalWrapper};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Represents an archetype from which various individual nodes can be derived.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Archetype {
    base: FinalWrapper,
}

impl Archetype {
    /// Individuals that adhere to this archetype. It is possible that some of these individuals
    /// might not be direct descendants of the archetype in question.
    pub fn individuals(&self) -> Vec<Tao> {
        let mut visited: HashSet<FinalWrapper> = HashSet::new();
        visited.insert(*self.essence());
        let mut to_be_visited: VecDeque<FinalWrapper> = VecDeque::new();
        to_be_visited.push_back(*self.essence());
        let mut leaves: Vec<FinalWrapper> = Vec::new();
        while let Some(next) = to_be_visited.pop_front() {
            let children = next.incoming_nodes(Inherits::TYPE_ID);
            if children.is_empty() {
                leaves.push(next);
            } else {
                for child in next.incoming_nodes(Inherits::TYPE_ID) {
                    if !visited.contains(&child) {
                        visited.insert(child);
                        to_be_visited.push_back(child);
                    }
                }
            }
        }
        leaves.into_iter().map(|fw| Tao::from(fw)).collect()
    }
}

impl Debug for Archetype {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Archetype", Box::new(self), f)
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
        Self::individuate_with_parent(Self::TYPE_ID)
    }

    fn individuate_with_parent(parent_id: usize) -> Self {
        Archetype {
            base: FinalWrapper::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for Archetype {
    fn essence(&self) -> &FinalWrapper {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalWrapper {
        &mut self.base
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
