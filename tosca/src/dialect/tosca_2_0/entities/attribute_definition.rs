use super::{
    super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
    data_type::*,
    value_assignment::*,
    value_assignments::*,
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
// AttributeDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An attribute definition defines a named, typed value that can be associated with an entity defined
/// in this specification (e.g., a node, relationship or capability type). Specifically, it is used
/// to expose the actual state of some a TOSCA entity after it has been deployed and instantiated (as
/// set by the TOSCA orchestrator).
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the attribute.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// The optional description for the attribute.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional key that may provide a value to be used as a default if not provided by another
    /// means. This value SHALL be type compatible with the type declared by the attribute
    /// definition's type keyname.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub default: Option<ValueAssignment<AnnotatedT>>,

    /// The optional validation clause for the attribute.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub validation: Option<ValueAssignment<AnnotatedT>>,

    /// The schema definition for the keys used to identify entries in attributes of type TOSCA map
    /// (or types that derive from map). If not specified, the key_schema defaults to string. For
    /// attributes of type other than map, the key_schema is not allowed.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub key_schema: Option<Variant<AnnotatedT>>,

    /// The schema definition for the entries in attributes of collection types such as list, map,
    /// or types that derive from list or map) If the attribute type is a collection type,
    /// entry_schema is mandatory. For other types, the entry_schema is not allowed.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub entry_schema: Option<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &Self,
        depot: &mut Depot,
        source_id: &SourceID,
        _scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        if let Some(_data_type) = depot
            .get_complete_entity::<DataType<AnnotatedT>, _, _>(
                DATA_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            //let scope = &self.type_name.scope;

            // merge "validation" field ($and ?)
            //
            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<AttributeDefinition<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            default: self.default.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.clone(),
            entry_schema: self.entry_schema.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// To assignment.
    pub fn to_assignment(&self) -> ValueAssignment<AnnotatedT> {
        self.default.as_ref().cloned().unwrap_or_default()
    }
}

impl<'own, AnnotatedT> Definition<DataType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, DataType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.type_name.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            default: self.default.clone(),
            validation: complete_clone(&self.validation, &context.type_.validation),
            key_schema: complete_clone(&self.key_schema, &context.type_.key_schema),
            entry_schema: complete_clone(&self.entry_schema, &context.type_.entry_schema),
            annotations: self.annotations.clone(),
        })
    }

    fn derive<ErrorRecipientT>(
        &self,
        context: DefinitionDeriveContext<'_, Self, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: self.type_name.clone(),
            description: complete_clone(&self.description, &context.parent_definition.description),
            metadata: self.metadata.clone(),
            default: complete_clone(&self.default, &context.parent_definition.default),
            validation: complete_clone(&self.validation, &context.parent_definition.validation),
            key_schema: complete_clone(&self.key_schema, &context.parent_definition.key_schema),
            entry_schema: complete_clone(&self.entry_schema, &context.parent_definition.entry_schema),
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.clone().in_scope(scope.clone());
    }
}

//
// AttributeDefinitions
//

/// Map of [AttributeDefinition].
pub type AttributeDefinitions<AnnotatedT> = BTreeMap<ByteString, AttributeDefinition<AnnotatedT>>;

//
// AttributeDefinitionsExt
//

/// Attribute definitions.
pub trait AttributeDefinitionsExt<AnnotatedT> {
    /// To attribute assignments.
    fn to_assignments(&self) -> ValueAssignments<AnnotatedT>;
}

impl<AnnotatedT> AttributeDefinitionsExt<AnnotatedT> for AttributeDefinitions<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_assignments(&self) -> ValueAssignments<AnnotatedT> {
        self.iter().map(|(name, definition)| (name.clone(), definition.default.clone().unwrap_or_default())).collect()
    }
}
