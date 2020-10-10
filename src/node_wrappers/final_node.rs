use super::{debug_wrapper, BaseNodeTrait, CommonNodeTrait};
use super::{InheritanceNode, InheritanceNodeTrait};
use crate::graph::value_wrappers::KBValue;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

/// Final node wrapper that offers a stable API for all concept abstractions dependent on it.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FinalNode {
    base: InheritanceNode,
}

impl FinalNode {
    /// Create a new node.
    pub fn new() -> Self {
        FinalNode {
            base: InheritanceNode::new(),
        }
    }

    /// Create a new node with an inheritance relation.
    pub fn new_with_inheritance(type_id: usize) -> Self {
        Self::from(InheritanceNode::new_with_inheritance(type_id))
    }
}

impl From<usize> for FinalNode {
    fn from(id: usize) -> Self {
        FinalNode {
            base: InheritanceNode::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for FinalNode {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        InheritanceNode::try_from(name).map(|n| FinalNode { base: n })
    }
}

impl From<InheritanceNode> for FinalNode {
    fn from(b: InheritanceNode) -> Self {
        FinalNode { base: b }
    }
}

impl Debug for FinalNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("FWrapper", Box::new(self), f)
    }
}

impl CommonNodeTrait for FinalNode {
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

impl BaseNodeTrait<FinalNode> for FinalNode {
    fn set_value(&mut self, value: Box<dyn KBValue>) {
        self.base.set_value(value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBValue>>> {
        self.base.value()
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &FinalNode) {
        self.base.add_outgoing(edge_type, &to.base)
    }

    fn add_incoming(&mut self, edge_type: usize, from: &FinalNode) {
        self.base.add_incoming(edge_type, &from.base)
    }

    fn has_outgoing(&self, edge_type: usize, to: &FinalNode) -> bool {
        self.base.has_outgoing(edge_type, &to.base)
    }

    fn has_incoming(&self, edge_type: usize, from: &FinalNode) -> bool {
        self.base.has_incoming(edge_type, &from.base)
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<FinalNode> {
        self.base
            .outgoing_nodes(edge_type)
            .into_iter()
            .map(|b| FinalNode::from(b))
            .collect()
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<FinalNode> {
        self.base
            .incoming_nodes(edge_type)
            .into_iter()
            .map(|b| FinalNode::from(b))
            .collect()
    }
}

impl InheritanceNodeTrait<FinalNode> for FinalNode {
    fn inheritance_nodes(&self) -> Vec<FinalNode> {
        self.base
            .inheritance_nodes()
            .into_iter()
            .map(|b| FinalNode::from(b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::{Inherits, Owner};
    use crate::concepts::ArchetypeTrait;
    use crate::graph::bind_in_memory_graph;
    use crate::graph::value_wrappers::{unwrap_weak, WeakValue};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = FinalNode::new();
        let node2 = FinalNode::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = FinalNode::new();
        let node_copy = FinalNode::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn from_name() {
        bind_in_memory_graph();
        let mut node = FinalNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(FinalNode::try_from("A"), Ok(node));
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = FinalNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = FinalNode::new();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn create_with_inheritance() {
        bind_in_memory_graph();
        let owner = FinalNode::new();
        let mut type1 = FinalNode::new();
        type1.add_outgoing(Owner::TYPE_ID, &owner);
        let node = FinalNode::new_with_inheritance(type1.id());
        assert!(node.has_outgoing(Owner::TYPE_ID, &owner));
    }

    #[test]
    fn check_inheritance_nodes() {
        bind_in_memory_graph();
        let type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.inheritance_nodes(), vec![type1, type2, a]);
        assert_eq!(type2.inheritance_nodes(), vec![type1, type2]);
        assert_eq!(type1.inheritance_nodes(), vec![type1]);
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = FinalNode::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let d = FinalNode::new();
        let mut e = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_outgoing_nodes() {
        bind_in_memory_graph();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let mut d = FinalNode::new();
        let edge_type = FinalNode::new();
        a.add_outgoing(edge_type.id(), &b);
        type1.add_outgoing(edge_type.id(), &c);
        type2.add_outgoing(edge_type.id(), &c);
        d.add_outgoing(edge_type.id(), &a);

        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.outgoing_nodes(edge_type.id()), vec![b, c]);
        assert_eq!(type1.outgoing_nodes(edge_type.id()), vec![c]);
    }

    #[test]
    fn no_incoming_nodes() {
        bind_in_memory_graph();
        let a = FinalNode::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let d = FinalNode::new();
        let mut e = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_incoming_nodes() {
        bind_in_memory_graph();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let mut d = FinalNode::new();
        let edge_type = FinalNode::new();
        a.add_incoming(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);
        type2.add_incoming(edge_type.id(), &c);
        d.add_incoming(edge_type.id(), &a);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.incoming_nodes(edge_type.id()), vec![b, c]);
        assert_eq!(type1.incoming_nodes(edge_type.id()), vec![c]);
    }

    #[test]
    fn test_has_outgoing() {
        bind_in_memory_graph();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }

    #[test]
    fn inherited_has_outgoing() {
        bind_in_memory_graph();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let edge_type = FinalNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(a.has_outgoing(edge_type.id(), &b));
        assert!(!a.has_outgoing(edge_type.id(), &c));
    }

    #[test]
    fn inherited_has_incoming() {
        bind_in_memory_graph();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let edge_type = FinalNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(!a.has_incoming(edge_type.id(), &b));
        assert!(a.has_incoming(edge_type.id(), &c));
    }
}
