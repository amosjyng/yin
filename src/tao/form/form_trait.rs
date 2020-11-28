use super::Form;
use crate::node_wrappers::{BaseNodeTrait, FinalNode, InheritanceNodeTrait};
use crate::tao::archetype::{Archetype, ArchetypeTrait, AttributeArchetype};
use crate::tao::relation::attribute::has_property::HasAttribute;
use crate::tao::relation::attribute::Inherits;
use crate::tao::Tao;
use crate::Wrapper;
use std::collections::{HashMap, VecDeque};

/// All forms are derived from archetypes. All forms, by their very existence, are capable of the
/// following interactions.
pub trait FormTrait: Wrapper<BaseType = FinalNode> {
    /// Jung called, and you answered. It is time to let go of your individuality and return to
    /// the Oneness from which you once came. There is no life or death, there is no existence or
    /// non-existence, there is no form or abstraction. Forget all preconceptions, blur all
    /// boundaries, be at peace with the universe again.
    fn ego_death(&self) -> Tao {
        Tao::from(*self.essence())
    }

    /// A less severe form of ego-death, where you still remember that you exist.
    fn as_form(&self) -> Form {
        Form::from(*self.essence())
    }

    /// Set a parent archetype. The current archetype will inherit all attributes of the parent
    /// archetype.
    fn add_parent(&mut self, parent: Archetype) {
        self.essence_mut()
            .add_outgoing(Inherits::TYPE_ID, parent.essence());
    }

    /// Get all direct parent archetypes of this concept.
    fn parents(&self) -> Vec<Archetype> {
        let direct_parents: Vec<Archetype> = self
            .essence()
            .outgoing_nodes(Inherits::TYPE_ID)
            .into_iter()
            .filter(|p| p != self.essence())
            .map(Archetype::from)
            .collect();
        let mut specific_parents = Vec::<Archetype>::new();
        for parent in direct_parents {
            if specific_parents.iter().any(|sp| sp.has_ancestor(parent)) {
                continue; // this is not the most specific parent
            }
            specific_parents.retain(|sp| !parent.has_ancestor(*sp));
            specific_parents.push(parent);
        }
        specific_parents
    }

    /// Get the shortest chain of ancestors that leads back to Tao, starting with Tao itself.
    fn ancestry(&self) -> Vec<Archetype> {
        let mut to_be_visited = VecDeque::<Form>::new();
        let mut backpointers = HashMap::<Form, Form>::new();
        to_be_visited.push_back(self.as_form());

        while let Some(next_node) = to_be_visited.pop_front() {
            for parent in next_node.parents() {
                let parent_tao = parent.as_form();
                #[allow(clippy::map_entry)]
                if !backpointers.contains_key(&parent_tao) {
                    backpointers.insert(parent_tao, next_node);
                    to_be_visited.push_back(parent_tao);
                    if parent == Tao::archetype() {
                        break;
                    }
                }
            }
        }

        let mut ancestry = Vec::new();
        let mut next_node = Tao::archetype().as_form();
        let selfless_ego = self.as_form();
        while next_node != selfless_ego {
            ancestry.push(Archetype::from(*next_node.essence()));
            next_node = *backpointers.get(&next_node).unwrap();
        }
        ancestry
    }

    /// Checks to see if another archetype is a direct parent of this one.
    fn has_parent(&self, possible_ancestor: Archetype) -> bool {
        self.essence()
            .outgoing_nodes(Inherits::TYPE_ID)
            .contains(possible_ancestor.essence())
    }

    /// Checks to see if another archetype is an ancestor of this one. If so, the current archetype
    /// will inherit all attributes of the ancestor.
    fn has_ancestor(&self, possible_ancestor: Archetype) -> bool {
        self.essence()
            .inheritance_nodes()
            .contains(possible_ancestor.essence())
    }

    /// Get all the types of attributes that this concept is predefined to potentially have.
    fn attribute_archetypes(&self) -> Vec<AttributeArchetype> {
        self.essence()
            .outgoing_nodes(HasAttribute::TYPE_ID)
            .into_iter()
            .map(AttributeArchetype::from)
            .collect()
    }

