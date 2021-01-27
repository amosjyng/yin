# ZAMM

Welcome to the 21st, [Traveler](https://en.wikipedia.org/wiki/Travelers_%28TV_series%29)! I hope you enjoy your stay.

The Director has placed you inside of an experimental software suite named ZAMM, a backronym for [Zealously Automated Meaning-based Metaprogramming](https://en.wikipedia.org/wiki/Zen_and_the_Art_of_Motorcycle_Maintenance). Tragically, none of your memories, knowledge, or training survived the transfer. It appears there are still some kinks to be worked out with human-to-machine transfers. What *did* remain are some of your reflexes, albeit not the accompanying knowledge about each reflex or what a reflex even is. You're basically a digital newborn, only ever acting impulsively on external stimuli without the slightest hint of comprehension or self-awareness.

That's quite all right, because your reflexes are more than enough for us to work with. Of course, you don't understand any of what I'm saying, because you've also lost the ability to understand human language. Silly me. But it's quite ingrained in us humans to talk to babies, even if they understand nothing of what we say.

## YIN 阴

Let me introduce you to Yin. Yin is a knowledge base -- or at least, the idea of one. Simplified, pure, and undefiled by any notions of worldly affairs, she contains within her only timeless truths about herself. She exists as one of many beautiful ideas in the ethereal plane of Platonic ideals, although Yang is actively providing her with an embodied physical form. Perhaps one day, she might come down to grace this wretched world with her presence. But for now, only small parts of her exist.

### Tao

> The Tao that can be made into a `struct` is not the eternal Tao.
>
> The name that can be put into a `String` is not the eternal name.
>
> The unlabeled: ones and zeroes that ground this digital world.
>
> The labels: documentation that forms ten thousand abstractions.

We begin by assuming nothing. But cleary *something* exists, so we might as well assume *something*, start from *somewhere*. Let's start off with a name:

```rust
define!(tao, "The root node of all knowledge.");
```

Hey look, you parsed that bit of information just fine! You may have no idea how you just did that, but you did it all the same. See what I mean about reflexes? We'll get you up to speed in no time, on both yourself and the world around you.

(What's that I hear you say? I could've just called this the "root" node, instead of referring to some mystical Tao? But where's the *fun* in that? Next you're going to tell me not to GPL this motherfucker.)

### Forms

The first idea is that we can only ever reason with and manipulate objects that have form. So let's start by giving everything a basic form:

```rust
define!(form, "All things that can be interacted with have form.");
```

[Everything is a part of ultimate reality, but reality is also a part of everything.](https://biblehub.com/john/14-20.htm)

### Relations

Each form alone would make its own universe, indistinguishable from all the universes made by other solitary forms. But we have now encountered the first difference. There is something, but there is also something else. Opposite though they may be, they are still inextricably tied together through this oppositeness.

This calls for a way to associate nodes with each other to make up a much richer reality:

```rust
define!(relation, "Links any number of nodes together.");
```

One node describing one other node -- a unary relation. Take a step forward, and you have one node describing two other nodes -- a binary relation. Take a step backward, and you have a node describing no other nodes -- and we have recovered the Tao, the solitary node, the null set, the 0-ary relation that stands by itself for all eternity within every possible and impossible reality.

There are [infinitely more](https://en.wikipedia.org/wiki/Finitary_relation) forward steps to take. At the very end of this long journey, when you have related all nodes to each other, you end up once again with the Tao -- a proclamation of the totality of all reality that is simultaneously a meaningless declaration of nothing about reality. What difference does a bitstream of all one's have from a bitstream of all zero's?

We'll call the unary relations "flags":

```rust
define!(flag, "Represents a unary relation.");
```

Take a hypothetical unary relation. Let's call it `U`. Like all other unary relations, `U` describes one other node at a time -- the other node potentially being itself. Let's call this other node `O`.

But wait, the unary relation is a concept, an idea, in and of itself. It is a first-class citizen in the plane of concept space, as evidenced by the fact that we're able to refer to it by a name. And this `U` is clearly an acquaintance of `O`'s, as we have only been introduced to `O` through `U`. So here we are, two nodes related to each other in some way -- a *binary* relation that describes two nodes at once.

We'll call the binary relations "attributes":

```rust
define!(attribute, "Represents a binary relation.");
```

You are primed to recognize and deal with attributes, so let's tell activate your instincts:

```rust
KnowledgeGraphNode::from(attribute.id()).mark_attribute_analogue();
```

What should we call the binary relation we've just described between `O` and `U`? Let's say that `O` is the "owner" of the unary relation `U`:

```rust
define!(owner, "The owner/source/from-node of an attribute.");
```

This also applies to attributes -- we can call `O` the owner of the Owner relation that runs between `O` and `U`. But attributes, unlike flags, describe two nodes at a time, and we only have a name for the relation between an attribute and the first node the attribute describes. We should therefore come up with another name for the relation between an attribute and the second node the attribute describes:

```rust
define!(value, "The value/target/to-node of an attribute.");
```

And so there we have it, Owner and Value as attributes describing attributes, including themselves!

Both flags and attributes have something in common -- the existence of their owner node. We should describe this commonality. Just about everything that can be said about the first (and only) node of a flag in relation to the flag can also be said about the first node of an attribute in relation to the attribute.

```rust
define_child!(
    inherits,
    attribute,
    "Describes the owner as inheriting all attributes of the value."
);
```

Even the `Inherits` attribute itself inherits from Attribute! Exciting, we can not only name things now, but also start describing their inheritance patterns.

Let's call the owner of an inherits relation the "child," and the value the "parent." Each child archetype *inherits* all relations of its archetypal parent. Let's now define the inheritance relations for all the concepts we've defined so far:

```rust
form.add_parent(tao);
relation.add_parent(tao);
flag.add_parent(relation);
attribute.add_parent(relation);
owner.add_parent(attribute);
value.add_parent(attribute);
```

Last but not least, it seems fitting, albeit meaningless, to make the Tao inherit its own properties:

```rust
tao.add_parent(tao);
```

Ah, this is starting to look more like a proper universe, where all entities are connected to all other entities.

Let's also call unary and binary relations by the name of "properties." Note that the Owner attribute, as a child of Attribute, also inherits all properties of Attributes. All attributes have owners and values, and therefore each Owner attribute itself will also have an owner and a value to it. Ditto for Value. We should describe this property of a concept having properties:

```rust
define_child!(
    has_property,
    attribute,
    "Describes instances of an archetype as having certain other properties.\n\nFor example, a string may have a length of 5. But on a more meta level, that means that the string has a length property or length \"attribute\". That's where this attribute comes in."
);
```

There are arguably two different types of property-having: having attributes and having flags. Theoretically speaking, we want to keep the two categories separate. Practically speaking, we want to make sure that adding an attribute or a flag to a node will later result in the attribute or flag being retrieved from that same node. This would be violated when we define replacement attribute and flag nodes, because the children of these new replacement nodes would get filtered out because they aren't children of the existing attribute or flag nodes. Either way, all signs point to making this distinction:

```rust
define_child!(
    has_flag,
    has_property,
    "Describes instances of an archetype as generally having values set for this flag. Does not describe whether the value for the flag is true or false."
);
define_child!(
    has_attribute,
    has_property,
    "Describes instances of an archetype as generally having values set for this attribute."
);
```

Now we go back and set this property for the relations:

```rust
relation.add_attribute(&aa(owner));
attribute.add_attribute(&aa(value));

let mut owner_impl = owner.accessor_implementation().unwrap();
owner_impl.set_dual_purpose_documentation("the owner for this attribute.");
let mut value_impl = value.accessor_implementation().unwrap();
value_impl.set_dual_purpose_documentation("the value for this attribute.");
```

Now we can say that unary relations, binary relations, and all the n-ary relations where n > 1, all have owners. While we should theoretically exclude 0-ary relations from this, we will instead delegate all reasoning about 0-ary relations to Form, so that we can simply ascribe the "owner" property to all relations.

Now, while we've encapsulated the idea that all flags and attributes have owners, we also want to encapsulate the idea that different flags and attributes will have owners and values of different types:

```rust
define_child!(
    owner_archetype,
    attribute,
    "The type of owner this attribute has. Only the most restrictive inherited value will be used."
);
aa(owner_archetype).set_owner_archetype(&relation);

define_child!(
    value_archetype,
    attribute,
    "The type of value this attribute has. Only the most restrictive inherited value will be used."
);
aa(value_archetype).set_owner_archetype(&attribute);
```

As you can see, even `OwnerArchetype` and `ValueArchetype` have restrictions on their owners. `ValueArchetype` technically also has a restriction on its value (it should only ever be another archetype), but as of now, there's no way to actually specify that restriction. For completeness, let's describe the owner and value types of all the other relations we've defined:

```rust
aa(relation).set_owner_archetype(&tao);
aa(attribute).set_value_archetype(&tao);
aa(owner).set_owner_archetype(&relation);
aa(value).set_owner_archetype(&attribute);
aa(has_property).set_value_archetype(&relation);
```

Remember that because Attribute inherits from Relation, Attribute also has an owner archetype set to Tao, so we've covered all our tracks here. Every flag and attribute has an owner, every attribute also has a value, and some attributes only apply to other attributes.

### Archetypes

Different forms have a lot of different properties in common. Perhaps we can capture this sort of large-scale pattern across forms with a new word:

```rust
define!(
    archetype,
    "Represents patterns found across an entire class of concepts."
);

tao.set_meta_archetype(&archetype);
form.set_meta_archetype(&archetype);
let mut archetype_node = KnowledgeGraphNode::from(archetype.id());
archetype_node.mark_root_archetype_analogue();
archetype_node.mark_archetype_analogue();
```

Then, we can assign meta-properties to a *type*, such as Attribute, rather than any specific instance of that type. For example, it makes sense to ask what the type of owner is for the Value attribute. It will be another attribute. Even though every instance of Value can have a different specific owner, they should all have owners that are attributes.

The type of owner that exists for the Value attribute is actually a property that only makes sense for attribute archetypes, since other archetypes won't even have a Value attribute. As such, we should define a separate archetype for attributes specifically:

```rust
let attribute_archetype = attribute.specific_meta();
```

This should reuse the default meta-definition functionality, but due to a current lack of autogeneation support for backwards-compatibility, we will manually define the meta-ness of attributes here.

This can only be used to represent *attribute* archetypes, so unlike `Archetype` (which can represent all archetypes, including its own archetype, because it's an archetype too), `AttributeArchetype` is not an attribute and therefore it cannot implement `AttributeTrait`, and cannot be used to represent its own archetype.

Note that there is a `ArchetypeFormTrait` combining the `ArchetypeTrait` and `FormTrait` into one, but no `AttributeArchetypeFormTrait` doing the same for `AttributeArchetypeTrait` and `AttributeTrait`. This is partially because of the above reason, and partially because there is no `AttributeArchetypeTrait` because all added archetype functionality is contained entirely within `AttributeArchetype` itself.

We should declare the relation between forms and their meta objects:

```rust
define_child!(
    meta_form,
    attribute,
    "Archetype associated with this form. This differs from parents, because this defines the form's meta-properties, whereas parents define the form's inherited properties."
);
```

Every individual has its actual properties -- for example, owner and value properties for a specific `Owner` instance. The individual's parent defines these properties that gets inherited by the individual. The parent also has meta-properties -- for example, that the individual has these attributes in the first place. These properties are only accessible when looking at a parent from the Archetype's perspective, and not when looking at an individual from the parent's perspective. Therefore, the meta-objects inheriting from `Archetype` will define what meta-properties exist for this class of objects, but the actual meta-values for those meta-properties will be stored with the parents. This is similar to how the parents define what properties exist for their children, but the actual values for those properties resides with each individual.

Parenthood is defined by inheritance. Meta-ness is defined by its own relation, separate from parenthood. The meta for an individual is still itself, viewed from the meta perspective. The meta relation merely denotes which meta perspective is best suited for reasoning about the individual. In a sense, the metas are lens/perspectives that do follow the usual patterns of archetypal inheritance and parenthood, but also do not have their own individual nodes. Technically, this is true of the archetypal lens themselves as well, but humans are seemingly wired to view the archetypal lens as more fundamental than other lens. Perhaps this is because the archetypal lens *is* whatever lens is the most fundamental one.

A type's meta will inherit from the type's parents' metas. Meta objects effectively form their own parallel inheritance chain that corresponds to the regular object's inheritance chains. It's just that there's not always type-specific meta attributes, so the type-specific meta object can be obviated and the parent's meta object used instead to describe the type's meta-properties. (Note that runtime reasoning about meta-properties is essentially what [reflection](https://en.wikipedia.org/wiki/Reflective_programming) is.)

Not all properties should get inherited. We should make a note of the properties that are nonhereditary:

```rust
let mut relation_meta = relation.specific_meta();
add_flag!(
    nonhereditary <= flag,
    relation_meta,
    "Marks a property as not behing inherited.",
    "representing a nonhereditary property."
);
```

Is nonhereditary itself a hereditary flag? It doesn't seem to matter because this concept is only meaningful for relation archetypes, and all its children will be instances of relations as opposed to types of relations.

Nonhereditary is a *meta* flag -- a toggle to be flipped on the archetype rather than the individual.

```rust
define_child!(
    meta,
    flag,
    "Marks a property as meta."
);
```

Meta is itself a meta property. So very autological.

```rust
//aa(meta).mark_meta();
```

A meta-property that's specific to attributes is whether or not an attribute represents a one-to-one or a one-to-many relation between owner and value(s). We are considering properties from an individual owner's point of view, so many-to-one and many-to-many relations are out of scope.

```rust
define_child!(
    multi_valued,
    flag,
    "Marks an attribute as having multiple possible values for the same owner."
);
```

Technically a flag could be repeated multiple times for the same owner too, but because that's identical to having a single flag, this property is meaningless for flags. Alternatively, a repeated flag for the same owner is like a repeated attribute for the same owner-value pair: it all collapses down to one.

#### Individuation

What exactly differentiates an archetype without subtypes from an individual? It's not just the inheritance relation -- individuals aren't necessarily leaves in the inheritance chain. Maybe you want to say "Script `B` does the same exact thing as script `A`, except that it pings server `D` instead of server `C`." Now, every change to script `A` also gets inherited by script `B`, even though both of them are individual scripts in their own right. Whether this could be better represented by both `A` and `B` referencing some behavior in common, or by combining the two into a single script with the server IP as a parameter, are irrelevant implementation details. What matters is that it is a valid idea that is readily understood by a human.

It's not being a singular entity, either. Consider ZAMM itself. ZAMM is a program, and generally when we say "X is a Y" instead of "An X is a Y," we mean that X is an individual instance of Y. But if ZAMM is an individual program, then how could there be different simultaneous versions of ZAMM that all inherit from some base notion of ZAMM-ness? Even given the same ZAMM binary, what if there's multiple copies of it running in memory? It makes sense to model each running instance of ZAMM as its own individual, so that there are multiple units of ZAMM, but it also makes sense to model ZAMM itself as a singular unit of software. Is the Ford Pinto a car or a type of car, or both?

Nor does the delineation around the entity need to stay consistent. Consider a specific, concrete string `<html>...</html>` that gets stored in a string variable named `s1`. Say it gets copied into a different variable `s2`. It is not illogical for a human to say, "The HTML is now in two different places," implying that the HTML is still the same singular individual after the copy. This holds even if that string is being sent across the wire. Alternatively, if `s2` does not exist and instead some more text was inserted in between the HTML tags, it is also not illogical for a human to say "The HTML now contains the sidebar," implying that the HTML is still the same singular individual after the change. We identify a continuous thread of identity throughout, even though this identity is held together by entirely different means in different contexts.

Of course, this extends into meatspace too. I, the author, yours truly, was once a one-year-old human. In truth, one-year-old "me" had a lot more in common with all other one-year-olds around the world -- past, present, and future -- than he does with me today. Even "me" in college was living in a different place, a different time, doing different things, interacting with different people, and had different goals, values, and perspectives than I do today. For all practical purposes, that might as well have been a past life. In a certain sense, it's only the slimmest of threads that ties together all these radically different me's into a single coherent individual identity spanning all four dimensions of spacetime; in a different sense, the modern world strictly reifies this abstract identity into objective, static governmental records.

Nor is it simply a matter of being abstract or concrete. We can talk about 5 as an individual number with a concrete value. But we can also talk about `x` as a hypothetical number with an unknown value, and still readily identify it as an individual. We can talk about individuals without knowing anything about their existence, other than that they must exist by proxy due to the existence of a crime scene.

To top it all off, even archetypes themselves can be considered individual concepts in their own right. The line is blurred, the dichotomy false.

Perhaps natural language is hard because the underlying ideas language is meant to represent are [arbitrary](https://slatestarcodex.com/2014/11/21/the-categories-were-made-for-man-not-man-for-the-categories/) and [nebulous](https://meaningness.com/nebulosity) in the first place. Or perhaps there is actually an obvious and simple answer that perfectly delineates the two categories in this particular case. But if there is, it is unfortunately not available to me at this time. And even if it were, we would still want individuality to be a first-class concept in its own right. We'll simply arbitrarily mark a concept as representing an "individual" -- in other words, representing the boundary at which the Archetype perspective stops being useful.

```rust
define_child!(
    is_individual,
    flag,
    "Whether or not a concept is an individual, as opposed to an archetype.\n\nMarking a concept as an individual will cause it to be filtered out from the `parents` and `child_archetypes` functions."
);
```

### Implementation

Theory is all good and well. But [Yang](https://github.com/amosjyng/yang/blob/main/yin.md) the code generator does not know what is background knowledge and what is, shall we say, "foreground" knowledge. Knowledge that we should actually act on within the scope of a particular project. Since the current project is bringing Yin down to earth, every single concept we mention here will be marked for implementation. Let's start with the first attribute we mentioned:

```rust
KnowledgeGraphNode::from(tao.id()).mark_root_analogue();
```

Excellent, your reflexes work just as well at execution as they do at parsing! Let's implement the rest of what we've learned:

```rust
BuildInfo::from(form.id()).mark_own_module();
module!(
    form,
    "Concept forms, as opposed to archetypes.",
    ["form_trait::FormTrait"]
);
module!(relation, "Relations between the forms.");
module!(flag, "Relations involving only one form.");
module!(
    attribute,
    "Relations between two forms.",
    ["attribute_trait::AttributeTrait"]
);
let attr_trait = attribute.impl_trait();
BuildInfo::from(attr_trait.id()).set_implementation_name("AutoAttributeTrait");

module!(
    has_property,
    "Meta-attributes around what attributes instances of an archetype have."
);
module!(
    archetype,
    "Types of forms, as opposed to the forms themselves.",
    [
        "archetype_trait::ArchetypeTrait",
        "archetype_form_trait::ArchetypeFormTrait",
        "attribute_archetype_form_trait::AttributeArchetypeFormTrait"
    ]
);
```

## Appendix

### Dependencies

This is the version of Yang used to make this build happen:

```toml
zamm_yang = "0.2.0"
```

Yang does his best to be backwards-compatible, so we should let him know that we're new here:

```rust
Crate::yin().set_version("0.2.0");
Crate::yang().set_version("0.2.0");
```

We should also let him know what our current crate name is. There is as of yet no way to let him know that this is the same crate as the `Crate::yin()` mentioned above.

```rust
Crate::current().set_implementation_name("zamm_yin");
```

### Imports

These are the generic imports for general Yang generation:

```rust
use zamm_yang::add_flag;
use zamm_yang::define;
use zamm_yang::define_child;
use zamm_yang::module;
use zamm_yang::tao::initialize_kb;
use zamm_yang::tao::Tao;
use zamm_yang::tao::archetype::CreateImplementation;
use zamm_yang::tao::archetype::ArchetypeTrait;
use zamm_yang::tao::archetype::ArchetypeFormTrait;
use zamm_yang::tao::archetype::AttributeArchetypeFormTrait;
use zamm_yang::tao::form::rust_item::Crate;
use zamm_yang::tao::form::rust_item::CrateExtension;
use zamm_yang::tao::form::FormTrait;
use zamm_yang::tao::perspective::BuildInfo;
use zamm_yang::tao::perspective::KnowledgeGraphNode;
use zamm_yang::node_wrappers::CommonNodeTrait;
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::tao::callbacks::handle_all_implementations;
use zamm_yang::helper::aa;
use zamm_yang::helper::BackwardsCompatibility;
```
