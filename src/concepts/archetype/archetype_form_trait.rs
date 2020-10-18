use super::Archetype;
use crate::concepts::archetype::attribute::AttributeArchetype;
use crate::concepts::attributes::{HasAttributeType, Inherits};
use crate::concepts::{ArchetypeTrait, FormTrait};
use crate::node_wrappers::{BaseNodeTrait, FinalNode};
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
///  * `A` type parameter: represents the ArchetypeForm that will reason about the node as a
///     specialized `Archetype`. Should be the same as the `Archetype` that this is currently being
///     implemented on.
///  * `F` type parameter: represents the direct Form that will reason about the node's leaves --
///    i.e. the individuals of this node. This will be the most specific FormTrait that is still
///    general enough to represent everything of this type.
///
/// Keep in mind that due to the melding of archetype and form in all Archetype structs, any   
/// references to `self` here refers to the node-as-archetype in question, whereas any references
/// to `Self` refers to the Archetype node itself. Since this FormTrait is supposed to reason about
/// the node-as-archetype, **there should be no instances of `Self` here**.
///
/// Tests are in the structs that implement this trait.
pub trait ArchetypeFormTrait<
    'a,
    A: ArchetypeTrait<'a, A> + FormTrait + From<FinalNode>,
    F: ArchetypeTrait<'a, F> + FormTrait + From<FinalNode>,
>: FormTrait + From<FinalNode> + From<usize>
{
    /// Forget everything about the current form, except that it's an ArchetypeForm representing
    /// some type.
    fn as_archetype(&self) -> Archetype {
        Archetype::from(*self.essence())
    }

    /// Create a subtype of the archetype represented by this Archetype instance.
    ///
    /// Convenience function for the static one.
    fn individuate_as_archetype(&self) -> A {
        A::individuate_with_parent(self.id())
    }

    /// Create a new individual of the archetype represented by this Archetype instance.
    ///
    /// Convenience function for the static one.
    fn individuate_as_form(&self) -> F {
        F::individuate_with_parent(self.id())
    }

    /// Individuals that adhere to this archetype. It is possible that some of these individuals
    /// might not be direct descendants of the archetype in question.
    fn individuals(&self) -> Vec<F> {
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
        let mut result: Vec<F> = leaves
            .into_iter()
            .filter(|l| l != self.essence()) // never return self, even if it's the only leaf
            .map(F::from)
            .collect();
        result.sort();
        result
    }

    /// Retrieve child archetypes.
    fn child_archetypes(&self) -> Vec<A> {
        self.essence()
            .incoming_nodes(Inherits::TYPE_ID)
            .into_iter()
            .map(A::from)
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
