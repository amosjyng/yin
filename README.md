# YIN

Yin is a rudimentary, experimental knowledge base. It is inspired by [Scone](https://github.com/sfahlman/scone), but is not meant to be a replacement/clone of Scone and its functionality.

## Functionality

Functionality is basically non-existent at the moment. Please do not use this library.

### Concept creation

```rust
use yin::graph::bind_in_memory_graph;
use yin::concepts::{Tao, ArchetypeTrait, FormTrait};
use yin::wrappers::CommonNodeTrait;
use std::rc::Rc;

fn main() {
    // Initialize the knowledge-base
    bind_in_memory_graph();

    // Create a new concept
    let mut concept = Tao::individuate();
    assert!(concept.has_ancestor(Tao::archetype()));

    // Set a name for the concept
    concept.set_internal_name("Hello, world.".to_string());
    println!("{}", concept.internal_name().unwrap());
}
```
