use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::form::FormTrait;
use crate::tao::Tao;
use crate::Wrapper;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// All things that can be interacted with have form.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Form {
    base: FinalNode,
}

impl Debug for Form {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("Form", self, f)
    }
}

impl From<usize> for Form {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for Form {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for Form {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl Wrapper for Form {
    type BaseType = FinalNode;

    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

impl<'a> ArchetypeTrait<'a> for Form {
    type ArchetypeForm = Archetype;
    type Form = Form;

    const TYPE_ID: usize = 1;
    const TYPE_NAME: &'static str = "form";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Form {}

impl From<Form> for Tao {
    fn from(this: Form) -> Tao {
        Tao::from(this.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Form::archetype().id(), Form::TYPE_ID);
        assert_eq!(
            Form::archetype().internal_name_str(),
            Some(Rc::from(Form::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Form::new();
        concept.set_internal_name_str("A");
        assert_eq!(Form::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(Form::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Form::archetype().added_attributes(), vec![]);
        assert_eq!(Form::archetype().attributes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Form::new();
        let concept_copy = Form::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = Form::new();
        assert_eq!(concept.essence(), &FinalNode::from(concept.id()));
    }
}
