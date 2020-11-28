use crate::node_wrappers::{BaseNodeTrait, FinalNode};
use crate::tao::archetype::ArchetypeTrait;
use crate::tao::form::{Form, FormTrait};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_wrappers::CommonNodeTrait;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;

    #[test]
    fn test_mark_and_check_newly_defined() {
        initialize_kb();
        let mut new_instance = Form::new();
        assert!(!new_instance.is_individual());

        new_instance.mark_individual();
        assert!(new_instance.is_individual());
    }

    #[test]
    fn test_newly_defined_inheritance() {
        initialize_kb();
        let new_type = Form::archetype().individuate_as_archetype();
        #[rustfmt::skip]
        let new_instance = Form::from(new_type.individuate_as_form().id());
        assert!(!new_instance.is_individual());

        Form::from(new_type.id()).mark_individual();
        assert!(new_instance.is_individual());
    }
}
