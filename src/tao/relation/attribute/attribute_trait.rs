use crate::node_wrappers::{BaseNodeTrait, FinalNode};
use crate::tao::archetype::ArchetypeTrait;
use crate::tao::form::FormTrait;
use crate::tao::relation::attribute::{Owner, Value};
use std::ops::{Deref, DerefMut};

/// Interface for all attributes.
pub trait AttributeTrait<'a>: FormTrait<'a> + Deref<Target = FinalNode> + DerefMut {
    /// The Form representing the owner.
    type OwnerForm: FormTrait<'a> + From<FinalNode>;
    /// The Form representing the value.
    type ValueForm: FormTrait<'a> + From<FinalNode>;

    /// Set the owner for this attribute.
    fn set_owner(&mut self, owner: &Self::OwnerForm) {
        self.add_outgoing(Owner::TYPE_ID, &owner);
    }

    /// The owner of an attribute, if it exists.
    fn owner(&self) -> Option<Self::OwnerForm> {
        self.outgoing_nodes(Owner::TYPE_ID)
            .get(0)
            .map(|n| Self::OwnerForm::from(*n))
    }

    /// Set the value for this attribute.
    fn set_value(&mut self, value: &Self::ValueForm) {
        self.add_outgoing(Value::TYPE_ID, &value);
    }

    /// The value of an attribute, if it exists.
    fn value(&self) -> Option<Self::ValueForm> {
        self.outgoing_nodes(Value::TYPE_ID)
            .get(0)
            .map(|n| Self::ValueForm::from(*n))
    }
}
