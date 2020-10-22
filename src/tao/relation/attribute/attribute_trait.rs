use crate::node_wrappers::BaseNodeTrait;
use crate::node_wrappers::FinalNode;
use crate::tao::archetype::ArchetypeTrait;
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::{Owner, Value};

/// Interface for all attributes.
pub trait AttributeTrait: FormTrait {
    /// The Form representing the owner.
    type OwnerForm: FormTrait + From<FinalNode>;
    /// The Form representing the value.
    type ValueForm: FormTrait + From<FinalNode>;

    /// Set the owner for this attribute.
    fn set_owner(&mut self, owner: &Self::OwnerForm) {
        self.essence_mut()
            .add_outgoing(Owner::TYPE_ID, owner.essence());
    }

    /// The owner of an attribute, if it exists.
    fn owner(&self) -> Option<Self::OwnerForm> {
        self.essence()
            .outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Self::OwnerForm::from(*n))
    }

    /// Set the value for this attribute.
    fn set_value(&mut self, value: &Self::ValueForm) {
        self.essence_mut()
            .add_outgoing(Value::TYPE_ID, value.essence());
    }

    /// The value of an attribute, if it exists.
    fn value(&self) -> Option<Self::ValueForm> {
        self.essence()
            .outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Self::ValueForm::from(*n))
    }
}
