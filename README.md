*Work in progress, not ready for general use. For now, see the [previous version](https://github.com/tliron/puccini).*

Puccini
=======

Tools for working with [TOSCA](https://www.oasis-open.org/committees/tosca/) (Topology and Orchestration Specification for Cloud Applications) and CSAR (Cloud Service Archive).

It comprises:

* Libraries for working with TOSCA and CSAR
* [Wasm](https://webassembly.org/) implementations for TOSCA's built-in functions
* CLI tools based on the above:

`puccini-tosca`
---------------

TOSCA parser, validator, and compiler.

The compilation output is a [Floria](https://github.com/tliron/rust-floria) template, which can be also be instantiated by Puccini in order to test imperative behaviors such as updating properties, attributes, and outputs, and calling operations and notifications.

For a Floria-based cloud orchestrator, see [Khutulun](https://github.com/tliron/rust-khutulun).

`puccini-csar`
--------------

CSAR package builder.

FAQ
---

### Why is it called "Puccini"?

Named after [Giacomo Puccini](https://en.wikipedia.org/wiki/Giacomo_Puccini), the composer of the [*Tosca*](https://en.wikipedia.org/wiki/Tosca) opera (based on Victorien Sardou's play, [*La Tosca*](https://en.wikipedia.org/wiki/La_Tosca)), as well as *La bohème*, *Madama Butterfly*, and other famous works. The theme here is orchestration, orchestras, composition, and thus operas. Capiche?

### How to pronounce "Puccini"?

Usually: "poo-CHEE-nee" ("ch" as in "change").

For a demonstration of its authentic 19th-century Italian pronunciation see [this clip](https://www.youtube.com/watch?v=dQw4w9WgXcQ).

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/rust-khutulun/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/rust-khutulun/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
