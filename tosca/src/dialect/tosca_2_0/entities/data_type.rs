use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        dispatch::*,
        property_definition::*,
        value_assignment::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::debug::*,
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
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// The optional validation clause that must evaluate to True for values of this data
    /// type to be valid.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub validation: Option<ValueAssignment<AnnotatedT>>,

    /// The optional map property definitions that comprise the schema for a complex data
    /// type in TOSCA.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// For data types that derive from the TOSCA map data type, the optional schema
    /// definition for the keys used to identify entries in properties of this data type.
    /// If not specified, the key_schema defaults to string. If present, the key_schema
    /// must derive from string. For data types that do not derive from the TOSCA map data
    /// type, the key_schema is not allowed.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub key_schema: Option<Variant<AnnotatedT>>,

    /// For data types that derive from the TOSCA list or map data types, the mandatory
    /// schema definition for the entries in properties of this data type. For data types
    /// that do not derive from the TOSCA list or map data type, the entry_schema is not
    /// allowed.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub entry_schema: Option<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

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
        depot: &mut Depot,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);

        let Some(derived_from) = &self.derived_from else {
            self.completion = Completion::Complete;
            return Ok(());
        };

        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        let Some(parent) = depot
            .get_complete_entity_next::<Self, _, _>(
                GROUP_TYPE,
                derived_from,
                source_id,
                callstack,
                &mut errors.with_field_annotations(self, "derived_from"),
            )?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &derived_from.scope;

        if_none_clone(&mut self.validation, &parent.validation);

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        if_none_clone(&mut self.key_schema, &parent.key_schema);
        if_none_clone(&mut self.entry_schema, &parent.entry_schema);

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Type<OldCatalog<'own, AnnotatedT>, AnnotatedT> for DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_entity_name() -> &'static str {
        "DataType"
    }

    fn get_floria_group_id_prefix() -> &'static str {
        "data"
    }

    fn get_version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    fn get_description(&self) -> Option<&ByteString> {
        self.description.as_ref()
    }

    fn get_metadata(&self) -> &Metadata<AnnotatedT> {
        &self.metadata
    }

    fn get_parent_name(&self) -> Option<&FullName> {
        self.derived_from.as_ref()
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: TypeCompleteContext<'_, Self, OldCatalog<'_, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            derived_from: self.derived_from.clone(),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            validation: complete_clone_from(&self.validation, context.parent_type, |entity| &entity.validation),
            properties: self.properties.complete(
                DefinitionsCompleteContext {
                    parent_definitions: context.parent_type.map(|entity| &entity.properties),
                    types: &context.catalog.data_types,
                    catalog: context.catalog,
                    index: context.index,
                },
                errors,
            )?,
            key_schema: complete_clone_from(&self.key_schema, context.parent_type, |entity| &entity.key_schema),
            entry_schema: complete_clone_from(&self.entry_schema, context.parent_type, |entity| &entity.entry_schema),
            annotations: self.annotations.clone(),
            completion: Default::default(),
        })
    }
}

//
// DataTypes
//

/// Map of [DataType].
pub type DataTypes<AnnotatedT> = BTreeMap<Name, DataType<AnnotatedT>>;
