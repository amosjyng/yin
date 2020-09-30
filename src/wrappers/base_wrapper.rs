use super::{debug_wrapper, CommonNodeTrait};
use crate::graph::{Graph, InjectionGraph, KBWrapper};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

/// All wrappers in the `wrappers` module will have these functions available.
pub trait BaseNodeTrait<T>: CommonNodeTrait {
    /// Associate this node with a value.
    fn set_value(&mut self, value: Box<dyn KBWrapper>);

    /// Retrieve the value associated with this node.
    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>>;

    /// Link this node to another one via an outgoing edge.
    fn add_outgoing(&mut self, edge_type: T, to: T);

    /// Link this node to another one via an incoming edge.
    fn add_incoming(&mut self, edge_type: T, from: T);

    /// Whether or not this node is linked to another one via an outgoing edge of a certain type.
    fn has_outgoing(&self, edge_type: T, to: T) -> bool;

    /// Whether or not this node is linked to another one via an outgoing edge of a certain type.
    fn has_incoming(&self, edge_type: T, from: T) -> bool;

    /// All nodes that this one links to via outgoing edges of a certain type.
    fn outgoing_nodes(&self, edge_type: T) -> Vec<T>;

    /// All nodes that this one links to via incoming edges of a certain type.
    fn incoming_nodes(&self, edge_type: T) -> Vec<T>;
}

/// Implementation for the most basic of node wrappers. Offers no additional functionality.
#[derive(Copy, Clone)]
pub struct BaseWrapper {
    graph: InjectionGraph,
    id: usize,
}

impl BaseWrapper {
    /// Create a node wrapper from an existing node's ID.
    pub fn from(id: usize) -> Self {
        BaseWrapper {
            graph: InjectionGraph {},
            id: id,
        }
    }

    /// Create a new node.
    pub fn create() -> Self {
        let mut g = InjectionGraph {};
        BaseWrapper {
            graph: g,
            id: g.add_node(),
        }
    }
}

impl Debug for BaseWrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debug_wrapper("Base", Box::new(self), f)
    }
}

impl Eq for BaseWrapper {}

impl PartialEq for BaseWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl CommonNodeTrait for BaseWrapper {
    fn id(&self) -> usize {
        self.id
    }

    fn set_internal_name(&mut self, name: String) {
        self.graph.set_node_name(self.id, name);
    }

    fn internal_name(&self) -> Option<String> {
        self.graph.node_name(self.id).map(|n| n.clone())
    }
}

impl BaseNodeTrait<BaseWrapper> for BaseWrapper {
    fn set_value(&mut self, value: Box<dyn KBWrapper>) {
        self.graph.set_node_value(self.id, value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.graph.node_value(self.id)
    }

    fn add_outgoing(&mut self, edge_type: BaseWrapper, to: BaseWrapper) {
        self.graph.add_edge(self.id(), edge_type.id(), to.id())
    }

    fn add_incoming(&mut self, edge_type: BaseWrapper, from: BaseWrapper) {
        self.graph.add_edge(from.id(), edge_type.id(), self.id())
    }

    fn has_outgoing(&self, edge_type: BaseWrapper, to: BaseWrapper) -> bool {
        self.graph.has_edge(self.id, edge_type.id, to.id)
    }

    fn has_incoming(&self, edge_type: BaseWrapper, from: BaseWrapper) -> bool {
        self.graph.has_edge(from.id, edge_type.id, self.id)
    }

    fn outgoing_nodes(&self, edge_type: BaseWrapper) -> Vec<BaseWrapper> {
        self.graph
            .outgoing_nodes(self.id(), edge_type.id())
            .into_iter()
            .map(|id| BaseWrapper::from(id))
            .collect()
    }

    fn incoming_nodes(&self, edge_type: BaseWrapper) -> Vec<BaseWrapper> {
        self.graph
            .incoming_nodes(self.id(), edge_type.id())
            .into_iter()
            .map(|id| BaseWrapper::from(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{bind_in_memory_graph, unwrap_weak, WeakWrapper};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = BaseWrapper::create();
        let node2 = BaseWrapper::create();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = BaseWrapper::create();
        let node_copy = BaseWrapper::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = BaseWrapper::create();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some("A".to_string()));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = BaseWrapper::create();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = BaseWrapper::create();
        assert_eq!(a.outgoing_nodes(a), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = BaseWrapper::create();
        let b = BaseWrapper::create();
        let c = BaseWrapper::create();
        let d = BaseWrapper::create();
        let mut e = BaseWrapper::create();
        let edge_type1 = BaseWrapper::create();
        let edge_type2 = BaseWrapper::create();
        a.add_outgoing(edge_type1, b);
        a.add_outgoing(edge_type2, c);
        a.add_outgoing(edge_type1, d);
        e.add_outgoing(edge_type1, a);
        assert_eq!(a.outgoing_nodes(edge_type1), vec![b, d]);
    }

    #[test]
    fn no_incoming_nodes() {
        bind_in_memory_graph();
        let a = BaseWrapper::create();
        assert_eq!(a.incoming_nodes(a), Vec::new());
    }

    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = BaseWrapper::create();
        let b = BaseWrapper::create();
        let c = BaseWrapper::create();
        let d = BaseWrapper::create();
        let mut e = BaseWrapper::create();
        let edge_type1 = BaseWrapper::create();
        let edge_type2 = BaseWrapper::create();
        a.add_incoming(edge_type1, b);
        a.add_incoming(edge_type2, c);
        a.add_incoming(edge_type1, d);
        e.add_incoming(edge_type1, a);
        assert_eq!(a.incoming_nodes(edge_type1), vec![b, d]);
    }

    #[test]
    fn test_has_outgoing() {
        bind_in_memory_graph();
        let mut a = BaseWrapper::create();
        let b = BaseWrapper::create();
        let edge_type1 = BaseWrapper::create();
        let edge_type2 = BaseWrapper::create();
        a.add_outgoing(edge_type1, b);
        assert!(a.has_outgoing(edge_type1, b));
        assert!(!a.has_outgoing(edge_type2, b));
        assert!(!b.has_outgoing(edge_type1, a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = BaseWrapper::create();
        let b = BaseWrapper::create();
        let edge_type1 = BaseWrapper::create();
        let edge_type2 = BaseWrapper::create();
        a.add_incoming(edge_type1, b);
        assert!(a.has_incoming(edge_type1, b));
        assert!(!a.has_incoming(edge_type2, b));
        assert!(!b.has_incoming(edge_type1, a));
    }
}
