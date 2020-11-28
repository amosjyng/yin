use super::archetype::{
    ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait,
};
use super::auto_init::initialize_types;
use super::relation::attribute::has_property::HasProperty;
use super::relation::attribute::{Attribute, Owner, OwnerArchetype, Value, ValueArchetype};
use super::relation::Relation;
use super::Tao;
use crate::graph::{bind_cypher_graph, bind_in_memory_graph};

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
/// use zamm_yin::tao::relation::attribute::Inherits;
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
            $g.set_node_name(<$t>::TYPE_ID, <$t>::TYPE_NAME);
        )*
        // set edges later, since edges contain references to node names, and that will be
        // impossible if the nodes themselves don't exist yet
        $($g.add_edge(<$t>::TYPE_ID, Inherits::TYPE_ID, <$t>::PARENT_TYPE_ID);)*
    };
}

/// Adds all concept relations to graph.
fn initialize_relations() {
    let mut relation = AttributeArchetype::from(Relation::TYPE_ID);
    relation.add_attribute_type(Owner::archetype());
    // Tao, not Form, here because even non-`Form`s like archetypes can have attributes
    // todo: add set_owner_archetype to Relation as well
    relation.set_owner_archetype(Tao::archetype());

    let mut attributes = Attribute::archetype();
    attributes.add_attribute_type(Value::archetype());
    attributes.set_value_archetype(Tao::archetype());

    HasProperty::archetype().set_value_archetype(Relation::archetype().into());

    Owner::archetype().set_owner_archetype(Relation::archetype().into());
    OwnerArchetype::archetype().set_owner_archetype(Relation::archetype().into());
    Value::archetype().set_owner_archetype(Attribute::archetype().into());
    ValueArchetype::archetype().set_owner_archetype(Attribute::archetype().into());
}

/// Initialize Yin with an in-memory graph database.
///
/// This not only creates the graph for Yin to act on, but also seeds the graph with initial
/// concepts and relationships.
pub fn initialize_kb() {
    bind_in_memory_graph();
    initialize_types();
    initialize_relations();
}

/// Initialize Yin with a Neo4j-backed graph database.
///
/// This not only creates the graph for Yin to act on, but also seeds the graph with initial
/// concepts and relationships.
pub fn initialize_cypher_kb(uri: &str) {
    bind_cypher_graph(uri);
    initialize_types();
    initialize_relations();
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
