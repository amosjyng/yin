# YIN 阴

[![Build Status](https://travis-ci.com/amosjyng/yin.svg?branch=master)](https://travis-ci.com/amosjyng/yin)

Yin is a rudimentary, experimental knowledge base. It is inspired by [Scone](https://github.com/sfahlman/scone), but is not meant to be a replacement/clone of Scone and its functionality. Yin is both used by, and depends on, [Yang](https://crates.io/crates/zamm_yang) for code generation. See the [internal documentation](yin.md) for more.

Functionality is basically non-existent at the moment. Please do not use this library.

Note that there is currently only single-threaded support. Unless using a Neo4j Cypher backend, every spawned thread will have its own independent instance of the KB.

### Semver

Semver "backwards compatibility" will be interpreted to mean any changes that do not break Yang's release builds. For example, there may be many changes in the public API in between Yin versions `0.x.0` and `0.x.y`. But so long as the Yang releases that depend on `0.x.0` do not break, and the releases that depend on the most recent `0.x.y` version also do not break, this will be considered "backwards compatible" for all practical purposes.

It's said when it comes to version numbers that [integers are cheap](https://github.com/semver/semver/issues/221#issuecomment-54581879) and version numbers should be [boring](https://xdg.me/version-numbers-should-be-boring/). While that may be, breaking changes carry some cost. Moreover, there could be psychological value in having *some* explicit marker to serve as an official stamp of recognition of an important milestone reached. Not so much for the users, who are nonexistent and don't care, but for the developer(s). That is why this project will attempt to keep the `0.x` incrementation as slow as practical.

## Examples

See [the docs](https://docs.rs/zamm_yin/) for examples of how to use the library.

## Development

By default, the Neo4j Cypher tests aren't run. To run them: 

 1. Ensure that Neo4j version 3 is installed and running (the `petgraph` dependency doesn't support version 4)
 2. Change the test DB password to `dummy_password`
 3. Run `cargo test -- --ignored`
