//! Yin is a rudimentary, experimental knowledge base.
//!
//! # Example
//!
//! ```rust
//! use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao};
//! use zamm_yin::concepts::initialize_kb;
//! use zamm_yin::node_wrappers::CommonNodeTrait;
//!
//! fn main() {
//!     // Initialize the knowledge-base
//!     initialize_kb();
//!
//!     // Create a new concept
//!     let mut concept = Tao::individuate();
//!     assert!(concept.has_ancestor(Tao::archetype()));
//!
//!     // Set a name for the concept
//!     concept.set_internal_name("Hello, world.".to_string());
//!     println!("{}", concept.internal_name().unwrap());
//! }
//! ```

#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]

pub mod concepts;
pub mod graph;
pub mod node_wrappers;
