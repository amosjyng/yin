use super::Graph;
use super::KBValue;
use std::rc::Rc;

/// Invalid default graph.
pub struct InvalidGraph {}

impl InvalidGraph {
    const INVALID_MSG: &'static str = "Initialize graph binding before use";
}

impl Graph for InvalidGraph {
    fn size(&self) -> usize {
        panic!(Self::INVALID_MSG);
    }

    fn add_node(&mut self) -> usize {
        panic!(Self::INVALID_MSG);
    }

    fn set_node_value(&mut self, _: usize, _: Box<dyn KBValue>) {
        panic!(Self::INVALID_MSG);
    }

    fn set_node_name(&mut self, _: usize, _: String) {
        panic!(Self::INVALID_MSG);
    }

    fn node_name(&self, _: usize) -> Option<Rc<String>> {
        panic!(Self::INVALID_MSG);
    }

    fn node_value(&self, _: usize) -> Option<Rc<Box<dyn KBValue>>> {
        panic!(Self::INVALID_MSG);
    }

    fn lookup(&self, _: &str) -> Vec<usize> {
        panic!(Self::INVALID_MSG);
    }

    fn add_edge(&mut self, _: usize, _: usize, _: usize) {
        panic!(Self::INVALID_MSG);
    }

    fn has_edge(&self, _: usize, _: usize, _: usize) -> bool {
        panic!(Self::INVALID_MSG);
    }

    fn outgoing_nodes(&self, _: usize, _: usize) -> Vec<usize> {
        panic!(Self::INVALID_MSG)
    }

    fn incoming_nodes(&self, _: usize, _: usize) -> Vec<usize> {
        panic!(Self::INVALID_MSG)
    }

    fn all_outgoing_nodes(&self, _: usize) -> Vec<usize> {
        panic!(Self::INVALID_MSG)
    }

    fn all_incoming_nodes(&self, _: usize) -> Vec<usize> {
        panic!(Self::INVALID_MSG)
    }

    fn into_dot(&self) -> String {
        panic!(Self::INVALID_MSG)
    }
}
