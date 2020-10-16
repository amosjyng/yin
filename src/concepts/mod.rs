//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//!
//! The `wrappers` module provides additional low-level capabilities for nodes, above and beyond
//! those of regular graphs. This module abstracts away those low-level capabilities and greatly
//! cuts down on the possibilities, in exchange for offering much stronger compile-time checks on
//! which edge-node combinations are semantically meaningful.
//!
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.
//!
//! # Examples
//!
//! We need to choose which graph implementation to ground our knowledge and reasoning on. All
//! implementations should be logically equivalent. Let's use the in-memory one for simplicity:
//!
//! ```rust
//! use zamm_yin::concepts::initialize_kb;
//!
//! initialize_kb();
//! ```
//!
//! Now, we can create a new concept:
//!
//! ```rust
//! # use zamm_yin::concepts::initialize_kb;
//! # initialize_kb();
//! use zamm_yin::concepts::{Tao, ArchetypeTrait, FormTrait};
//!
//! let mut concept = Tao::individuate();
//! assert!(concept.has_ancestor(Tao::archetype()));
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use zamm_yin::concepts::{Tao, ArchetypeTrait};
//! # use zamm_yin::concepts::initialize_kb;
//! # initialize_kb();
//! # let mut concept = Tao::individuate();
//! use zamm_yin::node_wrappers::CommonNodeTrait;
//! use std::rc::Rc;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
//! ```

mod archetype;
mod archetype_trait;
pub mod attributes;
/// Concept forms, as opposed to archetypes.
mod form_trait;
mod init;
mod tao;

use crate::graph::{bind_cypher_graph, bind_in_memory_graph};
pub use archetype::Archetype;
pub use archetype_trait::ArchetypeTrait;
pub use form_trait::FormTrait;
use init::initialize_types;
pub use tao::Tao;

/// The maximum concept ID inside the types distributed by Yin itself. App-specific type concepts
/// should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 6;

/// Add the given Concept type to the KB.
///
/// # Examples
///
/// Note: do not actually run this on existing types, since they are automatically added when the
/// KB is initialized.
///
/// ```rust
/// # use zamm_yin::concepts::initialize_kb;
/// # initialize_kb();
/// use zamm_yin::initialize_type;
/// use zamm_yin::concepts::ArchetypeTrait;
/// use zamm_yin::concepts::attributes::Inherits;
/// use zamm_yin::concepts::{Archetype, Tao}; // import your own types instead
/// use zamm_yin::graph::{Graph, InjectionGraph};
///
/// let mut ig = InjectionGraph::new();
/// initialize_type!(ig, (Archetype, Tao));
/// ```
#[macro_export]
macro_rules! initialize_type {
    ($g:expr, ($($t:ty),*)) => {
        $(
            $g.add_node();
            $g.set_node_name(<$t>::TYPE_ID, <$t>::TYPE_NAME.to_string());
        )*
        // set edges later, since edges contain references to node names, and that will be
        // impossible if the nodes themselves don't exist yet
        $($g.add_edge(<$t>::TYPE_ID, Inherits::TYPE_ID, <$t>::PARENT_TYPE_ID);)*
    };
}

/// Initialize Yin with an in-memory graph database.
///
/// This not only creates the graph for Yin to act on, but also seeds the graph with initial
/// concepts and relationships.
pub fn initialize_kb() {
    bind_in_memory_graph();
    initialize_types();
}

/// Initialize Yin with a Neo4j-backed graph database.
///
/// This not only creates the graph for Yin to act on, but also seeds the graph with initial
/// concepts and relationships.
pub fn initialize_cypher_kb(uri: &str) {
    bind_cypher_graph(uri);
    initialize_types();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Graph, InjectionGraph};

    #[test]
    fn test_yin_size() {
        initialize_kb();
        let g = InjectionGraph::new();
        assert_eq!(g.size(), crate::concepts::YIN_MAX_ID + 1); // node IDs are zero-indexed
    }
}
