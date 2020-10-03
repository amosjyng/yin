//! Contains all attribute archetypes.

mod inherits;
mod owner;
mod value;

pub use inherits::Inherits;
pub use owner::Owner;
pub use value::Value;

use crate::concepts::{Archetype, ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, BaseNodeTrait, CommonNodeTrait, FinalWrapper};
use std::fmt::{Debug, Formatter, Result};
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
#[derive(Copy, Clone)]
pub struct Attribute {
    /// Wrapper that this abstraction is based on.
    pub base: FinalWrapper,
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Attribute", Box::new(self), f)
    }
}

impl Eq for Attribute {}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<usize> for Attribute {
    fn from(id: usize) -> Self {
        Attribute {
            base: FinalWrapper::from(id),
        }
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

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Attribute {
            base: FinalWrapper::new(),
        }
    }
}

impl FormTrait for Attribute {
    fn essence(&self) -> &FinalWrapper {
        &self.base
    }
}

impl AttributeTrait<Attribute> for Attribute {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.base.add_outgoing(Attribute::TYPE_ID, owner.essence());
    }

    fn owner(&self) -> Option<Tao> {
        self.base
            .outgoing_nodes(Attribute::TYPE_ID)
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
