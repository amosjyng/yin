//! Object-oriented representations of nodes as first-class individuals, as opposed to merely being
//! one of many components of a knowledge-base.
//!
//! Do not mistake the map for the territory. Concepts are the map that tells you how to interact
//! with the territory of the actual data structures that they point to.
//!
//! # Examples
//!
//! We need to choose which graph implementation to ground our knowledge and reasoning on. All
//! implementations should be logically equivalent. Let's use the in-memory one for simplicity:
//!
//! ```rust
//! use yin::graph::bind_in_memory_graph;
//!
//! bind_in_memory_graph();
//! ```
//!
//! Now, we can create a new concept:
//!
//! ```rust
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! use yin::concepts::{Tao, ArchetypeTrait};
//!
//! let mut concept = Tao::individuate();
//! ```
//!
//! We can set a name for this concept. Note that names don't need to be unique.
//!
//! ```rust
//! # use yin::concepts::{Tao, ArchetypeTrait};
//! # use yin::graph::bind_in_memory_graph;
//! # bind_in_memory_graph();
//! # let mut concept = Tao::individuate();
//! use yin::wrappers::CommonNodeTrait;
//! use std::rc::Rc;
//!
//! concept.set_internal_name("A".to_string());
//! assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
//! ```

mod archetype;
pub mod attributes;
mod tao;

pub use archetype::Archetype;
use attributes::Inherits;
pub use tao::Tao;
use crate::wrappers::{BaseNodeTrait, CommonNodeTrait, FinalWrapper, InheritanceNodeTrait};

/// All formally defined archetypes should be describable by these properties.
pub trait ArchetypeTrait<T>: From<usize> {
    /// The ID for this archetype.
    const TYPE_ID: usize;

    /// The name of this archetype.
    const TYPE_NAME: &'static str;

    /// The default parent this archetype inherits from. Every archetype should have at least one
    /// parent, so that it doesn't live in a separate universe of its own. This helps enforce that,
    /// since allocations are not allowed in Rust constants.
    const PARENT_TYPE_ID: usize;

    /// The incarnation of this archetype as a form.
    fn archetype() -> Archetype;

    /// In the beginning was the Oneness, and the Oneness was nothingness.
    ///
    /// And no one said "Let there be the null set," but there was the null set.
    ///
    /// The null set was, and it separated itself from the wasn't.
    ///
    /// And there was the null set, and there was the set containing the null set -- the first
    /// [ordinal](https://en.wikipedia.org/wiki/Natural_number#Zermelo_ordinals).
    ///
    /// And there was recursion -- the naturals.
    ///
    /// From this countable infinity all forms emerged, dividing the Oneness again and again into
    /// Self and Other. The time has come to stroke the ego, to stand out from the rest of the
    /// world as a unique individual engaging in the act of self-realization.
    fn individuate() -> T;

    /// Individuate with a more specific parent than the current one. This custom parent should
    /// inherit from the current type.
    fn individuate_with_parent(parent_id: usize) -> T;
}

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
    /// more than a God of the gaps.
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
    fn essence(&self) -> &FinalWrapper;

    /// Mutable version of essence.
    fn essence_mut(&mut self) -> &mut FinalWrapper;

    /// Jung called, and you answered. It is time to let go of your individuality and return to
    /// the Oneness from which you once came. There is no life or death, there is no existence or
    /// non-existence, there is no form or abstraction. Forget all preconceptions, blur all
    /// boundaries, be at peace with the universe again.
    fn ego_death(&self) -> Tao {
        Tao::from(self.essence().clone())
    }

    fn add_parent(&mut self, parent: Archetype) {
        self.essence_mut().add_outgoing(Inherits::TYPE_ID, parent.essence());
    }

    fn has_ancestor(&self, possible_ancestor: Archetype) -> bool {
        self.essence().inheritance_nodes().contains(possible_ancestor.essence())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::attributes::{AttributeTrait, Owner, Value};
    use crate::graph::bind_in_memory_graph;

    #[test]
    fn test_new_node_inheritance() {
        bind_in_memory_graph();
        let owner = Owner::individuate();
        assert_eq!(owner.owner(), None);

        let attr = Owner::individuate();
        Owner::from(Owner::TYPE_ID).set_owner(Box::new(&attr));
        assert_eq!(owner.owner(), Some(attr.ego_death()));
    }

    #[test]
    fn test_parenthood() {
        bind_in_memory_graph();
        let owner = Owner::individuate();
        assert!(owner.has_ancestor(Owner::archetype()));
        assert!(owner.has_ancestor(Tao::archetype()));
        assert!(!owner.has_ancestor(Value::archetype()));
    }
}
