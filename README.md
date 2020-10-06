# YIN

[![Build Status](https://travis-ci.com/amosjyng/yin.svg?branch=master)](https://travis-ci.com/amosjyng/yin)

Yin is a rudimentary, experimental knowledge base. It is inspired by [Scone](https://github.com/sfahlman/scone), but is not meant to be a replacement/clone of Scone and its functionality.

## Examples

Functionality is basically non-existent at the moment. Please do not use this library.

### Initialization and concept creation

Note that there is currently only single-threaded support. Unless using a Neo4j Cypher backend, every spawned thread will have its own independent instance of the KB.

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
