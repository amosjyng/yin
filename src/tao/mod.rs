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
//! use zamm_yin::tao::initialize_kb;
//!
//! initialize_kb();
//! ```
//!
//! Now, we can create a new concept:
//!
//! ```rust
//! # use zamm_yin::tao::initialize_kb;
//! # initialize_kb();
//! use zamm_yin::tao::archetype::ArchetypeTrait;
//! use zamm_yin::tao::form::{Form, FormTrait};
//!
//! let mut concept = Form::new();
//! assert!(concept.has_ancestor(Form::archetype()));
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use zamm_yin::tao::initialize_kb;
//! # use zamm_yin::tao::archetype::ArchetypeTrait;
//! # use zamm_yin::tao::form::Form;
//! # initialize_kb();
//! # let mut concept = Form::new();
//! use zamm_yin::node_wrappers::CommonNodeTrait;
//! use std::rc::Rc;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
//! ```

/// Types of forms, as opposed to the forms themselves.
pub mod archetype;
/// Relations between the forms.
pub mod relation {
    mod relation_form;
    pub use relation_form::Relation;

    /// Relations involving only one form.
    pub mod flag {
        mod flag_form;
        pub use flag_form::Flag;
    }

    /// Relations between two forms.
    pub mod attribute {
        mod attribute_form;
        mod attribute_trait;
        mod default_value_form;
        mod has_property_form;
        mod inherits_form;
        mod owner_archetype_form;
        mod owner_form;
        mod value_archetype_form;
        mod value_form;

        /// Marker for attributes, for compile-time checks.
        trait IsAttribute {}

        pub use attribute_form::Attribute;
        pub use attribute_trait::AttributeTrait;
        pub use default_value_form::DefaultValue;
        pub use has_property_form::HasProperty;
        pub use inherits_form::Inherits;
        pub use owner_archetype_form::OwnerArchetype;
        pub use owner_form::Owner;
        pub use value_archetype_form::ValueArchetype;
        pub use value_form::Value;
    }
}
/// Concept forms, as opposed to archetypes.
pub mod form {
    /// Concepts that exist explicitly as bits.
    pub mod data {
        mod data_form;
        mod number_form;
        mod string_concept_form;

        pub use data_form::Data;
        pub use number_form::Number;
        pub use string_concept_form::StringConcept;
    }

    mod form_form;
    mod form_trait;

    pub use form_form::Form;
    pub use form_trait::FormTrait;
}
mod auto_init;
mod init;
mod tao_form;

pub use auto_init::YIN_MAX_ID;
pub use init::{initialize_cypher_kb, initialize_kb};
pub use tao_form::Tao;
