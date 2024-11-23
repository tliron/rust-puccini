[⇐ to main site](https://puccini.cloud)

Puccini and Floria
==================

[TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) is, at face value, a language for describing *templates*, and indeed the Puccini compiler output is [Floria](https://floria.khutulun.org) templates, specifically Floria vertex templates and edge templates.

However, TOSCA's [operational model](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#tosca-operational-model) furthermore specifies an association between these templates and their "topology representations". This world of representations is where most TOSCA functions and interfaces operate.

TOSCA topology representations are implemented as Floria *instances*: vertexes and edges. Thus, in order to call TOSCA functions you must execute two phases: first compile the TOSCA service template to Floria templates, and then *instantiate* those Floria templates.

In an orchestration environment you would be working with a running Floria service. However, to ease validation and testing Puccini defaults to using a simple in-memory Floria store. By default `puccini-tosca compile` will compile into this store and then dump the resulting Floria templates to stdout. Adding the `--instantiate` flag will instantiate those templates and dump the instances to stdout instead. With the `--update` flag it will also update all the instance properties, which finally calls the TOSCA functions.

Design Principles
-----------------

An important design principle of Floria is that instances should not depend on templates. In other words, the template is optional: after instantiating a template it can be deleted, and indeed instances can be created directly without a template. This guarantees the fullest freedom for "Day 2" operations to modify the orchestration state even if we don't have an anticipated template for that modification. One way to guide and restrict this Day 2 behavior is to apply policies, which *could* potentially include design templates. The point is that Floria does not require them.

A related design principle of Floria is that entities are untyped. While Floria can associate entities with "classes", they are explicitly *not* types, rather they are meant for organization, categorization, selection, indexing, etc. In other words, classes are metadata rather than data. Like Floria templates, they are optional and could potentially be deleted.

Puccini adheres to these design principles by ensuring that all its generated Floria templates are *self-contained*. Specifically, TOSCA type information is stored entirely within the Floria templates and *nowhere else*. This includes TOSCA data types, meaning that a Floria property compiled from a TOSCA attribute has everything it needs to validate its data schema.

The details and implications of this decisions are discussed below.

Types
-----

TOSCA types become Floria classes.

Note that Floria offers no intrinsic handling of class inheritance. For this reason, each Floria entity (vertex templates, edge templates, properties) will be associated with not only the class representing its nominal TOSCA type, but also all the classes representing the type's ancestors. For example, if a node is of type "VirtualMachine", which inherits from "Compute", that node will be associated with *both* the "VirtualMachine" *and* the "Compute" classes.

This allows for efficient indexing. Selecting the "Compute" class will include all vertexes that are also of type "VirtualMachine". Because each class association is a simple ID, storage is also efficient.

Service Template, Node Templates, and Capabilities
--------------------------------------------------

Puccini generates 3 levels of Floria vertex template nestings:

At the bottom, the TOSCA service template becomes a single Floria vertex template. Contained within it, each TOSCA node template also becomes a Floria vertex template. If the node has capabilities, each capability *also* becomes a Floria vertex template. This final step allows Floria edges to connect to capabilities.

Groups and Policies
-------------------

TODO: both a vertex template and a class?

Requirements and Relationships
------------------------------

TOSCA requirements become Floria edge templates. Because a TOSCA relationship is always contained in a requirement, that relationship is embedded in the edge template rather than as a separate entity.

When this Floria edge template is instantiated, it is the implementation of the "relationship representation" in the TOSCA operational model.

Properties, Attributes, and Parameters
--------------------------------------

All of these become Floria properties.

TOSCA properties are marked as "read-only" Floria properties, such that they can only be updated once, which happens during the first "update" operation after instantiation. TOSCA parameters can be untyped, but this requires no special handling in Floria, where properties are never typed.

TOSCA's powerful data validation requires special handling.

### Data Types

Because each Floria property would be associated with the Floria classes representing its TOSCA data type, we could have used the class to store schema information that would apply to all properties of that type. For example, a custom scalar data type would store its unit and prefix tables there.

However, Floria classes are not meant to store data: they only have metadata. They are meant to be used for selection, e.g. for applying policies, transformations, etc., on entire classes of entities.

So instead we opted instead to not store schema information in the class and instead store it in the property. The disadvantage is that this data is duplicated for each property of that type. However, the important advantage is that allows for properties to be self-contained, following the design principles discussed above. Individual property schemas can be modified, and indeed be moved between classes, without affecting other properties of that type. This also has a performance advantage as classes do not need to be retrieved from the database in order to apply schemas.

Puccini implements the schema by introducing a handful of "internal" built-in functions, prefixed with `_`.

Central is the `$_apply` function, which applies a sequence of *coercion* expressions (which also, as a side effect, act as validators) to the familiar TOSCA `$value`.

The most important coercion function is `$_schema`, which coerces any value to adhere to a schema structure. This schema contains all the TOSCA data type information: primitive type validation, required properties, default values, key and entry schema for collections, special types (timestamp, version, and scalar—which has its own special schema), and of course arbitrary expressions from the user-defined `validation` keyname. All of these can be nested, too, for collections and types with `properties`.

This schema structure ends up having a non-trivial design because, unlike system programming languages, TOSCA allows for recursive data types. For example, a data type deriving from a TOSCA `list` can have an `entry_schema` which is of the same type. If we were to naively nest these two schema structures we would hit infinite recursion. Instead, Puccini's schema structure is in fact a collection of *indexed* schemas, such that any schema can refer to another schema by a numerical index. This allows the nesting to happen during runtime, where the bounds of recursion are limited by the (finite) size of the value itself. This schema structure is easy enough to use, but not so trivial to generate from TOSCA.

Note that because the TOSCA `validation` keyname expects a boolean expression, it must be turned into coercion expression in order to be used in `$_apply`. We do this by wrapping it in an `$_assert` function, which simply raises an error if the expression does not evaluate to true.

The final complex expression, which combines `$_apply`, `$_schema`, and `$_assert` calls, is placed in the Floria property's "preparer", which is evaluated whenever its value is updated.

### Functions in Values

When you assign a value to a property, attribute, or parameter in TOSCA (including in the `default` and `value` keynames), you are allowed to embed function calls.

If there are no function calls, Puccini optimizes by simply placing the value as is in the Floria property value. (The "preparer" detailed above will be applied to it.)

Otherwise, Puccini wraps the value in the newly introduced `$_evaluate` function and places it in the Floria property's "updater". The "updater" is called whenever we issue an update operation on the property (this happens at least once, when the template is instantiated). The "preparer" is then called to ensure that the updated value is valid.

Note that the Floria property "updater" is not the only source of potential updates. For example, various orchestration events can cause properties to be updated from external data. This is the intended use for TOSCA attributes, and indeed TOSCA provides one way of updating attributes: interface notifications (see below). Whatever the source, the "preparer" will always be called to ensure that the value is valid.

Built-In Functions
------------------

All of TOSCA's built-in functions are provided as a single Wasm file. They are written in Rust using the [Floria Plugin SDK](https://floria.khutulun.org). This Wasm is embedded in the Puccini executable for convenience and will be delivered to a running Floria service during compilation.

Note that these are *Floria functions* and so they work with the Floria graph of vertexes, edges, and their properties (the TOSCA "topology representations"). It is thus relatively straightforward to implement TOSCA Path functions, such as `$get_property` and `$get_attribute`. We provide a general-purpose TOSCA Path parser/follower that can be reused by other functions.

The comparison functions, such as `$less_than` and `$greater_or_equal` are a bit more subtle, specifically when comparing the special TOSCA types: version, timestamp, and scalar. TOSCA allows you to compare these to their unparsed forms, e.g. `{ $less_than: [ $value, "2 GB" ] }`. Thus each of these must provide a specialized comparison implementation. Moreover, scalars must embed their own embedded schema so that the other expression could be parsed to a comparable form.

By wonderful coincidence, Floria supports a custom data type exactly for these kinds of situations. We can thus simply mark our special types as custom so that we know to treat them specially.

Custom Functions
----------------

Since TOSCA 2.0, you can formally declare [custom functions](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#104-function-definitions-).

With Puccini, these can be implemented in the same way we've implemented the built-in functions: Use the Floria Plugin SDK to program your own functions into a Wasm file. Include it in your CSAR, and that's it. Puccini will send it to the running Floria service in addition to the built-in Wasm.

If you're just testing locally this will work, too, using the in-memory Floria store included in Puccini.

Interfaces, Operations, and Notifications
-----------------------------------------

TODO: Notifications -> Floria property updater?
