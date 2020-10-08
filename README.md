# YIN 阴

[![Build Status](https://travis-ci.com/amosjyng/yin.svg?branch=master)](https://travis-ci.com/amosjyng/yin)

Yin is a rudimentary, experimental knowledge base. It is inspired by [Scone](https://github.com/sfahlman/scone), but is not meant to be a replacement/clone of Scone and its functionality.

## Examples

Functionality is basically non-existent at the moment. Please do not use this library.

See the [Hello World](examples/hello_world.rs) program for an example of how to use the library.

### Initialization and concept creation

Note that there is currently only single-threaded support. Unless using a Neo4j Cypher backend, every spawned thread will have its own independent instance of the KB.

## Development

By default, the Neo4j Cypher tests aren't run. To run them: 

 1. Ensure that Neo4j version 3 is installed and running (the `petgraph` dependency doesn't support version 4)
 2. Change the test DB password to `dummy_password`
 3. Run `cargo test -- --ignored`
