use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, Archetype, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::Attribute;
use crate::tao::relation::flag::IsIndividual;
use crate::Wrapper;

/// Public trait to store eventually-automated form attributes in.
/// 
/// This differs from ArchetypeTrait in that ArchetypeTrait applies to the class, but these
/// functions apply to individual instances of the class.
pub trait FormExtension: FormTrait + Wrapper<BaseType = FinalNode> + CommonNodeTrait {
    /// What meta perspective will be used to represent this by default.
    type MetaType: From<usize>;

    /// Mark this concept as representing an individual.
    fn mark_individual(&mut self) {
        self.essence_mut().add_flag(IsIndividual::TYPE_ID);
    }

    /// Whether this represents an individual.
    fn is_individual(&self) -> bool {
        self.essence().has_flag(IsIndividual::TYPE_ID)
    }

    /// View the current node from its meta perspective.
    fn meta(&self) -> Self::MetaType {
        Self::MetaType::from(self.id())
    }
}

impl FormExtension for Form {
    type MetaType = Archetype;
}

impl FormExtension for Attribute {
    type MetaType = AttributeArchetype;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::ArchetypeFormTrait;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::Owner;

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

    #[test]
    fn test_query_meta() {
        initialize_kb();
        // todo: use Owner::new() directly after `FormExtension` gets auto-generated for all 
        // descendants in future version of Yang
        let new_attr = Attribute::from(Owner::new().id());
        // todo: in the future, check that OwnerArchetype is not in this list, because that 
        // attribute belongs to the meta-object. The information will still be associated with the 
        // object node -- Owner will still have an OwnerArchetype. It's just that the Owner 
        // perspective does not include OwnerArchetype and does not know what to do with it -- but 
        // the meta-perspective for Owner (aka the AttributeArchetype perspective) does.
        assert!(new_attr.meta().attributes().contains(&Owner::archetype()));
    }
}
