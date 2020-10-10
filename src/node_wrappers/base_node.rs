use super::{debug_wrapper, CommonNodeTrait};
use crate::graph::value_wrappers::KBValue;
use crate::graph::{Graph, InjectionGraph};
use std::cmp::{Eq, Ordering, PartialEq};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// All low-level wrappers will have these functions available.
pub trait BaseNodeTrait<T>: CommonNodeTrait {
    /// Associate this node with a value.
    fn set_value(&mut self, value: Box<dyn KBValue>);

    /// Retrieve the value associated with this node.
    fn value(&self) -> Option<Rc<Box<dyn KBValue>>>;

    /// Link this node to another one via an outgoing edge.
    fn add_outgoing(&mut self, edge_type: usize, to: &T);

    /// Link this node to another one via an incoming edge.
    fn add_incoming(&mut self, edge_type: usize, from: &T);

    /// Whether or not this node is linked to another one via an outgoing edge of a certain type.
    fn has_outgoing(&self, edge_type: usize, to: &T) -> bool;

    /// Whether or not this node is linked to another one via an outgoing edge of a certain type.
    fn has_incoming(&self, edge_type: usize, from: &T) -> bool;

    /// All nodes that this one links to via outgoing edges of a certain type.
    fn outgoing_nodes(&self, edge_type: usize) -> Vec<T>;

    /// All nodes that this one links to via incoming edges of a certain type.
    fn incoming_nodes(&self, edge_type: usize) -> Vec<T>;
}

/// Implementation for the most basic of node wrappers. Offers no additional functionality.
#[derive(Copy, Clone)]
pub struct BaseNode {
    graph: InjectionGraph,
    id: usize,
}

impl BaseNode {
    /// Create a new node.
    pub fn new() -> Self {
        let mut g = InjectionGraph::new();
        BaseNode {
            graph: g,
            id: g.add_node(),
        }
    }
}

impl From<usize> for BaseNode {
    fn from(id: usize) -> Self {
        BaseNode {
            graph: InjectionGraph::new(),
            id: id,
        }
    }
}

impl<'a> TryFrom<&'a str> for BaseNode {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        let g = InjectionGraph {};
        // The last ID will be the most recently added node. We want later nodes to override
        // earlier ones.
        match g.lookup(name).last() {
            Some(id) => Ok(BaseNode { graph: g, id: *id }),
            None => Err(format!("No node with name \"{}\" found.", name)),
        }
    }
}

impl Debug for BaseNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("BWrapper", Box::new(self), f)
    }
}

impl Hash for BaseNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for BaseNode {}

impl PartialEq for BaseNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for BaseNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BaseNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CommonNodeTrait for BaseNode {
    fn id(&self) -> usize {
        self.id
    }

    fn set_internal_name(&mut self, name: String) {
        self.graph.set_node_name(self.id, name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.graph.node_name(self.id).map(|n| n.clone())
    }
}

impl BaseNodeTrait<BaseNode> for BaseNode {
    fn set_value(&mut self, value: Box<dyn KBValue>) {
        self.graph.set_node_value(self.id, value)
    }

    fn value(&self) -> Option<Rc<Box<dyn KBValue>>> {
        self.graph.node_value(self.id)
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &BaseNode) {
        self.graph.add_edge(self.id(), edge_type, to.id())
    }

    fn add_incoming(&mut self, edge_type: usize, from: &BaseNode) {
        self.graph.add_edge(from.id(), edge_type, self.id())
    }

    fn has_outgoing(&self, edge_type: usize, to: &BaseNode) -> bool {
        self.graph.has_edge(self.id, edge_type, to.id)
    }

    fn has_incoming(&self, edge_type: usize, from: &BaseNode) -> bool {
        self.graph.has_edge(from.id, edge_type, self.id)
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<BaseNode> {
        self.graph
            .outgoing_nodes(self.id(), edge_type)
            .into_iter()
            .map(|id| BaseNode::from(id))
            .collect()
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<BaseNode> {
        self.graph
            .incoming_nodes(self.id(), edge_type)
            .into_iter()
            .map(|id| BaseNode::from(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bind_in_memory_graph;
    use crate::graph::value_wrappers::{unwrap_weak, WeakValue};

    #[test]
    fn create_and_retrieve_node_id() {
        bind_in_memory_graph();
        let node1 = BaseNode::new();
        let node2 = BaseNode::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        bind_in_memory_graph();
        let node = BaseNode::new();
        let node_copy = BaseNode::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn from_name() {
        bind_in_memory_graph();
        let mut node = BaseNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(BaseNode::try_from("A"), Ok(node));
        assert!(BaseNode::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        bind_in_memory_graph();
        let mut node = BaseNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        bind_in_memory_graph();
        let mut node = BaseNode::new();
        let v = Rc::new(5);
        node.set_value(Box::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn no_outgoing_nodes() {
        bind_in_memory_graph();
        let a = BaseNode::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[test]
    fn outgoing_nodes() {
        bind_in_memory_graph();
        let mut a = BaseNode::new();
        let b = BaseNode::new();
        let c = BaseNode::new();
        let d = BaseNode::new();
        let mut e = BaseNode::new();
        let edge_type1 = BaseNode::new();
        let edge_type2 = BaseNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn no_incoming_nodes() {
        bind_in_memory_graph();
        let a = BaseNode::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[test]
    fn incoming_nodes() {
        bind_in_memory_graph();
        let mut a = BaseNode::new();
        let b = BaseNode::new();
        let c = BaseNode::new();
        let d = BaseNode::new();
        let mut e = BaseNode::new();
        let edge_type1 = BaseNode::new();
        let edge_type2 = BaseNode::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn test_has_outgoing() {
        bind_in_memory_graph();
        let mut a = BaseNode::new();
        let b = BaseNode::new();
        let edge_type1 = BaseNode::new();
        let edge_type2 = BaseNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        bind_in_memory_graph();
        let mut a = BaseNode::new();
        let b = BaseNode::new();
        let edge_type1 = BaseNode::new();
        let edge_type2 = BaseNode::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }
}
