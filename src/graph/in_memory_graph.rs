use super::{Graph, KBWrapper};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::rc::Rc;

struct NodeInfo {
    name: Option<String>,
    value: Option<Rc<Box<dyn KBWrapper>>>,
}

pub struct InMemoryGraph {
    graph: petgraph::graph::Graph<NodeInfo, usize>,
}

impl InMemoryGraph {
    /// Constructs an empty new in-memory graph
    pub fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::<NodeInfo, usize>::new(),
        }
    }
}

impl<'a> Graph<'a> for InMemoryGraph {
    fn add_node(&mut self) -> usize {
        let new_node_info = NodeInfo {
            name: None,
            value: None,
        };
        self.graph.add_node(new_node_info).index()
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        self.graph
            .node_weight_mut(NodeIndex::new(id))
            .unwrap()
            .value = Some(Rc::new(value));
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        self.graph.node_weight_mut(NodeIndex::new(id)).unwrap().name = Some(name);
    }

    fn node_name(&self, id: usize) -> Option<String> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.name.as_ref().map(|n| n.clone()))
            .flatten()
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.value.as_ref().map(|v| v.clone()))
            .flatten()
    }

    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize) {
        self.graph
            .add_edge(NodeIndex::new(from), NodeIndex::new(to), edge_type);
    }

    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool {
        // can't use petgraph's find_edge because it doesn't take into account the edge label
        self.graph.edges_connecting(NodeIndex::new(from), NodeIndex::new(to))
        .filter(|e| *e.weight() == edge_type)
        .next()
        .is_some()
    }

    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(from), Direction::Outgoing)
            .filter(|e| *e.weight() == edge_type)
            .map(|e| e.target().index())
            .collect();
        result.sort(); // sort for determinism
        result
    }

    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(to), Direction::Incoming)
            .filter(|e| *e.weight() == edge_type)
            .map(|e| e.source().index())
            .collect();
        result.sort(); // sort for determinism
        result
    }

    fn all_outgoing_nodes(&self, from: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(from), Direction::Outgoing)
            .map(|e| e.target().index())
            .collect();
        result.sort(); // sort for determinism
        result
    }

    fn all_incoming_nodes(&self, to: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(to), Direction::Incoming)
            .map(|e| e.source().index())
            .collect();
        result.sort(); // sort for determinism
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn in_memory_graph_create() {
        bind_in_memory_graph();
    }

    #[test]
    fn in_memory_graph_add_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let id = g.add_node();
        assert!(g.node_value(id).is_none());
        assert_eq!(g.node_name(id), None);
    }

    #[test]
    fn in_memory_graph_set_node_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new(5);
        g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_string_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new("5");
        g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_weak::<&str>(g.node_value(a_id)), Some(v));
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.node_name(a_id), Some("A".to_string()));
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new(5);
        g.set_node_name(a_id, "A".to_string());
        g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
        assert_eq!(g.node_name(a_id), Some("A".to_string()));
        assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
    }

    #[test]
    fn in_memory_graph_no_outgoing_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        assert_eq!(g.all_outgoing_nodes(a_id), Vec::new());
        assert_eq!(g.outgoing_nodes(a_id, a_id), Vec::new());
    }

    #[test]
    fn in_memory_graph_one_outgoing_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    fn in_memory_graph_multiple_outgoing_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        g.add_edge(a_id, edge_type, c_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id, c_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id, c_id]);
    }

    #[test]
    fn in_memory_graph_outgoing_ignores_incoming_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        g.add_edge(a_id, edge_type, d_id);
        g.add_edge(c_id, edge_type, a_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id, d_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id, d_id]);
    }

    #[test]
    fn in_memory_graph_outgoing_ignores_wrong_edge_type() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(a_id, edge_type1, b_id);
        g.add_edge(a_id, edge_type2, c_id);
        g.add_edge(a_id, edge_type1, d_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id, c_id, d_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type1), vec![b_id, d_id]);
    }

    #[test]
    fn test_has_edge() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(a_id, edge_type1, b_id);
        assert!(g.has_edge(a_id, edge_type1, b_id));
        assert!(!g.has_edge(a_id, edge_type2, b_id));
        assert!(!g.has_edge(b_id, edge_type2, a_id));
    }

    #[test]
    fn in_memory_graph_no_incoming_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        assert_eq!(g.all_incoming_nodes(a_id), Vec::new());
        assert_eq!(g.incoming_nodes(a_id, a_id), Vec::new());
    }

    #[test]
    fn in_memory_graph_incoming_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    fn in_memory_graph_multiple_incoming_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        g.add_edge(c_id, edge_type, a_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id, c_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id, c_id]);
    }

    #[test]
    fn in_memory_graph_incoming_ignores_outgoing_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        g.add_edge(d_id, edge_type, a_id);
        g.add_edge(a_id, edge_type, c_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id, d_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id, d_id]);
    }

    #[test]
    fn in_memory_graph_incoming_ignores_wrong_edge_type() {
        bind_in_memory_graph();
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(b_id, edge_type1, a_id);
        g.add_edge(c_id, edge_type2, a_id);
        g.add_edge(d_id, edge_type1, a_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id, c_id, d_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type1), vec![b_id, d_id]);
    }
}
