//! Graph implementations

use petgraph::graph::NodeIndex;
use std::any::Any;
use std::cell::RefCell;
use std::thread::LocalKey;

thread_local! {
    pub static GRAPH: RefCell<InjectionGraph> = RefCell::new(InjectionGraph{
        injection: None
    });
}

/// Bind GRAPH to a new graph that sits entirely in memory.
pub fn bind_in_memory_graph() {
    GRAPH.with(|g| g.borrow_mut().inject(Box::new(InMemoryGraph::new())));
}

/// Retrieve the thread-local bound graph.
pub fn thread_local_graph<'a>() -> &'a LocalKey<RefCell<InjectionGraph>> {
    &GRAPH
}

/// A classic directed Graph with nodes and labeled links.
pub trait Graph<'a> {
    /// Adds a new node to the graph, and returns the node's ID.
    fn add_node(&mut self) -> usize;

    /// Sets the value for a given node. Values can only be set once.
    fn set_node_value(&mut self, id: usize, value: &'a dyn Any);

    /// Sets the name for a given node. Names can only be set once.
    fn set_node_name(&mut self, id: usize, name: String);

    /// Retrieve's a node's name from the graph, or None if the node does not exist or is unnamed.
    fn node_name(&self, id: usize) -> Option<&String>;

    /// Retrieve's a node's name from the graph, or None if the node does not exist or does not
    /// have a value.
    fn node_value(&self, id: usize) -> Option<&dyn Any>;
}

/// Graph only for dependency injection.
pub struct InjectionGraph {
    injection: Option<Box<dyn Graph<'static>>>
}

impl InjectionGraph {
    fn inject(&mut self, graph: Box<dyn Graph<'static>>) {
        match &self.injection {
            Some(_) => panic!("Replacing existing graph with injection"),
            None => self.injection = Some(graph),
        }
    }
}

impl Graph<'static> for InjectionGraph {
    fn add_node(&mut self) -> usize {
        self.injection.as_mut()
        .expect("Initialize graph binding before use")
        .add_node()
    }

    fn set_node_value(&mut self, id: usize, value: &'static dyn Any) {
        self.injection.as_mut()
        .expect("Initialize graph binding before use")
        .set_node_value(id, value)
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        self.injection.as_mut()
        .expect("Initialize graph binding before use")
        .set_node_name(id, name)
    }

    fn node_name(&self, id: usize) -> Option<&String> {
        self.injection.as_ref()
        .expect("Initialize graph binding before use")
        .node_name(id)
    }

    fn node_value(&self, id: usize) -> Option<&dyn Any> {
        self.injection.as_ref()
        .expect("Initialize graph binding before use")
        .node_value(id)
    }
}

struct NodeInfo<'a> {
    name: Option<String>,
    value: Option<&'a dyn Any>,
}

struct InMemoryGraph<'a> {
    graph: petgraph::graph::Graph<NodeInfo<'a>, NodeIndex>,
}

impl<'a> InMemoryGraph<'a> {
    /// Constructs an empty new in-memory graph
    fn new() -> Self {
        InMemoryGraph {
            graph: petgraph::graph::Graph::<NodeInfo, NodeIndex>::new(),
        }
    }
}

impl<'a> Graph<'a> for InMemoryGraph<'a> {
    fn add_node(&mut self) -> usize {
        let new_node_info = NodeInfo {
            name: None,
            value: None,
        };
        self.graph.add_node(new_node_info).index()
    }

    fn set_node_value(&mut self, id: usize, value: &'a dyn Any) {
        self.graph.node_weight_mut(NodeIndex::new(id)).unwrap().value = Some(value);
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        self.graph.node_weight_mut(NodeIndex::new(id)).unwrap().name = Some(name);
    }

    fn node_name(&self, id: usize) -> Option<&String> {
        return self
            .graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.name.as_ref())
            .flatten();
    }

    fn node_value(&self, id: usize) -> Option<&dyn Any> {
        return self
            .graph
            .node_weight(NodeIndex::new(id))
            .map(|info| info.value)
            .flatten();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_graph_create() {
        bind_in_memory_graph();
    }

    #[test]
    fn in_memory_graph_add_node() {
        bind_in_memory_graph();
        GRAPH.with(|g_cell| {
            let mut g = g_cell.borrow_mut();
            let id = g.add_node();
            assert!(g.node_value(id).is_none());
            assert_eq!(g.node_name(id), None);
        })
    }

    #[test]
    fn in_memory_graph_set_node_value() {
        bind_in_memory_graph();
        GRAPH.with(|g_cell| {
            let mut g = g_cell.borrow_mut();
        let a_id = g.add_node();
        g.set_node_value(a_id, &5);
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
        assert_eq!(g.node_name(a_id), None);
    })
    }

    #[test]
    fn in_memory_graph_retrieve_node_string_value() {
        bind_in_memory_graph();
        GRAPH.with(|g_cell| {
            let mut g = g_cell.borrow_mut();
        let a_id = g.add_node();
        g.set_node_value(a_id, &"5");
        assert_eq!(
            g.node_value(a_id)
                .expect("entered 5")
                .downcast_ref::<&str>(),
            Some(&"5")
        );
        assert_eq!(g.node_name(a_id), None);
    })
    }

    #[test]
    fn in_memory_graph_retrieve_node_name() {
        bind_in_memory_graph();
        GRAPH.with(|g_cell| {
            let mut g = g_cell.borrow_mut();
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.node_name(a_id), Some(&"A".to_string()));
        })
    }

    #[test]
    fn in_memory_graph_retrieve_node_name_value() {
        bind_in_memory_graph();
        GRAPH.with(|g_cell| {
            let mut g = g_cell.borrow_mut();
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        g.set_node_value(a_id, &5);
        assert_eq!(g.node_name(a_id), Some(&"A".to_string()));
        assert_eq!(
            g.node_value(a_id).expect("entered 5").downcast_ref::<i32>(),
            Some(&5)
        );
    })
    }
}
