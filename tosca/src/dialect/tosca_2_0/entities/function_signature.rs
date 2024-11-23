use super::implementation_definition::*;

use {
    compris::{annotate::*, resolve::*},
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// FunctionSignature
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// Function signatures can be defined in TOSCA profiles or TOSCA service templates using a YAML
/// map under the functions keyname using the grammar specified below. Note that this grammar
/// allows the definition of functions that have arguments expressed within a YAML seq, however
/// intrinsic functions may accept other argument definition syntaxes.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
pub struct FunctionSignature<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// All defined arguments must be used in the function invocation (and in the order defined
    /// here). If no arguments are defined, the signature either accepts no arguments or any
    /// arguments of any form (depending on if the variadic keyname is false or true).
    #[depict(iter(kv), key_style(string), style(string))]
    pub arguments: BTreeMap<ByteString, ByteString>,

    /// Optional arguments may be used in the function invocation after the regular arguments.
    /// Still the order defined here must be respected.
    #[depict(iter(kv), key_style(string), style(string))]
    pub optional_arguments: BTreeMap<ByteString, ByteString>,

    /// Specifies if the last defined argument (or optional_arguments if defined) may be repeated
    /// any number of times in the function invocation. If this value is not specified, a default
    /// of False is assumed.
    #[depict(style(symbol))]
    pub variadic: bool,

    /// Defines the type of the function result. If no result keyname is defined, then the function
    /// may return any result
    #[depict(style(symbol))]
    pub result: bool,

    /// Defines the implementation (e.g., artifact) for the function. The same definition as for
    /// operation/notification implementation is used.
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

//
// FunctionSignatures
//

/// Map of [FunctionSignature].
pub type FunctionSignatures<AnnotatedT> = BTreeMap<ByteString, FunctionSignature<AnnotatedT>>;
