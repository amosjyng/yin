use super::Archetype;
use super::AttributeArchetype;
use crate::node_wrappers::{BaseNodeTrait, FinalNode};
use crate::tao::archetype::ArchetypeTrait;
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::{OwnerArchetype, ValueArchetype};
use crate::tao::Tao;
use crate::Wrapper;

/// Archetype functionality that is specific to attribute archetypes.
pub trait AttributeArchetypeFormTrait<'a>:
    ArchetypeTrait<'a> + FormTrait + Wrapper<BaseType = FinalNode>
{
    /// Restrict the owners for this type of attribute.
    fn set_owner_archetype(&mut self, owner_archetype: Archetype) {
        self.essence_mut()
            .add_outgoing(OwnerArchetype::TYPE_ID, owner_archetype.essence());
    }

    /// Retrieve the owner type for this type of attribute.
    fn owner_archetype(&self) -> Archetype {
        // outgoing nodes are sorted by ID, and more specific nodes are created later, resulting in
        // higher IDs
        Archetype::from(
            *self
                .essence()
                .outgoing_nodes(OwnerArchetype::TYPE_ID)
                .last()
                .unwrap_or(&FinalNode::from(Tao::TYPE_ID)),
        )
    }

    /// Restrict the values for this type of attribute.
    fn set_value_archetype(&mut self, value_archetype: Archetype) {
        self.essence_mut()
            .add_outgoing(ValueArchetype::TYPE_ID, value_archetype.essence());
    }

    /// Retrieve the value type for this type of attribute.
    fn value_archetype(&self) -> Archetype {
        // outgoing nodes are sorted by ID, and more specific nodes are created later, resulting in
        // higher IDs
        Archetype::from(
            *self
                .essence()
                .outgoing_nodes(ValueArchetype::TYPE_ID)
                .last()
                .unwrap_or(&FinalNode::from(Tao::TYPE_ID)),
        )
    }
}

impl<'a> AttributeArchetypeFormTrait<'a> for AttributeArchetype {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Attribute;

    #[test]
    fn test_overriding_owner_archetype() {
        initialize_kb();
        let mut attr_type1 = Attribute::archetype().individuate_as_archetype();
        let attr_type2 = attr_type1.individuate_as_archetype();
        assert_eq!(attr_type2.owner_archetype(), Tao::archetype());

        // owners should now be restricted to Attributes as opposed to Tao
        attr_type1.set_owner_archetype(Attribute::archetype().as_archetype());
        assert_eq!(
            attr_type2.owner_archetype(),
            Attribute::archetype().as_archetype()
        );
    }

    #[test]
    fn test_overriding_value_archetype() {
        initialize_kb();
        let mut attr_type1 = Attribute::archetype().individuate_as_archetype();
        let attr_type2 = attr_type1.individuate_as_archetype();
        assert_eq!(attr_type2.value_archetype(), Tao::archetype());

        // values should now be restricted to Attributes as opposed to Tao
        attr_type1.set_value_archetype(Attribute::archetype().as_archetype());
        assert_eq!(
            attr_type2.value_archetype(),
            Attribute::archetype().as_archetype()
        );
    }

    #[test]
    fn test_default_owner_value_archetypes() {
        initialize_kb();
        // for example, if we have a custom attribute node that does not inherit from the existing
        // Attribute archetype because it's newly defined, like in Yang
        let custom_attr =
            AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        assert_eq!(custom_attr.owner_archetype(), Tao::archetype());
        assert_eq!(custom_attr.value_archetype(), Tao::archetype());
    }
}
