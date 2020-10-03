use super::BaseWrapper;
use super::{debug_wrapper, BaseNodeTrait, CommonNodeTrait};
use crate::concepts::attributes::Inherits;
use crate::concepts::ArchetypeTrait;
use crate::graph::KBWrapper;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Implementation for a node wrapper that offers inheritance of nodes.
#[derive(Copy, Clone)]
pub struct InheritanceWrapper {
    base: BaseWrapper,
}

impl InheritanceWrapper {
    /// Create a new node.
    pub fn new() -> Self {
        InheritanceWrapper {
            base: BaseWrapper::new(),
        }
    }

    /// The set of nodes, including this one, whose attributes count as this one's.
    fn inheritance_nodes(&self) -> HashSet<BaseWrapper> {
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
        visited
    }
}

impl From<usize> for InheritanceWrapper {
    fn from(id: usize) -> Self {
        InheritanceWrapper {
            base: BaseWrapper::from(id),
        }
    }
}

impl From<BaseWrapper> for InheritanceWrapper {
    fn from(b: BaseWrapper) -> Self {
        InheritanceWrapper { base: b }
    }
}

impl Debug for InheritanceWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("IWrapper", Box::new(self), f)
    }
}

impl Eq for InheritanceWrapper {}

impl PartialEq for InheritanceWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Ord for InheritanceWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl PartialOrd for InheritanceWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CommonNodeTrait for InheritanceWrapper {
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

impl BaseNodeTrait<InheritanceWrapper> for InheritanceWrapper {
    fn set_value(&mut self, value: Box<dyn KBWrapper>) {
        self.base.set_value(value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.base.value()
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &InheritanceWrapper) {
        self.base.add_outgoing(edge_type, &to.base)
    }

    fn add_incoming(&mut self, edge_type: usize, from: &InheritanceWrapper) {
        self.base.add_incoming(edge_type, &from.base)
    }

    fn has_outgoing(&self, edge_type: usize, to: &InheritanceWrapper) -> bool {
        self.inheritance_nodes()
            .into_iter()
            .any(|b| b.has_outgoing(edge_type, &to.base))
    }

    fn has_incoming(&self, edge_type: usize, from: &InheritanceWrapper) -> bool {
        self.inheritance_nodes()
            .into_iter()
            .any(|b| b.has_incoming(edge_type, &from.base))
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<InheritanceWrapper> {
        let mut nodes = self
            .inheritance_nodes()
            .into_iter()
            .map(|b| b.outgoing_nodes(edge_type))
            .into_iter()
            .flatten()
            .map(|b| InheritanceWrapper::from(b))
            .collect::<Vec<InheritanceWrapper>>();
        nodes.sort();
        nodes.dedup();
        nodes
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<InheritanceWrapper> {
        let mut nodes = self
            .inheritance_nodes()
            .into_iter()
            .map(|b| b.incoming_nodes(edge_type))
            .into_iter()
            .flatten()
            .map(|b| InheritanceWrapper::from(b))
            .collect::<Vec<InheritanceWrapper>>();
        nodes.sort();
        nodes.dedup();
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{bind_in_memory_graph, unwrap_weak, WeakWrapper};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = InheritanceWrapper::new();
        let node2 = InheritanceWrapper::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = InheritanceWrapper::new();
        let node_copy = InheritanceWrapper::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = InheritanceWrapper::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = InheritanceWrapper::new();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = InheritanceWrapper::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let d = InheritanceWrapper::new();
        let mut e = InheritanceWrapper::new();
        let edge_type1 = InheritanceWrapper::new();
        let edge_type2 = InheritanceWrapper::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_outgoing_nodes() {
        bind_in_memory_graph();
        let mut type1 = InheritanceWrapper::new();
        let mut type2 = InheritanceWrapper::new();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let mut d = InheritanceWrapper::new();
        let edge_type = InheritanceWrapper::new();
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
        let a = InheritanceWrapper::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let d = InheritanceWrapper::new();
        let mut e = InheritanceWrapper::new();
        let edge_type1 = InheritanceWrapper::new();
        let edge_type2 = InheritanceWrapper::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_incoming_nodes() {
        bind_in_memory_graph();
        let mut type1 = InheritanceWrapper::new();
        let mut type2 = InheritanceWrapper::new();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let mut d = InheritanceWrapper::new();
        let edge_type = InheritanceWrapper::new();
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
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let edge_type1 = InheritanceWrapper::new();
        let edge_type2 = InheritanceWrapper::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let edge_type1 = InheritanceWrapper::new();
        let edge_type2 = InheritanceWrapper::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }

    #[test]
    fn inherited_has_outgoing() {
        bind_in_memory_graph();
        let mut type1 = InheritanceWrapper::new();
        let mut type2 = InheritanceWrapper::new();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let edge_type = InheritanceWrapper::new();
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
        let mut type1 = InheritanceWrapper::new();
        let mut type2 = InheritanceWrapper::new();
        let mut a = InheritanceWrapper::new();
        let b = InheritanceWrapper::new();
        let c = InheritanceWrapper::new();
        let edge_type = InheritanceWrapper::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(!a.has_incoming(edge_type.id(), &b));
        assert!(a.has_incoming(edge_type.id(), &c));
    }
}
