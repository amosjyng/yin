//! Wrappers around graph nodes that provide extended functionality at every level.

mod base_node;
mod final_node;
mod inheritance_node;

pub use base_node::{BaseNode, BaseNodeTrait};
pub use final_node::FinalNode;
pub use inheritance_node::{InheritanceNode, InheritanceNodeTrait};
use std::fmt::{Formatter, Result};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// All wrappers around a graph node will have these functions available.
pub trait CommonNodeTrait {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// Associate this concept with an internal name. The name does not need to be unique.
    fn set_internal_name(&mut self, name: &str);

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<Rc<str>>;
}

impl<T> CommonNodeTrait for T
where
    T: Deref + DerefMut,
    T::Target: CommonNodeTrait,
{
    fn id(&self) -> usize {
        (**self).id()
    }

    fn set_internal_name(&mut self, name: &str) {
        (**self).set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<str>> {
        (**self).internal_name()
    }
}

/// Helper function for implementing the Debug trait for a node wrapper.
pub fn debug_wrapper(wrapper_type: &str, node: &dyn CommonNodeTrait, f: &mut Formatter) -> Result {
    match node.internal_name() {
        Some(name) => f.write_fmt(format_args!("{}({},{})", wrapper_type, node.id(), name)),
        None => f.write_fmt(format_args!("{}({})", wrapper_type, node.id())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = BaseNode::new();
        let concept2 = BaseNode::new();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = BaseNode::new();
        concept.set_internal_name("A");
        assert_eq!(concept.internal_name(), Some(Rc::from("A")));
    }
}
