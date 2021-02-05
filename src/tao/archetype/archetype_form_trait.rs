use super::Archetype;
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait};
use crate::tao::form::{Form, FormTrait};
use crate::tao::relation::attribute::has_property::{HasAttribute, HasFlag};
use crate::tao::relation::attribute::{Inherits, MetaForm};
use std::collections::{HashSet, VecDeque};
use std::ops::{Deref, DerefMut};

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
pub trait ArchetypeFormTrait:
    ArchetypeTrait + FormTrait + Deref<Target = FinalNode> + DerefMut
{
    /// The ArchetypeTrait as defined for an Archetype will have an Archetype-based Form for
    /// reasoning about other nodes as archetypes. The Archetype's Form is the observer, and the
    /// subject under observation will have a different type for its leaves. This subject's Form
    /// should therefore be the most specific FormTrait that is still general enough to represent
    /// everything this ArchetypeTrait can observe.
    ///
    /// Here, Self::ArchetypeForm should never be used, Self::Form is the self as the observer, and
    /// Self::SubjectForm is the subject archetype that is currently being observed.
    type SubjectForm: ArchetypeTrait + FormTrait;

    /// Create a subtype of the archetype represented by this Archetype instance.
    fn individuate_as_archetype(&self) -> Self::Form {
        Self::Form::from(FinalNode::new_with_inheritance(self.id()))
    }

    /// Create a new individual of the archetype represented by this Archetype instance.
    ///
    /// Convenience function for the static one.
    fn individuate_as_form(&self) -> Self::SubjectForm {
        let mut result = Self::SubjectForm::from(FinalNode::new_with_inheritance(self.id()));
        result.mark_individual();
        result
    }

    /// Individuals that adhere to this archetype. It is possible that some of these individuals
    /// might not be direct descendants of the archetype in question.
    fn individuals(&self) -> Vec<Self::SubjectForm> {
        let mut visited: HashSet<FinalNode> = HashSet::new();
        visited.insert(*self.deref());
        let mut to_be_visited: VecDeque<FinalNode> = VecDeque::new();
        to_be_visited.push_back(*self.deref());
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
            .filter(|l| l != self.deref()) // never return self, even if it's the only leaf
            .map(Self::SubjectForm::from)
            .collect();
        result.sort();
        result
    }

    /// Retrieve child archetypes.
    fn child_archetypes(&self) -> Vec<Self::Form> {
        self.incoming_nodes(Inherits::TYPE_ID)
            .into_iter()
            .filter(|f| !Form::from(*f).is_individual())
            .map(Self::Form::from)
            .collect()
    }

    /// Add an attribute type to this archetype.
    fn add_attribute(&mut self, attribute_type: &AttributeArchetype) {
        self.add_outgoing(HasAttribute::TYPE_ID, attribute_type);
    }

    /// Retrieve non-inherited attribute types that are introduced by this archetype to all
    /// descendant archetypes. Attribute types introduced by an ancestor do not count.
    fn added_attributes(&self) -> Vec<AttributeArchetype> {
        self.base_wrapper()
            .outgoing_nodes(HasAttribute::TYPE_ID)
            .into_iter()
            .filter(|n| !Form::from(n.id()).is_individual())
            .map(|n| AttributeArchetype::from(n.id()))
            .collect()
    }

    /// Get all the types of attributes that this concept is predefined to potentially have.
    fn attributes(&self) -> Vec<AttributeArchetype> {
        self.outgoing_nodes(HasAttribute::TYPE_ID)
            .into_iter()
            .map(AttributeArchetype::from)
            .collect()
    }

    /// Checks to see if an archetype is one of the possible attribute types this concept could
    /// have.
    fn has_attribute(&self, possible_type: &AttributeArchetype) -> bool {
        self.has_outgoing(HasAttribute::TYPE_ID, &possible_type)
    }

    /// Opposite of a form's `meta_archetype`. This retrieves the form that this meta represents.
    ///
    /// Given the lack of a conventional antonym to "meta", this uses
    /// "[mesa](https://www.gwiznlp.com/wp-content/uploads/2014/08/Whats-the-opposite-of-meta.pdf)"
    /// as a proposed antonym.
    fn mesa_archetype(&self) -> Archetype {
        // todo: this is an archetype-specific attribute. There should therefore be an archetype
        // for archetypes
        Archetype::from(
            self.incoming_nodes(MetaForm::TYPE_ID)
                .last()
                .unwrap_or(&FinalNode::from(Archetype::TYPE_ID))
                .id(),
        )
    }

    /// If this archetype has attributes, then the attribute values will be of a certain archetype.
    /// This function returns that archetype for the specified attribute.
    fn attribute_form_archetype(&self, attribute: &AttributeArchetype) -> Option<Archetype> {
        attribute
            .attribute_form_archetype_override()
            .map(|override_attr| {
                self.outgoing_nodes(override_attr.id())
                    .last()
                    .map(|form_type| Archetype::from(form_type.id()))
            })
            .flatten()
    }

    /// If this archetype has attributes, then the attribute values will be of a certain archetype.
    /// This function sets that archetype for the specified attribute.
    fn set_attribute_form_archetype(
        &mut self,
        attribute: &AttributeArchetype,
        form_type: &Archetype,
    ) {
        let override_attr = attribute.attribute_form_archetype_override().unwrap();
        self.add_outgoing(override_attr.id(), &form_type)
    }

    /// Get all the types of flags that this type of concept is predefined to potentially have.
    fn flags(&self) -> Vec<Archetype> {
        self.outgoing_nodes(HasFlag::TYPE_ID)
            .into_iter()
            .map(Archetype::from)
            .collect()
    }

    /// Checks to see if this type of concept is predefined to have this as a flag.
    fn has_flag(&self, possible_type: &Archetype) -> bool {
        self.has_outgoing(HasFlag::TYPE_ID, &possible_type)
    }

    /// Add a flag type to this archetype.
    fn add_flag(&mut self, attribute_type: &Archetype) {
        self.add_outgoing(HasFlag::TYPE_ID, &attribute_type);
    }

    /// Retrieve non-inherited flag types that are introduced by this archetype to all descendant
    /// archetypes. Flag types introduced by an ancestor do not count.
    fn added_flags(&self) -> Vec<Archetype> {
        self.base_wrapper()
            .outgoing_nodes(HasFlag::TYPE_ID)
            .into_iter()
            .map(|n| Archetype::from(n.id()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype};
    use crate::tao::form::Form;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{Attribute, Owner, Value};
    use crate::tao::relation::flag::Flag;
    use crate::tao::Tao;

    #[test]
    fn test_individuation() {
        initialize_kb();
        let type1 = Owner::archetype().individuate_as_archetype();
        let type1_instance = type1.individuate_as_form();
        assert!(type1.has_ancestor(Owner::archetype().into()));
        assert!(!type1.has_ancestor(Value::archetype().into()));
        assert!(type1_instance.has_ancestor(type1.into()));
        assert!(type1_instance.has_ancestor(Owner::archetype().into()));
        assert!(!type1_instance.has_ancestor(Value::archetype().into()));
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
    fn test_child_archetypes_no_individuals() {
        initialize_kb();
        let type1 = Form::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        type1.individuate_as_form();
        let type3 = type1.individuate_as_archetype();
        assert_eq!(type1.child_archetypes(), vec![type2, type3]);
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        assert_eq!(type1.added_attributes(), Vec::<AttributeArchetype>::new());

        type1.add_attribute(&type2);
        assert_eq!(type1.added_attributes(), vec!(type2));
    }

    #[test]
    fn test_attribute_equivalents() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Tao::archetype().individuate_as_archetype();
        let type2_attr_arch = AttributeArchetype::from(type2.id());
        type1.add_attribute(&type2_attr_arch);

        assert_eq!(type1.attributes(), vec![type2_attr_arch]);
        assert_eq!(type1.added_attributes(), vec![type2_attr_arch]);
    }

    #[test]
    fn test_attribute_types_inherited() {
        initialize_kb();
        let mut type1 = Attribute::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute(&type2);

        assert_eq!(
            type3.attributes(),
            vec![Owner::archetype(), Value::archetype(), type2]
        );
        assert!(!type3.has_attribute(&type1));
        assert!(type3.has_attribute(&type2));
    }

    #[test]
    fn test_attribute_types_not_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute(&type2);

        assert_eq!(type3.added_attributes(), Vec::<AttributeArchetype>::new());
    }

    #[test]
    fn test_attributes_no_flags() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let flag_type = Flag::archetype().individuate_as_archetype();
        let attr_type = Attribute::archetype().individuate_as_archetype();
        form_type.add_flag(&flag_type);
        form_type.add_attribute(&attr_type);

        assert_eq!(form_type.attributes(), vec![attr_type]);
        assert_eq!(form_type.added_attributes(), vec![attr_type]);
    }

    #[test]
    fn test_attr_value_type_override() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let mut attr_type = Attribute::archetype().individuate_as_archetype();
        let attr_archetype_type = Attribute::archetype().individuate_as_archetype();
        attr_type.set_attribute_form_archetype_override(&attr_archetype_type);
        let attr_value_type = Form::archetype().individuate_as_archetype();
        assert_eq!(form_type.attribute_form_archetype(&attr_type), None);

        form_type.set_attribute_form_archetype(&attr_type, &attr_value_type);
        assert_eq!(
            form_type.attribute_form_archetype(&attr_type),
            Some(attr_value_type)
        );
    }

    #[test]
    fn test_infra_archetype() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let meta_type = Archetype::archetype().individuate_as_archetype();
        form_type.set_meta_archetype(&meta_type);
        let form_type2 = form_type.individuate_as_archetype();
        let mut form_type3 = form_type2.individuate_as_archetype();
        let meta_type3 = form_type3.specific_meta();

        assert_eq!(meta_type.mesa_archetype(), form_type);
        assert_eq!(meta_type3.mesa_archetype(), form_type3);
    }

    #[test]
    fn test_added_flags() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        assert_eq!(type1.added_flags(), Vec::<Archetype>::new());

        type1.add_flag(&type2);
        assert_eq!(type1.added_flags(), vec!(type2));
    }

    #[test]
    fn test_added_flags_not_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_flag(&type2);

        assert_eq!(type3.added_flags(), Vec::<Archetype>::new());
    }

    #[test]
    fn test_flags() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        type1.add_flag(&type2);

        assert_eq!(type1.flags(), vec![type2]);
        assert!(!type1.has_flag(&type1));
        assert!(type1.has_flag(&type2));
    }

    #[test]
    fn test_flag_equivalents() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Tao::archetype().individuate_as_archetype();
        type1.add_flag(&type2);

        assert_eq!(type1.flags(), vec![type2]);
        assert_eq!(type1.added_flags(), vec![type2]);
        assert!(!type1.has_flag(&type1));
        assert!(type1.has_flag(&type2));
    }

    #[test]
    fn test_flags_inherited() {
        initialize_kb();
        let mut type1 = Form::archetype().individuate_as_archetype();
        let type2 = Flag::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_flag(&type2);

        assert_eq!(type3.flags(), vec![type2]);
        assert!(!type3.has_flag(&type1));
        assert!(type3.has_flag(&type2));
    }

    #[test]
    fn test_flags_no_attributes() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let flag_type = Flag::archetype().individuate_as_archetype();
        let attr_type = Attribute::archetype().individuate_as_archetype();
        form_type.add_flag(&flag_type);
        form_type.add_attribute(&attr_type);

        assert_eq!(form_type.flags(), vec![flag_type]);
        assert_eq!(form_type.added_flags(), vec![flag_type]);
    }
}
