use super::Graph;
use super::KBWrapper;
use std::rc::Rc;

/// Invalid default graph.
pub struct InvalidGraph {}

impl Graph for InvalidGraph {
    fn add_node(&mut self) -> usize {
        panic!("Initialize graph binding before use");
    }

    fn set_node_value(&mut self, _: usize, _: Box<dyn KBWrapper>) {
        panic!("Initialize graph binding before use");
    }

    fn set_node_name(&mut self, _: usize, _: String) {
        panic!("Initialize graph binding before use");
    }

    fn node_name(&self, _: usize) -> Option<Rc<String>> {
        panic!("Initialize graph binding before use");
    }

    fn node_value(&self, _: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        panic!("Initialize graph binding before use");
    }

    fn add_edge(&mut self, _: usize, _: usize, _: usize) {
        panic!("Initialize graph binding before use");
    }

    fn has_edge(&self, _: usize, _: usize, _: usize) -> bool {
        panic!("Initialize graph binding before use");
    }

    fn outgoing_nodes(&self, _: usize, _: usize) -> Vec<usize> {
        panic!("Initialize graph binding before use")
    }

    fn incoming_nodes(&self, _: usize, _: usize) -> Vec<usize> {
        panic!("Initialize graph binding before use")
    }

    fn all_outgoing_nodes(&self, _: usize) -> Vec<usize> {
        panic!("Initialize graph binding before use")
    }

    fn all_incoming_nodes(&self, _: usize) -> Vec<usize> {
        panic!("Initialize graph binding before use")
    }

    fn into_dot(&self) -> String {
        panic!("Initialize graph binding before use")
    }
}
