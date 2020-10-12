use super::BaseNode;
use super::{debug_wrapper, BaseNodeTrait, CommonNodeTrait};
use crate::concepts::attributes::Inherits;
use crate::concepts::ArchetypeTrait;
use crate::graph::value_wrappers::KBValue;
use std::cmp::{Eq, PartialEq};
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

/// All wrappers that are aware of attribute inheritance will have these functions available.
pub trait InheritanceNodeTrait<T>: BaseNodeTrait<T> {
    /// The set of nodes, including this one, whose attributes count as this one's.
    fn inheritance_nodes(&self) -> Vec<T>;
}

/// Implementation for a node wrapper that offers inheritance of nodes.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct InheritanceNode {
    base: BaseNode,
}

#[allow(clippy::new_without_default)]
impl InheritanceNode {
    /// Create a new node.
    pub fn new() -> Self {
        InheritanceNode {
            base: BaseNode::new(),
        }
    }

    /// Create a new node with an inheritance relation.
    pub fn new_with_inheritance(type_id: usize) -> Self {
        let mut new_iw = Self::new();
        new_iw.add_outgoing(Inherits::TYPE_ID, &InheritanceNode::from(type_id));
        new_iw
    }
}

impl From<usize> for InheritanceNode {
    fn from(id: usize) -> Self {
        InheritanceNode {
            base: BaseNode::from(id),
        }
    }
}

impl<'a> TryFrom<&'a str> for InheritanceNode {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        BaseNode::try_from(name).map(|n| InheritanceNode { base: n })
    }
}

impl From<BaseNode> for InheritanceNode {
    fn from(b: BaseNode) -> Self {
        InheritanceNode { base: b }
    }
}

impl Debug for InheritanceNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("IWrapper", self, f)
    }
}

