use crate::node_wrappers::{BaseNodeTrait, FinalNode};
use crate::tao::archetype::ArchetypeTrait;
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::Attribute;
use crate::tao::relation::flag::IsIndividual;
use crate::Wrapper;

/// Public trait to store eventually-automated form attributes in.
pub trait FormExtension: FormTrait + Wrapper<BaseType = FinalNode> {
    /// Mark this concept as representing an individual.
    fn mark_individual(&mut self) {
        self.essence_mut().add_flag(IsIndividual::TYPE_ID);
    }

    /// Whether this represents an individual.
    fn is_individual(&self) -> bool {
        self.essence().has_flag(IsIndividual::TYPE_ID)
    }
}

impl FormExtension for Form {}
impl FormExtension for Attribute {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;

    #[test]
    fn test_new_is_individual() {
        initialize_kb();
        let new_instance = Form::new();
        // all individuals should automatically be marked as such
        assert!(new_instance.is_individual());
    }

    #[test]
    fn test_individual_as_form_is_individual() {
        initialize_kb();
        let new_instance = Form::archetype().individuate_as_form();
        // all individuals should automatically be marked as such
        assert!(new_instance.is_individual());
    }
}
