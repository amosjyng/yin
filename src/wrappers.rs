//! Wrappers around graph nodes that provide extended functionality at every level.

mod base_wrapper;

pub use base_wrapper::{BaseNodeTrait, BaseWrapper};
use std::fmt::{Formatter, Result};

/// All wrappers around a graph node will always have these associated functions available.
pub trait CommonNodeTrait {
    /// The unique integer that's associated with this concept.
    fn id(&self) -> usize;

    /// Associate this concept with an internal name. The name does not need to be unique.
    fn set_internal_name(&mut self, name: String);

    /// The internal name that's associated with this concept, if one exists.
    fn internal_name(&self) -> Option<String>;
}

pub fn debug_wrapper(
    wrapper_type: &str,
    node: Box<&dyn CommonNodeTrait>,
    f: &mut Formatter,
) -> Result {
    match node.internal_name() {
        Some(name) => f.write_fmt(format_args!("{}({},{})", wrapper_type, node.id(), name)),
        None => f.write_fmt(format_args!("{}({})", wrapper_type, node.id())),
    }
}