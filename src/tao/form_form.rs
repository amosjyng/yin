use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{Archetype, ArchetypeTrait};
use crate::tao::{FormTrait, Tao};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

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

impl CommonNodeTrait for Form {
    fn id(&self) -> usize {
        self.base.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.base.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.base.internal_name()
    }
}

impl<'a> ArchetypeTrait<'a> for Form {
    type ArchetypeForm = Archetype;
    type Form = Form;

    const TYPE_ID: usize = 10;
    const TYPE_NAME: &'static str = "form";
    const PARENT_TYPE_ID: usize = Tao::TYPE_ID;
}

impl FormTrait for Form {
    fn essence(&self) -> &FinalNode {
        &self.base
    }

    fn essence_mut(&mut self) -> &mut FinalNode {
        &mut self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(Form::archetype().id(), Form::TYPE_ID);
        assert_eq!(
            Form::archetype().internal_name(),
            Some(Rc::new(Form::TYPE_NAME.to_string()))
        );
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(Form::archetype().introduced_attribute_archetypes(), vec![]);
        assert_eq!(Form::archetype().attribute_archetypes(), vec![]);
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = Form::individuate();
        let concept_copy = Form::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = Form::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!(Form::try_from("A"), Ok(concept));
        assert!(Form::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let concept1 = Form::individuate();
        let concept2 = Form::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut concept = Form::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }
}
