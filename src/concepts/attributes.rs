//! Contains all attribute archetypes.

mod inherits;
mod owner;
mod value;

pub use inherits::Inherits;
pub use owner::Owner;
pub use value::Value;

use crate::concepts::{ArchetypeTrait, FormTrait, Tao};
use crate::node_wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalNode};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

/// Interface for all attributes.
pub trait AttributeTrait<T>: ArchetypeTrait<T> {
    /// Set the owner for this attribute.
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>);

    /// The owner of an attribute, if it exists.
    fn owner(&self) -> Option<Tao>;

    /// Set the value for this attribute.
    fn set_value(&mut self, value: Box<&dyn FormTrait>);

    /// The value of an attribute, if it exists.
    fn value(&self) -> Option<Tao>;
}

/// Represents either a unary or binary relation.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute {
    /// Wrapper that this abstraction is based on.
    pub base: FinalNode,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Attribute", Box::new(self), f)
    }
}

impl From<usize> for Attribute {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for Attribute {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl CommonNodeTrait for Attribute {
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

impl ArchetypeTrait<Attribute> for Attribute {
    const TYPE_ID: usize = 2;
    const TYPE_NAME: &'static str = "Attribute";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {
        Self {
            base: FinalNode::new_with_inheritance(parent_id),
        }
    }
}

impl FormTrait for Attribute {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl AttributeTrait<Attribute> for Attribute {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.base.add_outgoing(Owner::TYPE_ID, owner.essence());
    }

    fn owner(&self) -> Option<Tao> {
        self.base
            .outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Tao::from(*n))
    }

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
        self.base.add_outgoing(Value::TYPE_ID, value.essence());
    }

    fn value(&self) -> Option<Tao> {
        self.base
            .outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Tao::from(*n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Attribute::archetype().id(), Attribute::TYPE_ID);
        assert_eq!(
            Attribute::archetype().internal_name(),
            Some(Rc::new(Attribute::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let concept = Attribute::individuate();
        let concept_copy = Attribute::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        bind_in_memory_graph();
        let mut concept = Attribute::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Attribute::try_from("A"), Ok(concept));
    }

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let concept1 = Attribute::individuate();
        let concept2 = Attribute::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut concept = Attribute::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut attr_instance = Attribute::individuate();
        let owner_of_attr = Attribute::individuate();
        attr_instance.set_owner(Box::new(&owner_of_attr));
        assert_eq!(attr_instance.owner(), Some(owner_of_attr.ego_death()));
        assert_eq!(attr_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut attr_instance = Attribute::individuate();
        let value_of_attr = Attribute::individuate();
        attr_instance.set_value(Box::new(&value_of_attr));
        assert_eq!(attr_instance.owner(), None);
        assert_eq!(attr_instance.value(), Some(value_of_attr.ego_death()));
    }
}
