use super::Graph;
use super::KBWrapper;
use std::rc::Rc;

/// Invalid default graph.
pub struct InvalidGraph {}

impl Graph<'static> for InvalidGraph {
    fn add_node(&mut self) -> usize {
        panic!("Initialize graph binding before use");
    }

    fn set_node_value(&mut self, _: usize, _: Box<dyn KBWrapper>) {
        panic!("Initialize graph binding before use");
    }

    fn set_node_name(&mut self, _: usize, _: String) {
        panic!("Initialize graph binding before use");
    }

    fn node_name(&self, _: usize) -> Option<String> {
        panic!("Initialize graph binding before use");
    }

    fn node_value(&self, _: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        panic!("Initialize graph binding before use");
    }

    fn add_edge(&mut self, _: usize, _: usize, _: usize) {
        panic!("Initialize graph binding before use");
    }

    fn outgoing_nodes(&self, _: usize, _: usize) -> Vec<usize> {
        panic!("Initialize graph binding before use")
    }
}
