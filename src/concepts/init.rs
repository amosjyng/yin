use super::attributes::{
    Attribute, HasAttributeType, Inherits, Owner, OwnerArchetype, Value, ValueArchetype,
};
use super::{Archetype, ArchetypeTrait, Tao};
use crate::concepts::archetype::attribute::{AttributeArchetype, AttributeArchetypeTrait};
use crate::concepts::archetype::ArchetypeFormTrait;
use crate::graph::{bind_cypher_graph, bind_in_memory_graph};
use crate::graph::{Graph, InjectionGraph};

/// The maximum concept ID inside the types distributed by Yin itself. App-specific type concepts
/// should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 9;

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
            HasAttributeType,
            OwnerArchetype,
            ValueArchetype,
            AttributeArchetype
        )
    );

    let mut attributes = Attribute::attribute_archetype();
    attributes.add_attribute_type(AttributeArchetype::from(Owner::TYPE_ID));
    attributes.add_attribute_type(AttributeArchetype::from(Value::TYPE_ID));
    attributes.set_owner_archetype(Tao::archetype());
    attributes.set_value_archetype(Tao::archetype());

    // todo: use OwnerArchetype::attribute_archetype once yang generates that
    // todo: have yang generate init-verification tests for these
    let mut owner_archetypes = AttributeArchetype::from(OwnerArchetype::TYPE_ID);
    owner_archetypes.set_owner_archetype(Attribute::archetype());
    let mut value_archetypes = AttributeArchetype::from(ValueArchetype::TYPE_ID);
    value_archetypes.set_owner_archetype(Attribute::archetype());
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
