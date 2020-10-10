use crate::concepts::attributes::Inherits;
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Represents an archetype from which various individual nodes can be derived.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Archetype {
    base: FinalNode,
}

impl Archetype {
    /// Create a subtype of the archetype represented by this Archetype instance (as opposed to a
    /// new subtype of Archetype itself, that Archetype::individuate would produce).
    pub fn individuate_as_archetype(&self) -> Self {
        Self::individuate_with_parent(self.id())
    }

    /// Create a new individual of the archetype represented by this Archetype instance (as opposed
    /// to a new subtype of Archetype itself, that Archetype::individuate would produce).
    pub fn individuate_as_tao(&self) -> Tao {
        Tao::individuate_with_parent(self.id())
    }

    /// Individuals that adhere to this archetype. It is possible that some of these individuals
    /// might not be direct descendants of the archetype in question.
    pub fn individuals(&self) -> Vec<Tao> {
        let mut visited: HashSet<FinalNode> = HashSet::new();
        visited.insert(*self.essence());
        let mut to_be_visited: VecDeque<FinalNode> = VecDeque::new();
        to_be_visited.push_back(*self.essence());
        let mut leaves: Vec<FinalNode> = Vec::new();
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
        let mut result: Vec<Tao> = leaves.into_iter().map(|fw| Tao::from(fw)).collect();
        result.sort();
        result
    }
}

impl Debug for Archetype {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Archetype", Box::new(self), f)
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
    fn from(fw: FinalNode) -> Self {
        Self { base: fw }
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

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: FinalNode::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for Archetype {
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
    use crate::concepts::attributes::{Owner, Value};
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
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Archetype::individuate();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Archetype::individuate();
        let concept2 = Archetype::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn test_individuation() {
        bind_in_memory_graph();
        let type1 = Owner::archetype().individuate_as_archetype();
        let type1_instance = type1.individuate_as_tao();
        assert!(type1.has_ancestor(Owner::archetype()));
        assert!(!type1.has_ancestor(Value::archetype()));
        assert!(type1_instance.has_ancestor(type1));
        assert!(type1_instance.has_ancestor(Owner::archetype()));
        assert!(!type1_instance.has_ancestor(Value::archetype()));
    }

    #[test]
    fn test_individuals() {
        bind_in_memory_graph();
        let type1 = Tao::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type1_instance = type1.individuate_as_tao();
        let type2_instance = type2.individuate_as_tao();
        assert_eq!(type1.individuals(), vec![type1_instance, type2_instance]);
        assert_eq!(type2.individuals(), vec![type2_instance]);
    }
}
