use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
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

    /// Data kind.
    #[depict(option, style(string))]
    pub data_kind: Option<DataKind>,

    /// Scalar data kind.
    #[depict(option, style(symbol))]
    pub scalar_data_kind: Option<DataKind>,

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
    /// Initialize a schema. Return a reference to the relevant value schema if there is one.
    pub fn initialize_schema(&self, schema: &mut Schema<AnnotatedT>) -> Option<SchemaReference> {
        if let Some(data_kind) = self.data_kind {
            match data_kind {
                DataKind::Scalar => {
                    if !self.units.is_empty() {
                        return Some(schema.get_or_add_value(self.scalar_schema().into()));
                    }
                }

                DataKind::List => {
                    if self.entry_schema.is_some() {
                        return self.initialize_list_schema(schema);
                    }
                }

                DataKind::Map => {
                    if self.entry_schema.is_some() {
                        return self.initialize_map_schema(schema);
                    }
                }

                DataKind::Struct => {
                    if !self.properties.is_empty() {
                        return self.initialize_struct_schema(schema);
                    }
                }

                _ => {
                    return Some(schema.get_or_add_value(data_kind.into()));
                }
            }
        }

        None
    }

    /// Scalar schema.
    pub fn scalar_schema(&self) -> ScalarSchema<AnnotatedT> {
        ScalarSchema::new(self.scalar_data_kind, self.units.clone(), self.canonical_unit.clone(), self.prefixes.clone())
    }

    /// Initialize list schema.
    pub fn initialize_list_schema(&self, schema: &mut Schema<AnnotatedT>) -> Option<SchemaReference> {
        // Placeholder
        let list_reference = schema.add_value(0.into());

        let list_schema = ListSchema::default();

        // Replace placeholder
        schema.values[list_reference] = list_schema.into();
        Some(list_reference)
    }

    /// Initialize map schema.
    pub fn initialize_map_schema(&self, schema: &mut Schema<AnnotatedT>) -> Option<SchemaReference> {
        // Placeholder
        let map_reference = schema.add_value(0.into());

        let map_schema = MapSchema::default();

        // Replace placeholder
        schema.values[map_reference] = map_schema.into();
        Some(map_reference)
    }

    /// Initialize struct schema.
    pub fn initialize_struct_schema(&self, schema: &mut Schema<AnnotatedT>) -> Option<SchemaReference> {
        // Placeholder
        let struct_reference = schema.add_value(0.into());

        let mut struct_schema = StructSchema::default();
        // for (name, property) in &self.properties {
        //     // TODO: use property's data type

        //     let data_type: &Self = todo!();
        //     //get_complete_entity!(DATA_TYPE, DataType, self, type_name, catalog, source_id, errors);

        //     let field_reference = data_type.initialize_schema(schema).unwrap();

        //     struct_schema.fields.insert(name.clone(), field_reference);
        // }

        // Replace placeholder
        schema.values[struct_reference] = struct_schema.into();
        Some(struct_reference)
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

        complete_map_for_field!(properties, self, parent, catalog, source_id, errors);

        if let Some((parent, _scope)) = &parent {
            if_none_clone!(key_schema, self, parent);
            if_none_clone!(entry_schema, self, parent);
            if_none_clone!(data_type, self, parent);
            if_none_clone!(canonical_unit, self, parent);
            if_empty_clone!(units, self, parent);
            if_empty_clone!(prefixes, self, parent);

            if self.data_kind.is_none() && parent.data_kind.is_some() {
                self.data_kind = parent.data_kind;
            }
        }

        if self.data_kind.is_none() {
            self.data_kind = Some(DataKind::Struct);
        }

        // TODO: derived from Scalar requires units
        // TODO: derived from List requires entry_schema
        // TODO: derived from Map requires entry_schema (key_schema defaults to String)
        //
        // TODO: key_schema must be String or derived from it
        // TODO: scalar_data_kind must be Integer or Float
        // TODO: units has to map text -> numbers
        // TODO: prefixes has to map text -> numbers
        // TODO: units*prefixes combinations have to be unique (unambiguous)

        // Check if keynames are allowed

        if let Some(kind) = self.data_kind {
            if !self.properties.is_empty() {
                if kind != DataKind::Struct {
                    errors.give(InvalidKeyError::new(
                        Variant::from("properties").with_annotations_from_field(self, "properties"),
                    ))?;
                }
            }

            if self.key_schema.is_some() {
                if kind != DataKind::Map {
                    errors.give(InvalidKeyError::new(
                        Variant::from("key_schema").with_annotations_from_field(self, "key_schema"),
                    ))?;
                }
            }

            if self.entry_schema.is_some() {
                if (kind != DataKind::Map) && (kind != DataKind::List) {
                    errors.give(InvalidKeyError::new(
                        Variant::from("entry_schema").with_annotations_from_field(self, "entry_schema"),
                    ))?;
                }
            }

            if self.data_type.is_some() {
                if kind != DataKind::Scalar {
                    errors.give(InvalidKeyError::new(
                        Variant::from("data_type").with_annotations_from_field(self, "data_type"),
                    ))?;
                }
            }

            if !self.units.is_empty() {
                if kind != DataKind::Scalar {
                    errors.give(InvalidKeyError::new(
                        Variant::from("units").with_annotations_from_field(self, "units"),
                    ))?;
                }
            }

            if self.canonical_unit.is_some() {
                if kind != DataKind::Scalar {
                    errors.give(InvalidKeyError::new(
                        Variant::from("canonical_unit").with_annotations_from_field(self, "canonical_unit"),
                    ))?;
                }
            }

            if !self.prefixes.is_empty() {
                if kind != DataKind::Scalar {
                    errors.give(InvalidKeyError::new(
                        Variant::from("prefixes").with_annotations_from_field(self, "prefixes"),
                    ))?;
                }
            }
        }

        if self.scalar_data_kind.is_none()
            && let Some(data_type) = &self.data_type
            && let Some(data_type) =
                catalog.get_complete_entity::<Self, _, _>(DATA_TYPE, data_type, source_id, errors)?
            && data_type.data_kind.is_some()
        {
            self.scalar_data_kind = data_type.data_kind;
        }

        let mut schema = Default::default();
        self.initialize_schema(&mut schema);
        self.validation.finalize_validation(schema.into_option());

        if let Some((parent, _scope)) = &parent {
            complete_validation!(self, parent);
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
