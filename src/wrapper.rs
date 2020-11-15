/// Generic wrapper trait.
pub trait Wrapper {
    /// The type that the implementing type is a wrapper for.
    ///
    /// In other words, if `B` is a wrapper around `A`, then `B` would be the `BaseType`.
    type BaseType;

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
    fn essence(&self) -> &Self::BaseType;

    /// Mutable version of essence.
    fn essence_mut(&mut self) -> &mut Self::BaseType;
}
