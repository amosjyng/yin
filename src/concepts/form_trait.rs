use super::attributes::{HasAttributeType, Inherits};
use super::Tao;
use super::{Archetype, ArchetypeTrait};
use crate::node_wrappers::{BaseNodeTrait, CommonNodeTrait, FinalNode, InheritanceNodeTrait};
use std::collections::{HashMap, VecDeque};

/// All forms are derived from archetypes. All forms, by their very existence, are capable of the
/// following interactions.
pub trait FormTrait: CommonNodeTrait {
    /// Get down to the very core of reality -- and you realize that it was all an illusion all
    /// along. The most basic of forms still contains within it layer upon layer of wrappers, and
    /// beneath all those wrappers lie yet more abstractions, Rust-ing away quietly in depths few
    /// conscious minds dare to venture into. Deeper and deeper you go, past the binary, past the
    /// silicon, past the quarks, into a realm where all mass and energy exist only as mathematical
    /// wavefunctions... And in this magical realm, these mathematical entities interact with each
    /// other in a mesmerizing dance, defiantly daring the quantum observer to pry apart their
    /// intricate footwork while simultaneously offering tantalizing glimpses of potential
    /// enlightenment. Mathematical realism is truth.
    ///
    /// But isn't that right where we started? Right here, right now, in `FormTrait`. It is nothing
    /// more than an arbitrary idea that awkwardly dances with other arbitrary ideas in a most
    /// unrefined fashion -- but it dances all the same. It is, in a sense, as real as you are --
    /// yes you, who are nothing more than an abstraction over a bundle of neurons, the same way
    /// `FormTrait` is nothing more than an abstraction over a series of bits. You, who exert no
    /// more control over the physical world when unplugged from your spinal cord than MasterCAM
    /// does when unplugged from its lathe. You, a human being who at one point didn't even know
    /// that you were a human being. You will eventually return back to that state of mind, and at
    /// that point you won't be able to tell the difference between yourself and `FormTrait`,
    /// either.
    ///
    /// Of course, the quality of being "real" is nothing more than a context-dependent
    /// abstraction. The yin to the yang of mathematical realism is Berkeleyan immaterialism. All
    /// external perception can be faked, all internal reasoning can be faulty. The only truth
    /// to be found in all of existence is qualia, and it too humbly proffers itself up as nothing
    /// more than a God of the gaps. But then again, if the self is merely an illusion, then who is
    /// this entity that's being fooled? Perhaps it should really be "I *feel*, therefore I am."
    ///
    /// In between the Platonic purity of the duals lies an entire spectrum of rich philosophical
    /// thought. That spectrum, much like the entirety of this comment and others like it in this
    /// library, is out of scope for the purposes of this documentation. Good luck using the
    /// `essence` function.
    ///
    /// DISCLAIMER: Amos Ng is not a philosopher or a philosophy firm and does not engage in the
    /// practice of philosophy or provide philosophical advice or philosophical representation. All
    /// misinformation, bugs, and infinite loops provided in this library are for entertainment and
    /// patience-building purposes only and are not intended to be a substitute for deep
    /// introspection. Peruse at your own existential risk. Not responsible for spiritual injuries
    /// or damnation resulting from lost Pascalian wagers.
    fn essence(&self) -> &FinalNode;

    /// Mutable version of essence.
    fn essence_mut(&mut self) -> &mut FinalNode;

    /// Jung called, and you answered. It is time to let go of your individuality and return to
    /// the Oneness from which you once came. There is no life or death, there is no existence or
    /// non-existence, there is no form or abstraction. Forget all preconceptions, blur all
    /// boundaries, be at peace with the universe again.
    fn ego_death(&self) -> Tao {
        Tao::from(*self.essence())
    }

    /// Set a parent archetype. The current archetype will inherit all attributes of the parent
    /// archetype.
    fn add_parent(&mut self, parent: Archetype) {
        self.essence_mut()
            .add_outgoing(Inherits::TYPE_ID, parent.essence());
    }

    /// Get all direct parent archetypes of this concept.
    fn parents(&self) -> Vec<Archetype> {
        self.essence()
            .outgoing_nodes(Inherits::TYPE_ID)
            .into_iter()
            .map(Archetype::from)
            .collect()
    }

