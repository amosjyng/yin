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
    fn set_value(&mut self, value: Rc<dyn KBValue>);

    /// Retrieve the value associated with this node.
    fn value(&self) -> Option<Rc<dyn KBValue>>;

    /// Add flag to node.
    fn add_flag(&mut self, flag_type: usize);

    /// Check if node has flag.
    fn has_flag(&self, flag_type: usize) -> bool;

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

#[allow(clippy::new_without_default)]
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
            id,
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
        debug_wrapper("BWrapper", self, f)
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

    fn set_internal_name(&mut self, name: &str) {
        self.graph.set_node_name(self.id, name);
    }

    fn internal_name(&self) -> Option<Rc<str>> {
        self.graph.node_name(self.id)
    }
}

impl BaseNodeTrait<BaseNode> for BaseNode {
    fn set_value(&mut self, value: Rc<dyn KBValue>) {
        self.graph.set_node_value(self.id, value)
    }

    fn value(&self) -> Option<Rc<dyn KBValue>> {
        self.graph.node_value(self.id)
    }

    fn add_flag(&mut self, flag_type: usize) {
        // note: we can also reuse the edge API as such:
        //self.graph.add_edge(self.id, flag_type, self.id);
        self.graph.add_flag(self.id, flag_type);
    }

    fn has_flag(&self, flag_type: usize) -> bool {
        // note: we can also reuse the edge API as such:
        //self.graph.has_edge(self.id, flag_type, self.id)
        self.graph.has_flag(self.id, flag_type)
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
            .map(BaseNode::from)
            .collect()
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<BaseNode> {
        self.graph
            .incoming_nodes(self.id(), edge_type)
            .into_iter()
            .map(BaseNode::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::value_wrappers::{unwrap_value, WeakValue};
    use crate::tao::initialize_kb;

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let node1 = BaseNode::new();
        let node2 = BaseNode::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let node = BaseNode::new();
        let node_copy = BaseNode::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut node = BaseNode::new();
        node.set_internal_name("A");
        assert_eq!(BaseNode::try_from("A"), Ok(node));
        assert!(BaseNode::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut node = BaseNode::new();
        node.set_internal_name("A");
        assert_eq!(node.internal_name(), Some(Rc::from("A")));
    }

    #[test]
    fn retrieve_node_value() {
        initialize_kb();
        let mut node = BaseNode::new();
        let v = Rc::new(5);
        node.set_value(Rc::new(WeakValue::new(&v)));
        assert_eq!(unwrap_value::<i32>(node.value()), Some(v));
    }

    #[test]
    fn test_flags() {
        initialize_kb();
        let mut a = BaseNode::new();
        let b = BaseNode::new();
        assert!(!a.has_flag(b.id()));

        a.add_flag(b.id());
        assert!(a.has_flag(b.id()));
    }

    #[test]
    fn no_outgoing_nodes() {
        initialize_kb();
        let a = BaseNode::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn outgoing_nodes() {
        initialize_kb();
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
        initialize_kb();
        let a = BaseNode::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn incoming_nodes() {
        initialize_kb();
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
        initialize_kb();
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
        initialize_kb();
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
