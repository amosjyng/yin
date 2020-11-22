use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::{Form, FormTrait};
use crate::Wrapper;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};

/// Data that actually exist concretely as bits on the machine, as opposed to
/// only existing as a hypothetical, as an idea.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Data {
    base: FinalNode,
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Data", self, f)
    }
}

impl From<usize> for Data {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Data {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Data {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Data {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Data {
    type ArchetypeForm = Archetype;
    type Form = Data;

    const TYPE_ID: usize = 13;
    const TYPE_NAME: &'static str = "data";
    const PARENT_TYPE_ID: usize = Form::TYPE_ID;
}

impl FormTrait for Data {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::form::FormTrait;
    use crate::tao::initialize_kb;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Data::archetype().id(), Data::TYPE_ID);
        assert_eq!(
            Data::archetype().internal_name_str(),
            Some(Rc::from(Data::TYPE_NAME))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Data::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Data::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Data::new();
        let concept_copy = Data::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Data::new();
        concept.set_internal_name_str("A");
        assert_eq!(Data::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Data::try_from("B").is_err());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Data::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
