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

pub use archetype::Archetype;
pub use archetype_trait::ArchetypeTrait;
pub use form_trait::FormTrait;
pub use init::{initialize_cypher_kb, initialize_kb, YIN_MAX_ID};
pub use tao::Tao;
