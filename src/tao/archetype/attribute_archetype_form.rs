use super::ArchetypeFormTrait;
use super::IsArchetype;
use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::Attribute;
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Archetype representing attributes.
///
/// This can only be used to represent *attribute* archetypes, so unlike `Archetype` (which can
/// represent all archetypes, including its own archetype, because it's an archetype too),
/// `AttributeArchetype` is not an attribute and therefore it cannot implement `AttributeTrait`,
/// and cannot be used to represent its own archetype.
///
/// Note that there is a `ArchetypeFormTrait` combining the `ArchetypeTrait` and FormTrait` into
/// one, but no `AttributeArchetypeFormTrait` doing the same for `AttributeArchetypeTrait` and
/// `AttributeTrait`. This is partially because of the above reason, and partially because
/// there is no `AttributeArchetypeTrait` because all added archetype functionality is contained
/// entirely within `AttributeArchetype` itself.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeArchetype {
    base: FinalNode,
}

impl Debug for AttributeArchetype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("AttributeArchetype", self, f)
    }
}

impl From<usize> for AttributeArchetype {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for AttributeArchetype {
    fn from(fw: FinalNode) -> Self {
        Self { base: fw }
    }
}

impl<'a> TryFrom<&'a str> for AttributeArchetype {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|n| Self { base: n })
    }
}

impl Wrapper for AttributeArchetype {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for AttributeArchetype {
    type ArchetypeForm = Archetype;
    type Form = AttributeArchetype;

    const TYPE_ID: usize = 9;
    const TYPE_NAME: &'static str = "attribute-archetype";
    const PARENT_TYPE_ID: usize = Archetype::TYPE_ID;
}

impl FormTrait for AttributeArchetype {}

impl IsArchetype for AttributeArchetype {}

impl<'a> ArchetypeFormTrait<'a> for AttributeArchetype {
    type SubjectForm = Attribute;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeTrait;
    use crate::tao::initialize_kb;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Archetype::archetype().id(), Archetype::TYPE_ID);
        assert_eq!(
            Archetype::archetype().internal_name(),
            Some(Rc::new(Archetype::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Archetype::individuate();
        let concept_copy = Archetype::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Archetype::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Archetype::try_from("A"), Ok(concept));
        assert!(Archetype::try_from("B").is_err());
    }
}