impl CommonNodeTrait for InheritanceNode {
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

impl BaseNodeTrait<InheritanceNode> for InheritanceNode {
    fn set_value(&mut self, value: Box<dyn KBValue>) {
        self.base.set_value(value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBValue>>> {
        self.base.value()
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &InheritanceNode) {
        self.base.add_outgoing(edge_type, &to.base)
    }

    fn add_incoming(&mut self, edge_type: usize, from: &InheritanceNode) {
        self.base.add_incoming(edge_type, &from.base)
    }

    fn has_outgoing(&self, edge_type: usize, to: &InheritanceNode) -> bool {
        if edge_type == Inherits::TYPE_ID {
            self.base.has_outgoing(edge_type, &to.base)
        } else {
            self.inheritance_nodes()
                .into_iter()
                .any(|iw| iw.base.has_outgoing(edge_type, &to.base))
        }
    }

    fn has_incoming(&self, edge_type: usize, from: &InheritanceNode) -> bool {
        if edge_type == Inherits::TYPE_ID {
            self.base.has_incoming(edge_type, &from.base)
        } else {
            self.inheritance_nodes()
                .into_iter()
                .any(|iw| iw.base.has_incoming(edge_type, &from.base))
        }
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<InheritanceNode> {
        if edge_type == Inherits::TYPE_ID {
            self.base
                .outgoing_nodes(edge_type)
                .into_iter()
                .map(InheritanceNode::from)
                .collect()
        } else {
            let mut nodes = self
                .inheritance_nodes()
                .into_iter()
                .map(|iw| iw.base.outgoing_nodes(edge_type))
                .flatten()
                .map(InheritanceNode::from)
                .collect::<Vec<InheritanceNode>>();
            nodes.sort();
            nodes.dedup();
            nodes
        }
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<InheritanceNode> {
        if edge_type == Inherits::TYPE_ID {
            self.base
                .incoming_nodes(edge_type)
                .into_iter()
                .map(InheritanceNode::from)
                .collect()
        } else {
            let mut nodes = self
                .inheritance_nodes()
                .into_iter()
                .map(|iw| iw.base.incoming_nodes(edge_type))
                .flatten()
                .map(InheritanceNode::from)
                .collect::<Vec<InheritanceNode>>();
            nodes.sort();
            nodes.dedup();
            nodes
        }
    }
}

impl InheritanceNodeTrait<InheritanceNode> for InheritanceNode {
    fn inheritance_nodes(&self) -> Vec<InheritanceNode> {
        let mut visited = HashSet::new();
        visited.insert(self.base);
        let mut to_be_visited = VecDeque::new();
        to_be_visited.push_back(self.base);
        while let Some(next) = to_be_visited.pop_front() {
            for neighbor in next.outgoing_nodes(Inherits::TYPE_ID) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    to_be_visited.push_back(neighbor);
                }
            }
        }
        let mut result: Vec<InheritanceNode> =
            visited.into_iter().map(InheritanceNode::from).collect();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::Owner;
    use crate::graph::bind_in_memory_graph;
    use crate::graph::value_wrappers::{unwrap_weak, WeakValue};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = InheritanceNode::new();
        let node2 = InheritanceNode::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = InheritanceNode::new();
        let node_copy = InheritanceNode::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn from_name() {
        bind_in_memory_graph();
        let mut node = InheritanceNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(InheritanceNode::try_from("A"), Ok(node));
        assert!(InheritanceNode::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = InheritanceNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = InheritanceNode::new();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn create_with_inheritance() {
        bind_in_memory_graph();
        let owner = InheritanceNode::new();
        let mut type1 = InheritanceNode::new();
        type1.add_outgoing(Owner::TYPE_ID, &owner);
        let node = InheritanceNode::new_with_inheritance(type1.id());
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
        let a = InheritanceNode::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let d = InheritanceNode::new();
        let mut e = InheritanceNode::new();
        let edge_type1 = InheritanceNode::new();
        let edge_type2 = InheritanceNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn inherited_outgoing_nodes() {
        bind_in_memory_graph();
        let mut type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let mut d = InheritanceNode::new();
        let edge_type = InheritanceNode::new();
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
    fn not_inherit_inheritance_attr_outgoing() {
        bind_in_memory_graph();
        let type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        // the inherit edge should be treated specially and not inherited by lower levels
        assert_eq!(type2.outgoing_nodes(Inherits::TYPE_ID), vec![type1]);
        assert_eq!(a.outgoing_nodes(Inherits::TYPE_ID), vec![type2]);
    }

    #[test]
    fn no_incoming_nodes() {
        bind_in_memory_graph();
        let a = InheritanceNode::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let d = InheritanceNode::new();
        let mut e = InheritanceNode::new();
        let edge_type1 = InheritanceNode::new();
        let edge_type2 = InheritanceNode::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_incoming_nodes() {
        bind_in_memory_graph();
        let mut type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let mut d = InheritanceNode::new();
        let edge_type = InheritanceNode::new();
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
    fn not_inherit_inheritance_attr_incoming() {
        bind_in_memory_graph();
        let type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        // the inherit edge should be treated specially and not inherited by lower levels
        assert_eq!(type1.incoming_nodes(Inherits::TYPE_ID), vec![type2]);
        assert_eq!(type2.incoming_nodes(Inherits::TYPE_ID), vec![a]);
    }

    #[test]
    fn test_has_outgoing() {
        bind_in_memory_graph();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let edge_type1 = InheritanceNode::new();
        let edge_type2 = InheritanceNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let edge_type1 = InheritanceNode::new();
        let edge_type2 = InheritanceNode::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }

    #[test]
    fn inherited_has_outgoing() {
        bind_in_memory_graph();
        let mut type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let edge_type = InheritanceNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(a.has_outgoing(edge_type.id(), &b));
        assert!(!a.has_outgoing(edge_type.id(), &c));
    }

    #[test]
    fn not_inherit_inheritance_attr_has_outgoing() {
        bind_in_memory_graph();
        let type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        // the inherit edge should be treated specially and not inherited by lower levels
        assert!(a.has_outgoing(Inherits::TYPE_ID, &type2));
        assert!(!a.has_outgoing(Inherits::TYPE_ID, &type1));
    }

    #[test]
    fn inherited_has_incoming() {
        bind_in_memory_graph();
        let mut type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        let b = InheritanceNode::new();
        let c = InheritanceNode::new();
        let edge_type = InheritanceNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(!a.has_incoming(edge_type.id(), &b));
        assert!(a.has_incoming(edge_type.id(), &c));
    }

    #[test]
    fn not_inherit_inheritance_attr_has_incoming() {
        bind_in_memory_graph();
        let type1 = InheritanceNode::new();
        let mut type2 = InheritanceNode::new();
        let mut a = InheritanceNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        // the inherit edge should be treated specially and not inherited by lower levels
        assert!(type1.has_incoming(Inherits::TYPE_ID, &type2));
        assert!(!type1.has_incoming(Inherits::TYPE_ID, &a));
    }
}
