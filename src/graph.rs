//! Graph implementations

/// A classic directed Graph with nodes and labeled links.
pub trait Graph {
    /// Adds a new node with the given string name to the graph, and returns the node's ID.
    fn add_node(self, name: String) -> u32;
}
