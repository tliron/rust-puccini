*Work in progress, not ready for general use. For now, see the [previous version](https://github.com/tliron/go-puccini).*

Puccini
=======

Tools for working with [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) (Topology and Orchestration Specification for Cloud Applications) and CSAR (Cloud Service Archive).

It comprises:

* Libraries for TOSCA (parsing, validation, compilation) and CSAR (validation, reading, writing)
* [Wasm](https://webassembly.org) implementations for TOSCA's built-in functions
* CLI tools based on the above:

`puccini-tosca`
---------------

TOSCA parser, validator, and compiler.

The compilation output is a [Floria](https://floria.khutulun.org) template, which can be also be instantiated by Puccini in order to test imperative behaviors such as updating properties, attributes, and outputs, and calling operations and notifications.

Puccini's use of Floria is documented
[here](https://puccini.cloud/documentation/floria).

For a Floria-based cloud orchestrator, see [Khutulun](https://khutulun.org).

`puccini-csar`
--------------

Can create compliant CSAR files, validate existing ones, extract contained artifacts, and query or modify the metadata.

Supports both tarballs and zip files.

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

* [Apache License, Version 2.0](https://github.com/tliron/puccini/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/puccini/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
