use crate::concepts::attributes::{HasAttributeType, Inherits};
use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
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
        let mut result: Vec<Tao> = leaves.into_iter().map(Tao::from).collect();
        result.sort();
        result
    }

    /// Add an attribute type to this archetype.
    pub fn add_attribute_type(&mut self, attribute_type: Archetype) {
        self.essence_mut()
            .add_outgoing(HasAttributeType::TYPE_ID, attribute_type.essence());
    }

    /// Retrieve non-inherited attribute types that are introduced by this archetype to all
    /// descendant archetypes. Attribute types introduced by an ancestor do not count.
    pub fn introduced_attribute_types(&self) -> Vec<Archetype> {
        self.essence()
            .base_wrapper()
            .outgoing_nodes(HasAttributeType::TYPE_ID)
            .into_iter()
            .map(FinalNode::from)
            .map(Archetype::from)
            .collect()
    }
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
    fn from(fw: FinalNode) -> Self {
        Self { base: fw }
    }
}

impl<'a> TryFrom<&'a str> for Archetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
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

impl<'a> ArchetypeTrait<'a, Archetype> for Archetype {
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
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Archetype::archetype().id(), Archetype::TYPE_ID);
        assert_eq!(
            Archetype::archetype().internal_name(),
            Some(Rc::new(Archetype::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Archetype::individuate();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Archetype::try_from("A"), Ok(concept));
        assert!(Archetype::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Archetype::individuate();
        let concept2 = Archetype::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn test_individuation() {
        initialize_kb();
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
        initialize_kb();
        let type1 = Tao::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type1_instance = type1.individuate_as_tao();
        let type2_instance = type2.individuate_as_tao();
        assert_eq!(type1.individuals(), vec![type1_instance, type2_instance]);
        assert_eq!(type2.individuals(), vec![type2_instance]);
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        let type2 = Tao::archetype().individuate_as_archetype();
        assert_eq!(type1.introduced_attribute_types(), Vec::<Archetype>::new());

        type1.add_attribute_type(type2);
        assert_eq!(type1.introduced_attribute_types(), vec!(type2));
    }

    #[test]
    fn test_attribute_types_not_inherited() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        let type2 = Tao::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);

        assert_eq!(type3.introduced_attribute_types(), Vec::<Archetype>::new());
    }
}
