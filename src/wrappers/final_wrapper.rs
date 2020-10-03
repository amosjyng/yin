use super::InheritanceWrapper;
use super::{debug_wrapper, BaseNodeTrait, CommonNodeTrait};
use crate::graph::KBWrapper;
use std::cmp::{Eq, Ordering, PartialEq};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// Final node wrapper that offers a stable API for all concept abstractions dependent on it.
#[derive(Copy, Clone)]
pub struct FinalWrapper {
    base: InheritanceWrapper,
}

impl FinalWrapper {
    /// Create a new node.
    pub fn new() -> Self {
        FinalWrapper {
            base: InheritanceWrapper::new(),
        }
    }
}

impl From<usize> for FinalWrapper {
    fn from(id: usize) -> Self {
        FinalWrapper {
            base: InheritanceWrapper::from(id),
        }
    }
}

impl From<InheritanceWrapper> for FinalWrapper {
    fn from(b: InheritanceWrapper) -> Self {
        FinalWrapper { base: b }
    }
}

impl Debug for FinalWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("FWrapper", Box::new(self), f)
    }
}

impl Eq for FinalWrapper {}

impl PartialEq for FinalWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

impl Ord for FinalWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl PartialOrd for FinalWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CommonNodeTrait for FinalWrapper {
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

impl BaseNodeTrait<FinalWrapper> for FinalWrapper {
    fn set_value(&mut self, value: Box<dyn KBWrapper>) {
        self.base.set_value(value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.base.value()
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &FinalWrapper) {
        self.base.add_outgoing(edge_type, &to.base)
    }

    fn add_incoming(&mut self, edge_type: usize, from: &FinalWrapper) {
        self.base.add_incoming(edge_type, &from.base)
    }

    fn has_outgoing(&self, edge_type: usize, to: &FinalWrapper) -> bool {
        self.base.has_outgoing(edge_type, &to.base)
    }

    fn has_incoming(&self, edge_type: usize, from: &FinalWrapper) -> bool {
        self.base.has_incoming(edge_type, &from.base)
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<FinalWrapper> {
        self.base
            .outgoing_nodes(edge_type)
            .into_iter()
            .map(|b| FinalWrapper::from(b))
            .collect()
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<FinalWrapper> {
        self.base
            .incoming_nodes(edge_type)
            .into_iter()
            .map(|b| FinalWrapper::from(b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::Inherits;
    use crate::concepts::ArchetypeTrait;
    use crate::graph::{bind_in_memory_graph, unwrap_weak, WeakWrapper};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = FinalWrapper::new();
        let node2 = FinalWrapper::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = FinalWrapper::new();
        let node_copy = FinalWrapper::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = FinalWrapper::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = FinalWrapper::new();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = FinalWrapper::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let d = FinalWrapper::new();
        let mut e = FinalWrapper::new();
        let edge_type1 = FinalWrapper::new();
        let edge_type2 = FinalWrapper::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_outgoing_nodes() {
        bind_in_memory_graph();
        let mut type1 = FinalWrapper::new();
        let mut type2 = FinalWrapper::new();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let mut d = FinalWrapper::new();
        let edge_type = FinalWrapper::new();
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
        let a = FinalWrapper::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let d = FinalWrapper::new();
        let mut e = FinalWrapper::new();
        let edge_type1 = FinalWrapper::new();
        let edge_type2 = FinalWrapper::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_incoming_nodes() {
        bind_in_memory_graph();
        let mut type1 = FinalWrapper::new();
        let mut type2 = FinalWrapper::new();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let mut d = FinalWrapper::new();
        let edge_type = FinalWrapper::new();
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
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let edge_type1 = FinalWrapper::new();
        let edge_type2 = FinalWrapper::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let edge_type1 = FinalWrapper::new();
        let edge_type2 = FinalWrapper::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }

    #[test]
    fn inherited_has_outgoing() {
        bind_in_memory_graph();
        let mut type1 = FinalWrapper::new();
        let mut type2 = FinalWrapper::new();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let edge_type = FinalWrapper::new();
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
        let mut type1 = FinalWrapper::new();
        let mut type2 = FinalWrapper::new();
        let mut a = FinalWrapper::new();
        let b = FinalWrapper::new();
        let c = FinalWrapper::new();
        let edge_type = FinalWrapper::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(!a.has_incoming(edge_type.id(), &b));
        assert!(a.has_incoming(edge_type.id(), &c));
    }
}
