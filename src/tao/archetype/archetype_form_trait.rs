use super::Archetype;
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::has_property::HasAttribute;
use crate::tao::relation::attribute::{Attribute, Inherits};
use crate::Wrapper;
use std::collections::{HashSet, VecDeque};

/// Every concept represents a different way of looking at and manipulating the world. This one
/// allows one to treat an archetype -- nothing more than an idea, a piece of *meta*data -- as if
/// it had form, as if it were actual data. But of course metadata is also data, and if you look
/// around in the KB, this class definition is no different from any other class definition.
///
/// In fact, with the exception of Tao and Archetype, you're pretty much only ever viewing a node
/// from another node's perspective. When you manipulate an individual, you do so according to the
/// logic defined in its archetype. When you manipulate its archetype, you do so according to the
/// logic defined in the `Archetype` node.
///
/// Keep in mind that due to the melding of archetype and form in all Archetype structs, any   
/// references to `self` here refers to the node-as-archetype in question, whereas any references
/// to `Self` refers to the Archetype node itself. Since this FormTrait is supposed to reason about
/// the node-as-archetype, **there should be no instances of `Self` here**.
pub trait ArchetypeFormTrait<'a>:
    ArchetypeTrait<'a> + FormTrait + Wrapper<BaseType = FinalNode>
{
    /// The ArchetypeTrait as defined for an Archetype will have an Archetype-based Form for
    /// reasoning about other nodes as archetypes. The Archetype's Form is the observer, and the
    /// subject under observation will have a different type for its leaves. This subject's Form
    /// should therefore be the most specific FormTrait that is still general enough to represent
    /// everything this ArchetypeTrait can observe.
    ///
    /// Here, Self::ArchetypeForm should never be used, Self::Form is the self as the observer, and
    /// Self::SubjectForm is the subject archetype that is currently being observed.
    type SubjectForm: ArchetypeTrait<'a> + FormTrait;

    /// Forget everything about the current form, except that it's an ArchetypeForm representing
    /// some type.
    fn as_archetype(&self) -> Archetype {
        Archetype::from(*self.essence())
    }

    /// Create a subtype of the archetype represented by this Archetype instance.
    fn individuate_as_archetype(&self) -> Self::Form {
        Self::Form::from(FinalNode::new_with_inheritance(self.id()))
    }

    /// Create a new individual of the archetype represented by this Archetype instance.
    ///
    /// Convenience function for the static one.
    fn individuate_as_form(&self) -> Self::SubjectForm {
        Self::SubjectForm::from(FinalNode::new_with_inheritance(self.id()))
    }

    /// Individuals that adhere to this archetype. It is possible that some of these individuals
    /// might not be direct descendants of the archetype in question.
    fn individuals(&self) -> Vec<Self::SubjectForm> {
        let mut visited: HashSet<FinalNode> = HashSet::new();
        visited.insert(*self.essence());
        let mut to_be_visited: VecDeque<FinalNode> = VecDeque::new();
        to_be_visited.push_back(*self.essence());
        let mut leaves: Vec<FinalNode> = Vec::new();
        while let Some(next) = to_be_visited.pop_front() {
            let children = next.incoming_nodes(Inherits::TYPE_ID);
            if children.is_empty() {
                leaves.push(next);
            } else {
                for child in next.incoming_nodes(Inherits::TYPE_ID) {
                    if !visited.contains(&child) {
                        visited.insert(child);
                        to_be_visited.push_back(child);
                    }
                }
            }
        }
        let mut result: Vec<Self::SubjectForm> = leaves
            .into_iter()
            .filter(|l| l != self.essence()) // never return self, even if it's the only leaf
            .map(Self::SubjectForm::from)
            .collect();
        result.sort();
        result
    }

    /// Retrieve child archetypes.
    fn child_archetypes(&self) -> Vec<Self::Form> {
        self.essence()
            .incoming_nodes(Inherits::TYPE_ID)
            .into_iter()
            .map(Self::Form::from)
            .collect()
    }

    /// Add an attribute type to this archetype.
    fn add_attribute_type(&mut self, attribute_type: AttributeArchetype) {
        self.essence_mut()
            .add_outgoing(HasAttribute::TYPE_ID, attribute_type.essence());
    }

    /// Retrieve non-inherited attribute types that are introduced by this archetype to all
    /// descendant archetypes. Attribute types introduced by an ancestor do not count.
    fn introduced_attribute_archetypes(&self) -> Vec<AttributeArchetype> {
        self.essence()
            .base_wrapper()
            .outgoing_nodes(HasAttribute::TYPE_ID)
            .into_iter()
            .map(|n| AttributeArchetype::from(n.id()))
            .filter(|a| a.has_ancestor(Attribute::archetype().as_archetype()))
            .collect()
    }
}

impl<'a> ArchetypeFormTrait<'a> for Archetype {
    type SubjectForm = Form;
}

impl<'a> ArchetypeFormTrait<'a> for AttributeArchetype {
    type SubjectForm = Attribute;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::{ArchetypeFormExtensionTrait, ArchetypeTrait, AttributeArchetype};
    use crate::tao::form::Form;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{Attribute, Owner, Value};
    use crate::tao::relation::flag::Flag;

    #[test]
    fn test_individuation() {
        initialize_kb();
        let type1 = Owner::archetype().individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        assert!(type1.has_ancestor(Owner::archetype().as_archetype()));
        assert!(!type1.has_ancestor(Value::archetype().as_archetype()));
        assert!(type1_instance.has_ancestor(type1.as_archetype()));
        assert!(type1_instance.has_ancestor(Owner::archetype().as_archetype()));
        assert!(!type1_instance.has_ancestor(Value::archetype().as_archetype()));
    }

    #[test]
    fn test_individuals() {
        initialize_kb();
        let type1 = Form::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        let type2_instance = type2.individuate_as_form();
        assert_eq!(type1.individuals(), vec![type1_instance, type2_instance]);
        assert_eq!(type2.individuals(), vec![type2_instance]);
    }

    #[test]
    fn test_individuals_not_self() {
        initialize_kb();
        let childless_type = Form::archetype().individuate_as_archetype();
        assert_eq!(childless_type.individuals(), Vec::<Form>::new())
    }

    #[test]
    fn test_child_archetypes() {
        initialize_kb();
        let type1 = Form::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        assert_eq!(type1.child_archetypes(), vec![type2, type3]);
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        assert_eq!(
            type1.introduced_attribute_archetypes(),
            Vec::<AttributeArchetype>::new()
        );

        type1.add_attribute_type(type2);
        assert_eq!(type1.introduced_attribute_archetypes(), vec!(type2));
    }

    #[test]
    fn test_attribute_types_not_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);

        assert_eq!(
            type3.introduced_attribute_archetypes(),
            Vec::<AttributeArchetype>::new()
        );
    }

    #[test]
    fn test_attributes_no_flags() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let flag_type = Flag::archetype().individuate_as_archetype();
        let attr_type = Attribute::archetype().individuate_as_archetype();
        form_type.add_flag(flag_type);
        form_type.add_attribute_type(attr_type);

        assert_eq!(form_type.attribute_archetypes(), vec![attr_type]);
        assert_eq!(form_type.introduced_attribute_archetypes(), vec![attr_type]);
    }
}
