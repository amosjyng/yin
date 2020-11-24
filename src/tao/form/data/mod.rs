//! Data that actually exist concretely as bits on the machine, as opposed to
//! only existing as a hypothetical, as an idea.

mod data_form;
mod number_form;
mod string_concept_form;

pub use data_form::Data;
pub use number_form::Number;
pub use string_concept_form::StringConcept;
