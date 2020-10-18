//! Yin is a rudimentary, experimental knowledge base.
//!
//! # Example
//!
//! ```rust
//! use zamm_yin::tao::{initialize_kb, FormTrait, Form};
//! use zamm_yin::tao::archetype::ArchetypeTrait;
//! use zamm_yin::node_wrappers::CommonNodeTrait;
//!
//! fn main() {
//!     // Initialize the knowledge-base
//!     initialize_kb();
//!
//!     // Create a new concept
//!     let mut concept = Form::individuate();
//!     assert!(concept.has_ancestor(Form::archetype()));
//!
//!     // Set a name for the concept
//!     concept.set_internal_name("Hello, world.".to_string());
//!     println!("{}", concept.internal_name().unwrap());
//! }
//! ```

#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]

pub mod graph;
pub mod node_wrappers;
pub mod tao;
