//! Wrappers around graph nodes that provide extended functionality at every level.

mod base_node;
mod final_node;
mod inheritance_node;

use crate::Wrapper;
pub use base_node::{BaseNode, BaseNodeTrait};
pub use final_node::FinalNode;
pub use inheritance_node::{InheritanceNode, InheritanceNodeTrait};
use std::fmt::{Formatter, Result};
use std::rc::Rc;

/// All wrappers around a graph node will have these functions available.
pub trait CommonNodeTrait {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// Associate this concept with an internal name. The name does not need to be unique.
    fn set_internal_name(&mut self, name: String);

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<Rc<String>>;
}

impl<T> CommonNodeTrait for T
where
    T: Wrapper,
    T::BaseType: CommonNodeTrait,
{
    fn id(&self) -> usize {
        self.essence().id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.essence_mut().set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.essence().internal_name()
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

    struct TestNodeWrapper {
        actual: BaseNode,
    }

    impl From<BaseNode> for TestNodeWrapper {
        fn from(actual: BaseNode) -> Self {
            Self {
                actual,
            }
        }
    }

    impl Wrapper for TestNodeWrapper {
        type BaseType = BaseNode;

        fn essence(&self) -> &BaseNode {
            &self.actual
        }

        fn essence_mut(&mut self) -> &mut BaseNode {
            &mut self.actual
        }
    }

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
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
