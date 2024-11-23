use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
    std::collections::*,
};

//
// DataType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A data type defines the schema for user-defined data types in TOSCA. User-defined data types
/// comprise derived types that derive from from the TOSCA built-in types and complex types that
/// define collections of properties that each have their own data types.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional validation clause that must evaluate to True for values of this data type to
    /// be valid.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The optional map property definitions that comprise the schema for a complex data type in
    /// TOSCA.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// For data types that derive from the TOSCA map data type, the optional schema definition for
    /// the keys used to identify entries in properties of this data type. If not specified, the
    /// key_schema defaults to string. If present, the key_schema must derive from string. For data
    /// types that do not derive from the TOSCA map data type, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<Variant<AnnotatedT>>,

    /// For data types that derive from the TOSCA list or map data types, the mandatory schema
    /// definition for the entries in properties of this data type. For data types that do not
    /// derive from the TOSCA list or map data type, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<Variant<AnnotatedT>>,

    /// The data type of the number element of the scalar. Default value if not present is float.
    #[resolve]
    #[depict(option, as(depict))]
    pub data_type: Option<FullName>,

    /// Defines at least one unit string and its associated multiplier. At least one entry MUST
    /// have a multiplier value of one. The multiplier MUST be an integer or a float. If the
    /// data_type is integer then the multiplier MUST be an integer. If prefixes is used then the
    /// map MUST only contain one entry which MUST have a multiplier value of one.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub units: BTreeMap<ByteString, Variant<AnnotatedT>>,

    /// Informs the TOSCA processor which of the possible units to use when storing, computing and
    /// presenting scalars of this type. MUST be present if 'units has more than one multiplier of
    /// one. If not present the unit with multipler of one is the default canonical_unit.
    #[resolve]
    #[depict(option, style(string))]
    pub canonical_unit: Option<ByteString>,

    /// Defines at least one prefix and its associated multiplier. Where prefixes are defined they
    /// are prepended to the unit to obtain the unit string. This keyname is provided as a
    /// convenience so that metric units can use YAML anchor and alias to avoid repeating the table
    /// of SI prefixes.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub prefixes: BTreeMap<ByteString, Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl_type_entity!(DataType);

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Validation.
    pub fn get_validation(&self) -> Option<floria::Call> {
        self.get_conform_to_schema()
    }

    /// Conform to schema.
    pub fn get_conform_to_schema(&self) -> Option<floria::Call> {
        // TODO: "and" with self.validation

        Some(floria::Call::new(get_dispatch_name("conform_to_schema"), Default::default()))
    }
}

impl<AnnotatedT> Entity for DataType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        catalog: &mut Catalog,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let parent = get_complete_parent!(DATA_TYPE, self, derived_from, catalog, source_id, callstack, errors);

        complete_map!(self, properties, parent, catalog, source_id, errors);

        if let Some((parent, _scope)) = &parent {
            complete_validation(
                &mut self.validation,
                parent.validation.as_ref(),
                &mut self.annotations,
                &parent.annotations,
            );

            if_none_clone(
                &mut self.key_schema,
                &parent.key_schema,
                &mut self.annotations,
                &parent.annotations,
                "key_schema",
            );

            if_none_clone(
                &mut self.entry_schema,
                &parent.entry_schema,
                &mut self.annotations,
                &parent.annotations,
                "entry_schema",
            );
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// DataTypes
//

/// Map of [DataType].
pub type DataTypes<AnnotatedT> = BTreeMap<Name, DataType<AnnotatedT>>;
