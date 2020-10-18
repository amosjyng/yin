use super::attribute::{
    Attribute, HasAttributeType, Inherits, Owner, OwnerArchetype, Value, ValueArchetype,
};
use super::{Form, Tao};
use crate::graph::{bind_cypher_graph, bind_in_memory_graph};
use crate::graph::{Graph, InjectionGraph};
use crate::tao::archetype::ArchetypeFormTrait;
use crate::tao::archetype::AttributeArchetype;
use crate::tao::archetype::{Archetype, ArchetypeTrait};

/// The maximum concept ID inside the types distributed by Yin itself. App-specific type concepts
/// should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 10;

/// Add the given Concept type to the KB.
///
/// # Examples
///
/// Note: do not actually run this on existing types, since they are automatically added when the
/// KB is initialized.
///
/// ```rust
/// # use zamm_yin::tao::initialize_kb;
/// # initialize_kb();
/// use zamm_yin::initialize_type;
/// use zamm_yin::tao::archetype::{Archetype, ArchetypeTrait};
/// use zamm_yin::tao::attribute::Inherits;
/// use zamm_yin::tao::Tao; // import your own types instead
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
            AttributeArchetype,
            Form
        )
    );

    let mut attributes = Attribute::archetype();
    attributes.add_attribute_type(Owner::archetype());
    attributes.add_attribute_type(Value::archetype());
    // Tao, not Form, here because even non-`Form`s like archetypes can have attributes
    attributes.set_owner_archetype(Tao::archetype());
    attributes.set_value_archetype(Tao::archetype());

    // todo: have yang generate init-verification tests for these
    OwnerArchetype::archetype().set_owner_archetype(Attribute::archetype().as_archetype());
    ValueArchetype::archetype().set_owner_archetype(Attribute::archetype().as_archetype());
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
        assert_eq!(g.size(), crate::tao::YIN_MAX_ID + 1); // node IDs are zero-indexed
    }
}
