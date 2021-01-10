use crate::graph::{Graph, InjectionGraph};
use crate::initialize_type;
use crate::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use crate::tao::form::Form;
use crate::tao::relation::attribute::has_property::{HasAttribute, HasFlag, HasProperty};
use crate::tao::relation::attribute::{
    Attribute, Inherits, MetaForm, Owner, OwnerArchetype, Value, ValueArchetype,
};
use crate::tao::relation::flag::{Flag, IsIndividual, Meta, MultiValued, Nonhereditary};
use crate::tao::relation::Relation;
use crate::tao::Tao;

/// The maximum concept ID inside the types distributed by Yin itself. App-
/// specific type concepts should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 19;

/// Adds all concepts to knowledge graph.
pub fn initialize_types() {
    let mut ig = InjectionGraph::new();
    #[rustfmt::skip]
    initialize_type!(
        ig,
        (
            Tao,
            Form,
            Relation,
            Flag,
            Attribute,
            Owner,
            Value,
            Inherits,
            HasProperty,
            HasFlag,
            HasAttribute,
            OwnerArchetype,
            ValueArchetype,
            Archetype,
            AttributeArchetype,
            MetaForm,
            Nonhereditary,
            Meta,
            MultiValued,
            IsIndividual
        )
    );
    ig.add_edge(Relation::TYPE_ID, HasFlag::TYPE_ID, Nonhereditary::TYPE_ID);
    ig.add_edge(Relation::TYPE_ID, HasAttribute::TYPE_ID, Owner::TYPE_ID);
    ig.add_edge(Relation::TYPE_ID, OwnerArchetype::TYPE_ID, Tao::TYPE_ID);
    ig.add_edge(Attribute::TYPE_ID, HasAttribute::TYPE_ID, Value::TYPE_ID);
    ig.add_edge(Attribute::TYPE_ID, ValueArchetype::TYPE_ID, Tao::TYPE_ID);
    ig.add_edge(Owner::TYPE_ID, OwnerArchetype::TYPE_ID, Relation::TYPE_ID);
    ig.add_edge(Value::TYPE_ID, OwnerArchetype::TYPE_ID, Attribute::TYPE_ID);
    ig.add_edge(
        HasProperty::TYPE_ID,
        ValueArchetype::TYPE_ID,
        Relation::TYPE_ID,
    );
    ig.add_edge(
        OwnerArchetype::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Relation::TYPE_ID,
    );
    ig.add_edge(
        ValueArchetype::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Attribute::TYPE_ID,
    );
    ig.add_edge(
        Nonhereditary::TYPE_ID,
        OwnerArchetype::TYPE_ID,
        Relation::TYPE_ID,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;

    #[test]
    fn test_yin_size() {
        initialize_kb();
        let g = InjectionGraph::new();
        assert_eq!(g.size(), YIN_MAX_ID + 1);
    }
}
