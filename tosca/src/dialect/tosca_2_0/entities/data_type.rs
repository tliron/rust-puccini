use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type_kind::*,
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::{collections::*, str::*},
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

    /// Data type kind.
    #[depict(option, style(symbol))]
    pub kind: Option<DataTypeKind>,

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
    /// Apply to variant.
    pub fn apply(&self, variant: Variant<AnnotatedT>) -> Result<Variant<AnnotatedT>, ToscaError<AnnotatedT>> {
        if let Some(kind) = self.kind {
            match kind {
                DataTypeKind::Scalar => {
                    return Ok(self.to_scalar(&variant)?.into());
                }

                DataTypeKind::Timestamp => {
                    return Ok(self.to_timestamp(&variant)?.into());
                }

                DataTypeKind::Version => {
                    return Ok(self.to_version(&variant)?.into());
                }

                _ => {}
            }
        }

        Ok(variant)
    }

    /// To scalar.
    pub fn to_scalar(&self, variant: &Variant<AnnotatedT>) -> Result<Scalar, ToscaError<AnnotatedT>> {
        let Variant::Text(text) = variant else {
            return Err(IncompatibleVariantTypeError::new(variant, &["text"]).into());
        };

        // TODO: get data type and check if kind=integer
        let integer = match &self.data_type {
            Some(data_type) => data_type.name.0 == "integer",
            None => false, // defaults to float
        };

        Ok(Scalar::parse(text.as_str(), integer, &self.units, &self.canonical_unit, &self.prefixes)
            .map_err(|error| error.with_annotations_from(variant))?)
    }

    /// To timestamp.
    pub fn to_timestamp(&self, variant: &Variant<AnnotatedT>) -> Result<Timestamp, ToscaError<AnnotatedT>> {
        let Variant::Text(text) = variant else {
            return Err(IncompatibleVariantTypeError::new(variant, &["text"]).into());
        };

        Ok(Timestamp::from_str(text.as_str()).map_err(|error| {
            MalformedError::new("timestamp".into(), error.to_string()).with_annotations_from(variant)
        })?)
    }

    /// To version.
    pub fn to_version(&self, variant: &Variant<AnnotatedT>) -> Result<Version, ToscaError<AnnotatedT>> {
        let Variant::Text(text) = variant else {
            return Err(IncompatibleVariantTypeError::new(variant, &["text"]).into());
        };

        Ok(Version::from_str(text.as_str())
            .map_err(|error| MalformedError::new("version".into(), error.to_string()).with_annotations_from(variant))?)
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

        complete_map!(properties, self, parent, catalog, source_id, errors);

        if let Some((parent, _scope)) = &parent {
            complete_validation!(self, parent);

            if_none_clone!(key_schema, self, parent);
            if_none_clone!(entry_schema, self, parent);
            if_none_clone!(data_type, self, parent);
            if_none_clone!(canonical_unit, self, parent);
            if_empty_clone!(units, self, parent);
            if_empty_clone!(prefixes, self, parent);

            // TODO: validate data_type, units, and prefixes

            if self.kind.is_none() && parent.kind.is_some() {
                self.kind = parent.kind.clone();
            }
        }

        if self.kind.is_none() {
            self.kind = Some(DataTypeKind::Complex);
        }

        if let Some(kind) = self.kind {
            if !self.properties.is_empty() {
                if kind != DataTypeKind::Complex {
                    errors.give(
                        InvalidKeyError::new("properties".into()).with_annotations_from_field(self, "properties"),
                    )?;
                }
            }

            if self.key_schema.is_some() {
                if kind != DataTypeKind::Map {
                    errors.give(
                        InvalidKeyError::new("key_schema".into()).with_annotations_from_field(self, "key_schema"),
                    )?;
                }
            }

            if self.entry_schema.is_some() {
                if (kind != DataTypeKind::Map) && (kind != DataTypeKind::List) {
                    errors.give(
                        InvalidKeyError::new("entry_schema".into()).with_annotations_from_field(self, "entry_schema"),
                    )?;
                }
            }

            if self.data_type.is_some() {
                if kind != DataTypeKind::Scalar {
                    errors.give(
                        InvalidKeyError::new("data_type".into()).with_annotations_from_field(self, "data_type"),
                    )?;
                }
            }

            if !self.units.is_empty() {
                if kind != DataTypeKind::Scalar {
                    errors.give(InvalidKeyError::new("units".into()).with_annotations_from_field(self, "units"))?;
                }
            }

            if self.canonical_unit.is_some() {
                if kind != DataTypeKind::Scalar {
                    errors.give(
                        InvalidKeyError::new("canonical_unit".into())
                            .with_annotations_from_field(self, "canonical_unit"),
                    )?;
                }
            }

            if !self.prefixes.is_empty() {
                if kind != DataTypeKind::Scalar {
                    errors
                        .give(InvalidKeyError::new("prefixes".into()).with_annotations_from_field(self, "prefixes"))?;
                }
            }
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
