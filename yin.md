# ZAMM

Welcome to the 21st, [Traveler](https://en.wikipedia.org/wiki/Travelers_%28TV_series%29)! I hope you enjoy your stay.

The Director has placed you inside of an experimental software suite named ZAMM, a backronym for [Zany Automated Maniacal Machines](https://en.wikipedia.org/wiki/Zen_and_the_Art_of_Motorcycle_Maintenance). Tragically, none of your memories, knowledge, or training survived the transfer. It appears there are still some kinks to be worked out with human-to-machine transfers. What *did* remain are some of your reflexes, albeit not the accompanying knowledge about each reflex or what a reflex even is. You're basically a digital newborn, only ever acting impulsively on external stimuli without the slightest hint of comprehension or self-awareness.

That's quite all right, because your reflexes are more than enough for us to work with. Of course, you don't understand any of what I'm saying, because you've also lost the ability to understand human language. Silly me. But it's quite ingrained in us humans to talk to babies, even if they understand nothing of what we say.

## YIN 阴

Let me introduce you to Yin. Yin is a knowledge base -- or at least, the idea of one. Simplified, pure, and undefiled by any notions of worldly affairs, she contains within her only timeless truths about herself. She exists as one of many beautiful ideas in the ethereal plane of Platonic ideals, although Yang is actively providing her with an embodied physical form. Perhaps one day, she might come down to grace this wretched world with her presence. But for now, only small parts of her exist.

### Forms

We begin by assuming nothing. We can at least name ideas as they come up.

The first idea is that we can only ever reason with and manipulate objects that have form. So let's start by giving everything a basic form:

```yaml
- define: Form
```

Hey look, you parsed those bits of information just fine! You may have no idea how you just did that, but you did it all the same. See what I mean about reflexes? We'll get you up to speed in no time, on both yourself and the world around you.

[Everything is a part of ultimate reality, but reality is also a part of everything.](https://biblehub.com/john/14-20.htm)

### Relations

Each form alone would make its own universe, indistinguishable from all the universes made by other solitary forms. But we have now encountered the first difference. There is something, but there is also something else. Opposite though they may be, they are still inextricably tied together through this oppositeness.

This calls for a way to associate nodes with each other to make up a much richer reality:

```yaml
- define: Relation
```

One node describing one other node -- a unary relation. Take a step forward, and you have one node describing two other nodes -- a binary relation. Take a step backward, and you have a node describing no other nodes -- and we have recovered the Tao, the solitary node, the null set, the 0-ary relation that stands by itself for all eternity within every possible and impossible reality.

There are [infinitely more](https://en.wikipedia.org/wiki/Finitary_relation) forward steps to take. At the very end of this long journey, when you have related all nodes to each other, you end up once again with the Tao -- a proclamation of the totality of all reality that is simultaneously a meaningless declaration of nothing about reality. What difference does a bitstream of all one's have from a bitstream of all zero's?

We'll call the unary relations "flags":

```yaml
- define: Flag
```

Take a hypothetical unary relation. Let's call it `U`. Like all other unary relations, `U` describes one other node at a time -- the other node potentially being itself. Let's call this other node `O`.

But wait, the unary relation is a concept, an idea, in and of itself. It is a first-class citizen in the plane of concept space, as evidenced by the fact that we're able to refer to it by a name. And this `U` is clearly an acquaintance of `O`'s, as we have only been introduced to `O` through `U`. So here we are, two nodes related to each other in some way -- a *binary* relation that describes two nodes at once.

We'll call the binary relations "attributes":

```yaml
- define: Attribute
```

What should we call the binary relation we've just described between `O` and `U`? Let's say that `O` is the "owner" of the unary relation `U`:

```yaml
- define: Owner
```

This also applies to attributes -- we can call `O` the owner of the Owner relation that runs between `O` and `U`. But attributes, unlike flags, describe two nodes at a time, and we only have a name for the relation between an attribute and the first node the attribute describes. We should therefore come up with another name for the relation between an attribute and the second node the attribute describes:

```yaml
- define: Value
```

And so there we have it, Owner and Value as attributes describing attributes, including themselves!

Both flags and attributes have something in common -- the existence of their owner node. We should describe this commonality. Just about everything that can be said about the first (and only) node of a flag in relation to the flag can also be said about the first node of an attribute in relation to the attribute.

```yaml
- define: Inherits
  parent: Attribute
```

Even the `Inherits` attribute itself inherits from Attribute! Exciting, we can not only name things now, but also start describing their inheritance patterns.

Let's call the owner of an inherits relation the "child," and the value the "parent." Each child archetype *inherits* all relations of its archetypal parent. Let's now define the inheritance relations for all the concepts we've defined so far:

```yaml
- name: Form
  parent: Tao
- name: Relation
  parent: Tao
- name: Flag
  parent: Relation
- name: Attribute
  parent: Relation
- name: Owner
  parent: Attribute
- name: Value
  parent: Attribute
```

Ah, this is starting to look more like a proper universe, where all entities are connected to all other entities.

Let's also call unary and binary relations by the name of "properties." Note that the Owner attribute, as a child of Attribute, also inherits all properties of Attributes. All attributes have owners and values, and therefore each Owner attribute iteslf will also have an owner and a value to it. Ditto for Value. We should describe this property of a concept having properties:

```yaml
- define: HasProperty
  parent: Attribute
```

And go back and set this property for Attribute:

```yaml
- name: Attribute
  attributes:
    - Owner
    - Value
```

Now we can say that unary relations, binary relations, and all the n-ary relations where n > 1, all have owners. While we should theoretically exclude 0-ary relations from this, we will instead delegate all reasoning about 0-ary relations to Form, so that we can simply ascribe the "owner" property to all relations.

Now, while we've encapsulated the idea that all flags and attributes have owners, we also want to encapsulate the idea that different flags and attributes will have owners and values of different types:

```yaml
- define: OwnerArchetype
  parent: Attribute
  owner_archetype: Relation
- define: ValueArchetype
  parent: Attribute
  owner_archetype: Attribute
```

As you can see, even `OwnerArchetype` and `ValueArchetype` have restrictions on their owners. For completness, let's describe the owner and value types of all the other relations we've defined:

```yaml
- name: Relation
  owner_archetype: Tao
- name: Attribute
  value_archetype: Tao
- name: Owner
  owner_archetype: Relation
- name: Value
  owner_archetype: Attribute
- name: HasProperty
  value_archetype: Relation
```

Remember that because Attribute inherits from Relation, Attribute also has an owner archetype set to Tao, so we've covered all our tracks here. Every flag and attribute has an owner, every attribute also has a value, and some attributes only apply to other attributes.

### Implementation

Theory is all good and well. But [Yang](https://github.com/amosjyng/yang/blob/main/yin.md) the code generator does not know what is background knowledge and what is, shall we say, "foreground" knowledge. Knowledge that we should actually act on within the scope of a particular project. Since the current project is bringing Yin down to earth, every single concept we mention here will be marked for implementation. Let's start with the first attribute we mentioned:

```yaml
- parent: Implement
  target: Form
  output_id: 10
  force_own_module: true
  documentation: |-
```

> All things that can be interacted with have form.

Excellent, your reflexes work just as well at execution as they do at parsing! Let's implement the rest of what we've learned:

```yaml
- parent: Implement
  target: Relation
  output_id: 11
  documentation: |-
```

> Links any number of nodes together.

```yaml
- parent: Implement
  target: Flag
  output_id: 12
  force_own_module: true
  documentation: |-
```

> Represents a unary relation.

```yaml
- parent: Implement
  target: Attribute
  output_id: 2
  attribute_logic: true
  documentation: |-
```

> Represents a binary relation.

```yaml
- parent: Implement
  target: Owner
  output_id: 3
  documentation: |-
```

> The owner/source/from-node of an attribute.

```yaml
- parent: Implement
  target: Value
  output_id: 4
  documentation: |-
```

> The value/target/to-node of an attribute.

```yaml
- parent: Implement
  target: Inherits
  output_id: 5
  documentation: |-
```

> Describes the owner as inheriting all attributes of the value.

```yaml
- parent: Implement
  target: HasProperty
  output_id: 6
  documentation: |-
```

> Describes instances of an archetype as having certain other properties.
>
> For example, a string may have a length of 5. But on a more meta level, that means that the string has a length property or length "attribute". That's where this attribute comes in.

```yaml
- parent: Implement
  target: OwnerArchetype
  output_id: 7
  documentation: |-
```

> The type of owner this attribute has. Only the most restrictive inherited value will be used.

```yaml
- parent: Implement
  target: ValueArchetype
  output_id: 8
  documentation: |-
```

> The type of value this attribute has. Only the most restrictive inherited value will be used.