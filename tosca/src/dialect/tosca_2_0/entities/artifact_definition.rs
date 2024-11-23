use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        artifact_type::*,
        value_assignments::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
    std::collections::*,
};

//
// ArtifactDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An artifact definition defines a named, typed file that can be associated with a node type or
/// node template and used by a TOSCA Orchestrator to facilitate deployment and implementation of
/// artifact operations.
///
/// Puccini note: Though this is called a "definition" in the TOSCA spec, it is actually used both
/// as a definition and as a template. See
/// [ArtifactAssignment](super::artifact_assignment::ArtifactAssignment).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory artifact type for the artifact definition.
    #[resolve(required, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The mandatory URI string (relative or absolute) that can be used to locate the artifact's
    /// file.
    #[resolve(required)]
    #[depict(style(string))]
    pub file: ByteString,

    /// The optional name of the repository definition that contains the location of the external
    /// repository that contains the artifact. The artifact is expected to be referenceable by its
    /// file URI within the repository.
    #[resolve]
    #[depict(option, as(depict))]
    pub repository: Option<Name>,

    /// The optional description for the artifact definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	The version of this artifact. One use of this artifact_version is to declare the particular
    /// version of this artifact type, in addition to its mime_type (that is declared in the
    /// artifact type definition). Together with the mime_type it may be used to select a
    /// particular artifact processor for this artifact. For example, a Python interpreter that can
    /// interpret Python version 2.7.0.
    #[resolve]
    #[depict(option, style(string))]
    pub artifact_version: Option<ByteString>,

    /// The checksum used to validate the integrity of the artifact.
    #[resolve]
    #[depict(option, style(string))]
    pub checksum: Option<ByteString>,

    /// Algorithm used to calculate the artifact checksum (e.g. MD5, SHA [Ref]). Shall be specified
    /// if checksum is specified for an artifact.
    #[resolve]
    #[depict(option, style(string))]
    pub checksum_algorithm: Option<ByteString>,

    /// The optional map of property assignments associated with the artifact.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<ArtifactDefinition<AnnotatedT>> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &Self,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        if let Some(artifact_type) = depot
            .get_complete_entity::<ArtifactType<_>, _, _>(
                ARTIFACT_TYPE,
                &self.type_name,
                source_id,
                &mut errors.with_field_annotations(self, "type_name"),
            )?
            .cloned()
        {
            let scope = &self.type_name.scope;

            errors_with_field_annotations!(
                errors, self, "properties",
                complete_map(&mut self.properties, &artifact_type.properties, depot, source_id, scope, errors)?;
            );
        }

        if_none_clone(
            &mut self.repository,
            &parent.repository,
            &mut self.annotations,
            &parent.annotations,
            "repository",
        );

        if_none_clone(
            &mut self.artifact_version,
            &parent.artifact_version,
            &mut self.annotations,
            &parent.annotations,
            "artifact_version",
        );

        if_none_clone(&mut self.checksum, &parent.checksum, &mut self.annotations, &parent.annotations, "checksum");

        if_none_clone(
            &mut self.checksum_algorithm,
            &parent.checksum_algorithm,
            &mut self.annotations,
            &parent.annotations,
            "checksum_algorithm",
        );

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ArtifactDefinition<AnnotatedT>> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            file: self.file.clone(),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.into_scoped(scope),
            annotations: self.annotations.clone(),
        }
    }
}

impl<'own, AnnotatedT> Definition<ArtifactType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

    fn entype<ErrorRecipientT>(
        &self,
        context: DefinitionEntypeContext<'_, ArtifactType<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.type_name.clone(),
            file: self.file.clone(),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.complete_as_properties(&context.type_.properties, errors)?,
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
            file: self.file.clone(),
            repository: complete_clone(&self.repository, &context.parent_definition.repository),
            description: complete_clone(&self.description, &context.parent_definition.description),
            metadata: self.metadata.clone(),
            artifact_version: complete_clone(&self.artifact_version, &context.parent_definition.artifact_version),
            checksum: complete_clone(&self.checksum, &context.parent_definition.checksum),
            checksum_algorithm: complete_clone(&self.checksum_algorithm, &context.parent_definition.checksum_algorithm),
            properties: self.properties.complete(&context.parent_definition.properties),
            annotations: self.annotations.clone(),
        })
    }

    fn to_scope(&mut self, scope: &Scope) {
        self.type_name = self.type_name.clone().in_scope(scope.clone());
    }
}

//
// ArtifactDefinitions
//

/// Map of [ArtifactDefinition].
pub type ArtifactDefinitions<AnnotatedT> = BTreeMap<ByteString, ArtifactDefinition<AnnotatedT>>;
