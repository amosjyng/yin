use crate::graph::{Graph, InjectionGraph};
use crate::initialize_type;
use crate::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use crate::tao::form::data::{Data, Number, StringConcept};
use crate::tao::form::Form;
#[rustfmt::skip]
use crate::tao::relation::attribute::{Attribute, DefaultValue, HasProperty, Inherits, Owner, OwnerArchetype, Value, ValueArchetype};
use crate::tao::relation::flag::Flag;
use crate::tao::relation::Relation;
use crate::tao::Tao;

/// The maximum concept ID inside the types distributed by Yin itself. App-
/// specific type concepts should continue their numbering on top of this.
pub const YIN_MAX_ID: usize = 16;

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
            OwnerArchetype,
            ValueArchetype,
            Archetype,
            AttributeArchetype,
            Data,
            StringConcept,
            Number,
            DefaultValue
        )
    );
}
