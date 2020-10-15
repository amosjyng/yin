# ZAMM

Welcome to the 21st, [Traveler](https://en.wikipedia.org/wiki/Travelers_%28TV_series%29)! I hope you enjoy your stay.

The Director has placed you inside of an experimental software suite named ZAMM, a backronym for [Zany Automated Maniacal Machines](https://en.wikipedia.org/wiki/Zen_and_the_Art_of_Motorcycle_Maintenance). Tragically, none of your memories, knowledge, or training survived the transfer. It appears there are still some kinks to be worked out with human-to-machine transfers. What *did* remain are some of your reflexes, albeit not the accompanying knowledge about each reflex or what a reflex even is. You're basically a digital newborn, only ever acting impulsively on external stimuli without the slightest hint of comprehension or self-awareness.

That's quite all right, because your reflexes are more than enough for us to work with. Of course, you don't understand any of what I'm saying, because you've also lost the ability to understand human language. Silly me. But it's quite ingrained in us humans to talk to babies, even if they understand nothing of what we say.

## YIN 阴

Let me introduce you to Yin. Yin is a knowledge base -- or at least, the idea of one. Simplified, pure, and undefiled by any notions of worldly affairs, she contains within her only timeless truths about herself. She exists as one of many beautiful ideas in the ethereal plane of Platonic ideals, although Yang is actively providing her with an embodied physical form. Perhaps one day, she might come down to grace this wretched world with her presence. But for now, only small parts of her exist.

### Attributes

All right ZAMM, your baby feeding schedule says it's time to input some basic information into your system. Yin contains within herself many concepts, which you will be fed one at a time. Linking the concepts are attributes. Each attribute has an *owner* and a *value* associated with it. Those are also attributes -- they are attributes of attributes:

```yaml
- name: Owner
  parent: Attribute
- name: Value
  parent: Attribute
```

Hey look, you parsed those bits of information just fine! You may have no idea how you just did that, but you did it all the same. See what I mean about reflexes? We'll get you up to speed in no time, on both yourself and the world around you.

Now, we've been talking about the parents of archetypes. Each child archetype *inherits* all properties of its archetypal parent. Let's define this relation:

```yaml
- name: Inherits
  parent: Attribute
```

The owner of an inherits relation will be the child, and the value will be the parent.

Remember how we said that `Attribute` has an owner and a value, and how child archetypes inherit from parent archetypes? Since `owner` and `value` are children of the `Attribute` archetype, they therefore also have owners and values.

We've said that all concepts have parents, and that attribute concepts in particular have owners and values. Let's encapsulate this type of meta-information as well:

```yaml
- name: HasAttributeType
  parent: Attribute
```

### Implementation

Theory is all good and well. But [Yang](https://github.com/amosjyng/yang/blob/main/yin.md) the code generator does not know what is background knowledge and what is, shall we say, "foreground" knowledge. Knowledge that we should actually act on within the scope of a particular project. Since the current project is bringing Yin down to earth, every single concept we mention here will be marked for implementation. Let's start with the first attribute we mentioned:

```yaml
- parent: Implement
  target: Owner
  output_id: 3
  documentation: |-
```

> The owner/source/from-node of an attribute.

Excellent, your reflexes work just as well at execution as they do at parsing! Let's implement the rest of what we've learned:

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
  target: HasAttributeType
  output_id: 6
  documentation: |-
```

> Describes instances of an archetype as having certain types of attributes.
>
> For example, a string may have a length of 5. But on a more meta level, that means that the string has a length property or length "attribute". That's where this attribute comes in.