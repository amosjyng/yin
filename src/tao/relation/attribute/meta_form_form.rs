use crate::node_wrappers::{debug_wrapper, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::{Attribute, AttributeTrait};
use crate::tao::relation::Relation;
use crate::tao::Tao;
use std::convert::{From, TryFrom};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// Archetype associated with this form. This differs from parents, because this
/// defines the form's meta-properties, whereas parents define the form's
/// inherited properties.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetaForm {
    base: FinalNode,
}

impl Debug for MetaForm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("MetaForm", self, f)
    }
}

impl From<usize> for MetaForm {
    fn from(id: usize) -> Self {
        Self {
            base: FinalNode::from(id),
        }
    }
}

impl From<FinalNode> for MetaForm {
    fn from(f: FinalNode) -> Self {
        Self { base: f }
    }
}

impl<'a> TryFrom<&'a str> for MetaForm {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        FinalNode::try_from(name).map(|f| Self { base: f })
    }
}

impl ArchetypeTrait for MetaForm {
    type ArchetypeForm = AttributeArchetype;
    type Form = MetaForm;

    const TYPE_ID: usize = 15;
    const TYPE_NAME: &'static str = "meta-form";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;
}

impl Deref for MetaForm {
    type Target = FinalNode;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for MetaForm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FormTrait for MetaForm {}

impl From<MetaForm> for Tao {
    fn from(this: MetaForm) -> Tao {
        Tao::from(this.base)
    }
}

impl From<MetaForm> for Relation {
    fn from(this: MetaForm) -> Relation {
        Relation::from(this.base)
    }
}

impl From<MetaForm> for Attribute {
    fn from(this: MetaForm) -> Attribute {
        Attribute::from(this.base)
    }
}

impl AttributeTrait for MetaForm {
    type OwnerForm = Form;
    type ValueForm = Form;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::{ArchetypeFormTrait, AttributeArchetypeFormTrait};
    use crate::tao::relation::attribute::{Owner, Value};
    use crate::tao::{initialize_kb, Tao};
    use std::rc::Rc;

    #[test]
    fn check_type_created() {
        initialize_kb();
        assert_eq!(MetaForm::archetype().id(), MetaForm::TYPE_ID);
        assert_eq!(
            MetaForm::archetype().internal_name(),
            Some(Rc::from(MetaForm::TYPE_NAME))
        );
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut concept = MetaForm::new();
        concept.set_internal_name("A");
        assert_eq!(MetaForm::try_from("A").map(|c| c.id()), Ok(concept.id()));
        assert!(MetaForm::try_from("B").is_err());
    }

    #[test]
    fn check_type_attributes() {
        initialize_kb();
        assert_eq!(MetaForm::archetype().added_attributes(), vec![]);
        assert_eq!(
            MetaForm::archetype().attributes(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let concept = MetaForm::new();
        let concept_copy = MetaForm::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }

    #[test]
    fn test_wrapper_implemented() {
        initialize_kb();
        let concept = MetaForm::new();
        assert_eq!(concept.deref(), &FinalNode::from(concept.id()));
    }

    #[test]
    #[allow(clippy::useless_conversion)]
    fn check_attribute_constraints() {
        initialize_kb();
        assert_eq!(
            MetaForm::archetype().owner_archetype(),
            Tao::archetype().into()
        );
        assert_eq!(
            MetaForm::archetype().value_archetype(),
            Tao::archetype().into()
        );
    }

    #[test]
    fn get_owner() {
        initialize_kb();
        let mut instance = MetaForm::new();
        let owner_of_instance = Tao::new();
        instance.set_owner(&owner_of_instance);
        assert_eq!(instance.owner(), Some(owner_of_instance));
        assert_eq!(instance.value(), None);
    }

    #[test]
    fn get_value() {
        initialize_kb();
        let mut instance = MetaForm::new();
        let value_of_instance = Tao::new();
        instance.set_value(&value_of_instance);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_instance));
    }
}
