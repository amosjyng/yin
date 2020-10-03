use super::{Attribute, AttributeTrait};
use crate::concepts::{Archetype, ArchetypeTrait, FormTrait, Tao};
use crate::wrappers::{debug_wrapper, CommonNodeTrait, FinalWrapper};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Describes the owner as inheriting all attributes of the value.
#[derive(Copy, Clone)]
pub struct Inherits {
    attr: Attribute,
}

impl Debug for Inherits {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Inherits", Box::new(self), f)
    }
}

impl Eq for Inherits {}

impl PartialEq for Inherits {
    fn eq(&self, other: &Self) -> bool {
        self.attr == other.attr
    }
}

impl From<usize> for Inherits {
    fn from(id: usize) -> Self {
        Inherits {
            attr: Attribute::from(id),
        }
    }
}

impl CommonNodeTrait for Inherits {
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

impl ArchetypeTrait<Inherits> for Inherits {
    const TYPE_ID: usize = 5;
    const TYPE_NAME: &'static str = "Inherits";

    fn archetype() -> Archetype {
        Archetype::from(Self::TYPE_ID)
    }

    fn individuate() -> Self {
        Inherits {
            attr: Attribute::individuate(),
        }
    }
}

impl FormTrait for Inherits {
    fn essence(&self) -> &FinalWrapper {
        self.attr.essence()
    }
}

impl AttributeTrait<Inherits> for Inherits {
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
        assert_eq!(Inherits::archetype().id(), Inherits::TYPE_ID);
        assert_eq!(
            Inherits::archetype().internal_name(),
            Some(Rc::new(Inherits::TYPE_NAME.to_string()))
        );
    }
}
