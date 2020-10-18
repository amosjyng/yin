use crate::concepts::{FormTrait, Tao};

/// Interface for all attributes.
pub trait AttributeTrait<'a, T>: FormTrait {
    /// Set the owner for this attribute.
    fn set_owner(&mut self, owner: &dyn FormTrait);

    /// The owner of an attribute, if it exists.
    fn owner(&self) -> Option<Tao>;

    /// Set the value for this attribute.
    fn set_value(&mut self, value: &dyn FormTrait);

    /// The value of an attribute, if it exists.
    fn value(&self) -> Option<Tao>;
}