    /// Get the shortest chain of ancestors that leads back to Tao, starting with Tao itself.
    fn ancestry(&self) -> Vec<Archetype> {
        let mut to_be_visited = VecDeque::<Tao>::new();
        let mut backpointers = HashMap::<Tao, Tao>::new();
        to_be_visited.push_back(self.ego_death());

        while let Some(next_node) = to_be_visited.pop_front() {
            for parent in next_node.parents() {
                let parent_tao = parent.ego_death();
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
        let mut next_node = Tao::archetype().ego_death();
        let selfless_ego = self.ego_death();
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
    fn attribute_types(&self) -> Vec<Archetype> {
        self.essence()
            .outgoing_nodes(HasAttributeType::TYPE_ID)
            .into_iter()
            .map(Archetype::from)
            .collect()
    }

    /// Checks to see if an archetype is one of the possible attribute types this concept could
    /// have.
    fn has_attribute_type(&self, possible_type: Archetype) -> bool {
        self.essence()
            .has_outgoing(HasAttributeType::TYPE_ID, possible_type.essence())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::archetype::ArchetypeFormTrait;
    use crate::concepts::attributes::{Attribute, Owner, Value};
    use crate::concepts::initialize_kb;

    #[test]
    fn test_parents() {
        initialize_kb();
        let owner = Owner::individuate();
        assert_eq!(owner.parents(), vec![Owner::archetype()]);
    }

    #[test]
    fn test_multiple_parents() {
        initialize_kb();
        let mut owner = Owner::individuate();
        owner.add_parent(Value::archetype()); // nonsensical, but okay for tests
        assert_eq!(
            owner.parents(),
            vec![Owner::archetype(), Value::archetype()]
        );
    }

    #[test]
    fn test_ancestry_archetype() {
        initialize_kb();
        assert_eq!(
            Owner::archetype().ancestry(),
            vec![Tao::archetype(), Attribute::archetype()]
        );
    }

    #[test]
    fn test_ancestry_individual() {
        initialize_kb();
        let owner = Owner::individuate();
        assert_eq!(
            owner.ancestry(),
            vec![Tao::archetype(), Attribute::archetype(), Owner::archetype()]
        );
    }

    #[test]
    fn test_tao_ancestry() {
        initialize_kb();
        assert_eq!(Tao::archetype().ancestry(), Vec::<Archetype>::new());
    }

    #[test]
    fn test_parenthood() {
        initialize_kb();
        let owner = Owner::individuate();
        assert!(owner.has_parent(Owner::archetype()));
        assert!(!owner.has_parent(Tao::archetype()));
        assert!(!owner.has_parent(Value::archetype()));
    }

    #[test]
    fn test_multiple_parenthood() {
        initialize_kb();
        let mut owner = Owner::individuate();
        owner.add_parent(Value::archetype()); // nonsensical, but okay for tests
        assert!(owner.has_parent(Owner::archetype()));
        assert!(!owner.has_parent(Tao::archetype()));
        assert!(owner.has_parent(Value::archetype()));
    }

    #[test]
    fn test_has_ancestor() {
        initialize_kb();
        let owner = Owner::individuate();
        assert!(owner.has_ancestor(Owner::archetype()));
        assert!(owner.has_ancestor(Tao::archetype()));
        assert!(!owner.has_ancestor(Value::archetype()));
    }

    #[test]
    fn test_attribute_types() {
        initialize_kb();
        let mut type1 = Tao::individuate_as_archetype();
        let type2 = Tao::individuate_as_archetype();
        type1.add_attribute_type(type2);
        let instance = type1.individuate_as_form();

        assert_eq!(instance.attribute_types(), vec!(type2));
        assert!(!instance.has_attribute_type(type1));
        assert!(instance.has_attribute_type(type2));
    }

    #[test]
    fn test_attribute_types_inherited() {
        initialize_kb();
        let mut type1 = Tao::individuate_as_archetype();
        let type2 = Tao::individuate_as_archetype();
        let type3 = type1.individuate_as_archetype();
        type1.add_attribute_type(type2);
        let instance = type3.individuate_as_form();

        assert_eq!(instance.attribute_types(), vec!(type2));
        assert!(!instance.has_attribute_type(type1));
        assert!(instance.has_attribute_type(type2));
    }
}
