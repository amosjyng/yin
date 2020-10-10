use super::{Graph, KBValue};
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

/// Wrapper around node name so that we can have multiple references to this in both the node and
/// the edge weights, and also implement a custom Display for both of them.
#[derive(Default)]
struct NodeName {
    name: Option<Rc<String>>,
}

#[derive(Default)]
struct NodeInfo {
    /// Store ID here as well in order to allow printing the ID as a label when no internal name is
    /// assigned.
    id: usize,
    name: Rc<RefCell<NodeName>>,
    value: Option<Rc<Box<dyn KBValue>>>,
}

impl<'a> Display for NodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.name.borrow().name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "{}", self.id),
        }
    }
}

/// Wrapper for edge's type ID so that it can have its own Display when it comes time for printing
/// out its label.
struct EdgeInfo {
    type_id: usize,
    type_name: Rc<RefCell<NodeName>>,
}

impl Display for EdgeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.type_name.borrow().name {
            Some(name) => write!(f, "{}", name),
            None => write!(f, "{}", self.type_id),
        }
    }
}

/// Graph that resides entirely in-memory, based on PetGraph.
pub struct InMemoryGraph {
    graph: petgraph::graph::Graph<NodeInfo, EdgeInfo>,
    names: HashMap<Rc<String>, Vec<usize>>,
}

impl InMemoryGraph {
    /// Constructs an empty new in-memory graph
    pub fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::new(),
            names: HashMap::new(),
        }
    }
}

impl Graph for InMemoryGraph {
    fn size(&self) -> usize {
        self.graph.node_count()
    }

    fn add_node(&mut self) -> usize {
        let new_id = self.graph.add_node(Default::default());
        self.graph.node_weight_mut(new_id).unwrap().id = new_id.index();
        new_id.index()
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBValue>) {
        self.graph
            .node_weight_mut(NodeIndex::new(id))
            .unwrap()
            .value = Some(Rc::new(value));
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        let name_rc = Rc::new(name);
        match self.names.get_mut(&name_rc) {
            Some(existing_vec) => existing_vec.push(id),
            None => {
                self.names.insert(name_rc.clone(), vec![id]);
            }
        };
        self.graph
            .node_weight_mut(NodeIndex::new(id))
            .unwrap()
            .name
            .borrow_mut()
            .name = Some(name_rc);
    }

    fn node_name(&self, id: usize) -> Option<Rc<String>> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.name.borrow().name.as_ref().map(|rc| rc.clone()))
            .flatten()
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBValue>>> {
        self.graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.value.as_ref().map(|v| v.clone()))
            .flatten()
    }

    fn lookup(&self, name: &str) -> Vec<usize> {
        let mut ids = self
            .names
            .get(&Rc::new(name.to_string()))
            .map(|v| v.clone())
            .unwrap_or(Vec::new());
        ids.sort();
        ids
    }

    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize) {
        let edge_info = EdgeInfo {
            type_id: edge_type,
            type_name: self
                .graph
                .node_weight(NodeIndex::new(edge_type))
                .unwrap()
                .name
                .clone(),
        };
        self.graph
            .add_edge(NodeIndex::new(from), NodeIndex::new(to), edge_info);
    }

    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool {
        // can't use petgraph's find_edge because it doesn't take into account the edge label
        self.graph
            .edges_connecting(NodeIndex::new(from), NodeIndex::new(to))
            .filter(|e| e.weight().type_id == edge_type)
            .next()
            .is_some()
    }

    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(from), Direction::Outgoing)
            .filter(|e| e.weight().type_id == edge_type)
            .map(|e| e.target().index())
            .collect();
        result.sort(); // sort for determinism
        result
    }

    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize> {
        let mut result: Vec<usize> = self
            .graph
            .edges_directed(NodeIndex::new(to), Direction::Incoming)
            .filter(|e| e.weight().type_id == edge_type)
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

    fn into_dot(&self) -> String {
        format!("{}", Dot::new(&self.graph))
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::graph::value_wrappers::{unwrap_weak, WeakValue};

    #[test]
    fn test_create() {
        bind_in_memory_graph();
    }

    #[test]
    fn test_add_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let id = g.add_node();
        assert!(g.node_value(id).is_none());
        assert_eq!(g.node_name(id), None);
    }

    #[test]
    fn test_size() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let initial_size = g.size();
        g.add_node();
        assert_eq!(g.size(), initial_size + 1);
    }

    #[test]
    fn test_set_node_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let v = Rc::new(5);
        g.set_node_value(a_id, Box::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn test_retrieve_node_string_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let v = Rc::new("5");
        g.set_node_value(a_id, Box::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<&str>(g.node_value(a_id)), Some(v));
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    fn test_retrieve_node_name() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn test_retrieve_node_name_value() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let v = Rc::new(5);
        g.set_node_name(a_id, "A".to_string());
        g.set_node_value(a_id, Box::new(WeakValue::new(&v)));
        assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
        assert_eq!(unwrap_weak::<i32>(g.node_value(a_id)), Some(v));
    }

    #[test]
    fn test_lookup_by_name_none() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        g.add_node();
        g.add_node();
        assert_eq!(g.lookup("A"), Vec::<usize>::new());
    }

    #[test]
    fn test_lookup_by_name_one() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.lookup("A"), vec![a_id]);
    }

    #[test]
    fn test_lookup_by_name_multiple() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let b_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        g.set_node_name(b_id, "A".to_string());
        assert_eq!(g.lookup("A"), vec![a_id, b_id]);
    }

    #[test]
    fn test_no_outgoing_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        assert_eq!(g.all_outgoing_nodes(a_id), Vec::<usize>::new());
        assert_eq!(g.outgoing_nodes(a_id, a_id), Vec::<usize>::new());
    }

    #[test]
    fn test_one_outgoing_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    fn test_multiple_outgoing_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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
    fn test_outgoing_ignores_incoming_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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
    fn test_outgoing_ignores_wrong_edge_type() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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
        let mut g = InjectionGraph::new();
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
    fn test_no_incoming_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        assert_eq!(g.all_incoming_nodes(a_id), Vec::<usize>::new());
        assert_eq!(g.incoming_nodes(a_id, a_id), Vec::<usize>::new());
    }

    #[test]
    fn test_incoming_node() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    fn test_multiple_incoming_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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
    fn test_incoming_ignores_outgoing_nodes() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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
    fn test_incoming_ignores_wrong_edge_type() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
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

    #[test]
    fn test_into_dot() {
        bind_in_memory_graph();
        let mut g = InjectionGraph::new();
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge1_type_id = g.add_node();
        let edge2_type_id = g.add_node();
        g.set_node_name(b_id, "B node".to_owned());
        g.set_node_name(edge1_type_id, "test attr".to_owned());
        g.add_edge(a_id, edge1_type_id, b_id);
        g.add_edge(a_id, edge2_type_id, b_id);

        let dot_representation = g.into_dot();
        print_graph_debug();
        assert!(dot_representation.starts_with("digraph"));
        assert!(dot_representation.contains(" [ label = \"B node\" ]"));
        assert_eq!(
            dot_representation
                .matches(" [ label = \"test attr\" ]")
                .count(),
            2 // one label for the node, another for the edge
        );
        // Test that unlabeled edges get represented fine
        let edge2_label = format!(" [ label = \"{}\" ]", edge2_type_id);
        assert_eq!(
            dot_representation.matches(edge2_label.as_str()).count(),
            2 // one label for the node, another for the edge
        );
    }
}
