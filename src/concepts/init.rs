use super::attributes::{Attribute, HasAttributeType, Inherits, Owner, Value};
use super::{Archetype, ArchetypeTrait, Tao};
use crate::graph::{Graph, InjectionGraph};
use crate::initialize_type;

/// Adds all concepts and relations to graph.
pub fn initialize_types() {
    let mut ig = InjectionGraph::default();
    initialize_type!(
        ig,
        (
            Tao,
            Archetype,
            Attribute,
            Owner,
            Value,
            Inherits,
            HasAttributeType
        )
    );

    let mut attributes = Attribute::archetype();
    attributes.add_attribute_type(Owner::archetype());
    attributes.add_attribute_type(Value::archetype());
}
