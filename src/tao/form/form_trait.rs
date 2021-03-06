use super::Form;
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode, InheritanceNodeTrait};
use crate::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use crate::tao::relation::attribute::{Inherits, MetaForm};
use crate::tao::relation::flag::IsIndividual;
use crate::tao::Tao;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

/// All forms are derived from archetypes. All forms, by their very existence, are capable of the
/// following interactions.
pub trait FormTrait: Deref<Target = FinalNode> + DerefMut + std::fmt::Debug + ArchetypeTrait {
    /// Jung called, and you answered. It is time to let go of your individuality and return to
    /// the Oneness from which you once came. There is no life or death, there is no existence or
    /// non-existence, there is no form or abstraction. Forget all preconceptions, blur all
    /// boundaries, be at peace with the universe again.
    fn ego_death(&self) -> Tao {
        Tao::from(self.id())
    }

    /// A less severe form of ego-death, where you still remember that you exist.
    fn as_form(&self) -> Form {
        Form::from(self.id())
    }

    /// Set a parent archetype. The current archetype will inherit all attributes of the parent
    /// archetype.
    fn add_parent(&mut self, parent: Archetype) {
        self.add_outgoing(Inherits::TYPE_ID, &parent);
    }

    /// Whether this represents an individual.
    fn is_individual(&self) -> bool {
        self.has_flag(IsIndividual::TYPE_ID)
    }

    /// Get all direct parent archetypes of this concept.
    fn parents(&self) -> Vec<Archetype> {
        let direct_parents: Vec<Archetype> = self
            .outgoing_nodes(Inherits::TYPE_ID)
            .into_iter()
            .filter(|p| p.id() == Tao::TYPE_ID || p != self.deref())
            .map(Archetype::from)
            .filter(|a| !a.is_individual())
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
            ancestry.push(Archetype::from(next_node.id()));
            next_node = *backpointers.get(&next_node).unwrap();
        }
        ancestry
    }

    /// Checks to see if another archetype is a direct parent of this one.
    fn has_parent(&self, possible_ancestor: Archetype) -> bool {
        self.outgoing_nodes(Inherits::TYPE_ID)
            .contains(&possible_ancestor)
    }

    /// Checks to see if another archetype is an ancestor of this one. If so, the current archetype
    /// will inherit all attributes of the ancestor.
    fn has_ancestor(&self, possible_ancestor: Archetype) -> bool {
        self.inheritance_nodes().contains(&possible_ancestor)
    }

    /// View the current node from its meta perspective.
    fn meta(&self) -> Self::ArchetypeForm {
        Self::ArchetypeForm::from(self.id())
    }

    /// Get the node representing the current node's meta-perspective.
    ///
    /// This is in contrast to `self.meta()`, which views the current node *from* the
    /// meta-perspective.
    fn meta_archetype(&self) -> Archetype {
        // same assumption as in attribute archetype form trait about ID and specificity
        Archetype::from(
            self.outgoing_nodes(MetaForm::TYPE_ID)
                .last()
                .unwrap_or(&FinalNode::from(Archetype::TYPE_ID))
                .id(),
        )
    }

    /// Grab the meta-perspective that's specific to the current type. If it doesn't exist yet,
    /// then it will be created.
    fn specific_meta(&mut self) -> Archetype {
        // there should only be one of these
        let uninherited_metas = self.base_wrapper().outgoing_nodes(MetaForm::TYPE_ID);
        match uninherited_metas.last() {
            Some(specific_meta) => Archetype::from(specific_meta.id()),
            None => {
                // grabbing parent metas first so that they get created first and the
                // greater-ID-greater-specificity assumption still holds
                let mut parent_metas = Vec::<Archetype>::new();
                for parent in self.parents().iter_mut() {
                    // calling specific_meta() here instead of meta_archetype(), so that in case
                    // the child meta is defined before the parent meta is, the child meta will
                    // still inherit from the parent
                    parent_metas.push(parent.specific_meta());
                }
                let mut new_meta = Archetype::archetype().individuate_as_archetype();
                for parent_meta in parent_metas {
                    new_meta.add_parent(parent_meta);
                }
                self.set_meta_archetype(&new_meta);
                new_meta
            }
        }
    }

    /// Whether or not this type has its own specific meta. If it doesn't, that means its meta
    /// object is inherited.
    fn has_specific_meta(&self) -> bool {
        !self
            .base_wrapper()
            .outgoing_nodes(MetaForm::TYPE_ID)
            .is_empty()
    }

    /// Set the meta-form for this Form.
    fn set_meta_archetype(&mut self, archetype: &Archetype) {
        self.add_outgoing(MetaForm::TYPE_ID, &archetype)
    }

    /// Mark this concept as representing an individual.
    fn mark_individual(&mut self) {
        self.add_flag(IsIndividual::TYPE_ID);
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
            vec![Owner::archetype().into(), Value::archetype().into()]
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
    fn test_tao_self_parenthood_not_ignored() {
        initialize_kb();
        assert_eq!(Tao::archetype().parents(), vec![Tao::archetype()]);
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
    fn test_individual_inheritance_ignored() {
        initialize_kb();
        let form1 = Tao::archetype().individuate_as_form();
        let mut form2 = Tao::archetype().individuate_as_form();
        // no high-level way to declare inheritance just yet
        form2.add_parent(Archetype::from(form1.id()));
        assert_eq!(form2.parents(), vec![Tao::archetype()]);
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
    fn test_form_meta_set() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let form_indv = form_type.individuate_as_form();
        let meta_type = Archetype::archetype().individuate_as_archetype();
        // also test default value here
        assert_eq!(form_indv.meta_archetype(), Archetype::archetype());

        form_type.set_meta_archetype(&meta_type);
        assert_eq!(form_indv.meta_archetype(), meta_type);
    }

    #[test]
    fn test_form_meta_inheritance() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let meta_type = Archetype::archetype().individuate_as_archetype();
        form_type.set_meta_archetype(&meta_type);
        assert_eq!(form_type.meta_archetype(), meta_type);

        let form_type2 = form_type.individuate_as_archetype();
        let form_type3 = form_type2.individuate_as_archetype();
        assert_eq!(form_type3.meta_archetype(), meta_type);
        assert!(!form_type3.has_specific_meta());
    }

    #[test]
    fn test_form_meta_inheritance_override() {
        initialize_kb();
        let mut form_type = Form::archetype().individuate_as_archetype();
        let meta_type = Archetype::archetype().individuate_as_archetype();
        form_type.set_meta_archetype(&meta_type);
        let form_type2 = form_type.individuate_as_archetype();
        let mut form_type3 = form_type2.individuate_as_archetype();
        let meta_type3 = form_type3.specific_meta();
        assert_eq!(form_type3.meta_archetype(), meta_type3);
        assert!(meta_type3.has_ancestor(meta_type));
        assert!(form_type3.has_specific_meta());
    }

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
