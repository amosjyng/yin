use super::{Attribute, AttributeTrait};
use crate::concepts::{Archetype, ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, BaseWrapper, CommonNodeTrait};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// The owner/source/from-node of an attribute.
#[derive(Copy, Clone)]
pub struct Owner {
    attr: Attribute,
}

impl Debug for Owner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Owner", Box::new(self), f)
    }
}

impl Eq for Owner {}

impl PartialEq for Owner {
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr
    }
}

impl From<usize> for Owner {
    fn from(id: usize) -> Self {
        Owner {
            attr: Attribute::from(id),
        }
    }
}

impl CommonNodeTrait for Owner {
    fn id(&self) -> usize {
        self.attr.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.attr.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.attr.internal_name()
    }
}

impl ArchetypeTrait<Owner> for Owner {
    const TYPE_ID: usize = 3;
    const TYPE_NAME: &'static str = "Owner";

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Owner {
            attr: Attribute::individuate(),
        }
    }
}

impl FormTrait for Owner {
    fn essence(&self) -> &BaseWrapper {
        self.attr.essence()
    }
}

impl AttributeTrait<Owner> for Owner {
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {
        self.attr.set_owner(owner);
    }

    fn owner(&self) -> Option<Tao> {
        self.attr.owner()
    }

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {
        self.attr.set_value(value);
    }

    fn value(&self) -> Option<Tao> {
        self.attr.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn check_type_created() {
        bind_in_memory_graph();
        assert_eq!(Owner::archetype().id(), Owner::TYPE_ID);
        assert_eq!(
            Owner::archetype().internal_name(),
            Some(Rc::new(Owner::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn get_owner() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::individuate();
        let owner_of_owner = Owner::individuate();
        owner_instance.set_owner(Box::new(&owner_of_owner));
        assert_eq!(owner_instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(owner_instance.value(), None);
    }

    #[test]
    fn get_value() {
        bind_in_memory_graph();
        let mut owner_instance = Owner::individuate();
        let value_of_owner = Owner::individuate();
        owner_instance.set_value(Box::new(&value_of_owner));
        assert_eq!(owner_instance.owner(), None);
        assert_eq!(owner_instance.value(), Some(value_of_owner.ego_death()));
    }
}
