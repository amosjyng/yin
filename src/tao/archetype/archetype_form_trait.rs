use super::Archetype;
use crate::node_wrappers::{BaseNodeTrait, FinalNode};
use crate::tao::archetype::{ArchetypeTrait, AttributeArchetype, IsArchetype};
use crate::tao::attribute::{HasAttributeType, Inherits};
use crate::tao::FormTrait;
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
///
/// Tests are in the structs that implement this trait.
pub trait ArchetypeFormTrait<'a>: ArchetypeTrait<'a> + FormTrait + IsArchetype {
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
            .add_outgoing(HasAttributeType::TYPE_ID, attribute_type.essence());
    }

    /// Retrieve non-inherited attribute types that are introduced by this archetype to all
    /// descendant archetypes. Attribute types introduced by an ancestor do not count.
    fn introduced_attribute_types(&self) -> Vec<AttributeArchetype> {
        self.essence()
            .base_wrapper()
            .outgoing_nodes(HasAttributeType::TYPE_ID)
            .into_iter()
            .map(FinalNode::from)
            .map(AttributeArchetype::from)
            .collect()
    }
}