    /// Checks to see if an archetype is one of the possible attribute types this concept could
    /// have.
    fn has_attribute_type(&self, possible_type: AttributeArchetype) -> bool {
        self.essence()
            .has_outgoing(HasAttribute::TYPE_ID, possible_type.essence())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::archetype::{Archetype, ArchetypeFormTrait};
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{Attribute, Owner, Value};

    #[test]
    fn test_parents() {
        initialize_kb();
        let owner = Owner::new();
        assert_eq!(owner.parents(), vec![Owner::archetype().into()]);
    }

    #[test]
    fn test_multiple_parents() {
        initialize_kb();
        let mut owner = Owner::new();
        owner.add_parent(Value::archetype().into()); // nonsensical, but okay for tests
        assert_eq!(
            owner.parents(),
            vec![
                Owner::archetype().into(),
                Value::archetype().into()
            ]
        );
    }

    #[test]
    fn test_ancestry_archetype() {
        initialize_kb();
        let type1 = Tao::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        assert_eq!(type2.ancestry(), vec![Tao::archetype(), type1]);
    }

    #[test]
    fn test_ancestry_individual() {
        initialize_kb();
        let type1 = Tao::archetype().individuate_as_archetype();
        let type2 = type1.individuate_as_archetype();
        let form = type2.individuate_as_form();
        assert_eq!(form.ancestry(), vec![Tao::archetype(), type1, type2]);
    }

    #[test]
    fn test_tao_ancestry() {
        initialize_kb();
        assert_eq!(Tao::archetype().ancestry(), Vec::<Archetype>::new());
    }

    #[test]
    fn test_looped_ancestry() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        type1.add_parent(type1);
        assert_eq!(type1.ancestry(), vec![Tao::archetype()]);
    }

    #[test]
    fn test_looped_child_ancestry() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        type1.add_parent(type1);
        let type2 = type1.individuate_as_archetype();
        assert_eq!(type2.ancestry(), vec![Tao::archetype(), type1]);
    }

    #[test]
    fn test_parenthood() {
        initialize_kb();
        let owner = Owner::new();
        assert!(owner.has_parent(Owner::archetype().into()));
        assert!(!owner.has_parent(Tao::archetype()));
        assert!(!owner.has_parent(Value::archetype().into()));
    }

    #[test]
    fn test_multiple_parenthood() {
        initialize_kb();
        let mut owner = Owner::new();
        owner.add_parent(Value::archetype().into()); // nonsensical, but okay for tests
        assert!(owner.has_parent(Owner::archetype().into()));
        assert!(!owner.has_parent(Tao::archetype()));
        assert!(owner.has_parent(Value::archetype().into()));
    }

    #[test]
    fn test_self_parenthood_ignored() {
        initialize_kb();
        let mut new_type = Tao::archetype().individuate_as_archetype();
        new_type.add_parent(new_type);
        assert_eq!(new_type.parents(), vec![Tao::archetype()]);
    }

    #[test]
    fn specific_to_generic_parenthood() {
        initialize_kb();
        let mut form = Form::new();
        form.add_parent(Tao::archetype());
        assert_eq!(form.parents(), vec![Form::archetype()]);
    }

    #[test]
    fn generic_to_specific_parenthood() {
        initialize_kb();
        let mut form = Tao::new();
        form.add_parent(Form::archetype());
        assert_eq!(form.parents(), vec![Form::archetype()]);
    }

    #[test]
    fn test_has_ancestor() {
        initialize_kb();
        let owner = Owner::new();
        assert!(owner.has_ancestor(Owner::archetype().into()));
        assert!(owner.has_ancestor(Tao::archetype()));
        assert!(!owner.has_ancestor(Value::archetype().into()));
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Attribute::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        type1.add_attribute_type(type2);
        let instance = type1.individuate_as_form();

        assert_eq!(
            instance.attribute_archetypes(),
            vec![Owner::archetype(), Value::archetype(), type2]
        );
        assert!(!instance.has_attribute_type(type1));
        assert!(instance.has_attribute_type(type2));
    }

    #[test]
    fn test_attribute_types_inherited() {
        initialize_kb();
        let mut type1 = Attribute::archetype().individuate_as_archetype();
        let type2 = Attribute::archetype().individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);
        let instance = type3.individuate_as_form();

        assert_eq!(
            instance.attribute_archetypes(),
            vec![Owner::archetype(), Value::archetype(), type2]
        );
        assert!(!instance.has_attribute_type(type1));
        assert!(instance.has_attribute_type(type2));
    }
}
