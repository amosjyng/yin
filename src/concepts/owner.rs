use super::value::Value;
use crate::concepts::{Concept, ConceptTrait, ConceptTypeTrait};
use crate::wrappers::{debug_wrapper, BaseNodeTrait, BaseWrapper, CommonNodeTrait};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Interface for all attributes.
pub trait AttributeTrait<T>: ConceptTrait {
    /// Set the owner for this attribute.
    fn set_owner(&mut self, owner: Box<&dyn ConceptTrait>);

    /// The owner of an attribute, if it exists.
    fn owner(&self) -> Option<Concept>;

    /// Set the value for this attribute.
    fn set_value(&mut self, value: Box<&dyn ConceptTrait>);

    /// The value of an attribute, if it exists.
    fn value(&self) -> Option<Concept>;
}

/// The owner/source/from-node of an attribute.
#[derive(Copy, Clone)]
pub struct Owner {
    /// Wrapper that this abstraction is based on.
    pub base: BaseWrapper,
}

impl Debug for Owner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Owner", Box::new(self), f)
    }
}

impl Eq for Owner {}

impl PartialEq for Owner {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<usize> for Owner {
    fn from(id: usize) -> Self {
        Owner {
            base: BaseWrapper::from(id),
        }
    }
}

impl CommonNodeTrait for Owner {
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

impl ConceptTypeTrait<Owner> for Owner {
    const TYPE_ID: usize = 1;
    const TYPE_NAME: &'static str = "Owner";

    fn type_concept() -> Concept {
        Concept {
            base: BaseWrapper::from(Self::TYPE_ID),
        }
    }

    fn new() -> Self {
        Owner {
            base: BaseWrapper::new(),
        }
    }
}

impl ConceptTrait for Owner {
    fn base(&self) -> &BaseWrapper {
        &self.base
    }
}

impl AttributeTrait<Owner> for Owner {
    fn set_owner(&mut self, owner: Box<&dyn ConceptTrait>) {
        self.base.add_outgoing(Owner::TYPE_ID, owner.base());
    }

    fn owner(&self) -> Option<Concept> {
        self.base
            .outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Concept { base: *n })
    }

    fn set_value(&mut self, value: Box<&dyn ConceptTrait>) {
        self.base.add_outgoing(Value::TYPE_ID, value.base());
    }

    fn value(&self) -> Option<Concept> {
        self.base
            .outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Concept { base: *n })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Owner::type_concept().id(), Owner::TYPE_ID);
        assert_eq!(
            Owner::type_concept().internal_name(),
            Some(Rc::new(Owner::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::new();
        let owner_of_owner = Owner::new();
        owner_instance.set_owner(Box::new(&owner_of_owner));
        assert_eq!(owner_instance.owner(), Some(owner_of_owner.as_concept()));
        assert_eq!(owner_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::new();
        let value_of_owner = Owner::new();
        owner_instance.set_value(Box::new(&value_of_owner));
        assert_eq!(owner_instance.owner(), None);
        assert_eq!(owner_instance.value(), Some(value_of_owner.as_concept()));
    }
}
