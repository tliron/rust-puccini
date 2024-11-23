use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog},
        artifact_definition::*,
        value_assignments::*,
    },
    crate::errors_with_field_annotations,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::debug::*,
        std::{error::*, zerocopy::*},
    },
    std::collections::*,
};

//
// ArtifactAssignment
//

/// See [ArtifactDefinition].
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory artifact type for the artifact definition.
    ///
    /// Puccini note: Should not be mandatory for assignments.
    #[resolve(key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// The mandatory URI string (relative or absolute) that can be used to locate the artifact's
    /// file.
    ///
    /// Puccini note: Should not be mandatory for assignments.
    #[debuggable(option, style(string), tag = tag::span)]
    pub file: Option<ByteString>,

    /// The optional name of the repository definition that contains the location of the external
    /// repository that contains the artifact. The artifact is expected to be referenceable by its
    /// file URI within the repository.
    #[resolve]
    #[debuggable(option, as(debuggable), tag = tag::span)]
    pub repository: Option<Name>,

    /// The optional description for the artifact definition.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	The version of this artifact. One use of this artifact_version is to declare the particular
    /// version of this artifact type, in addition to its mime_type (that is declared in the
    /// artifact type definition). Together with the mime_type it may be used to select a
    /// particular artifact processor for this artifact. For example, a Python interpreter that can
    /// interpret Python version 2.7.0.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub artifact_version: Option<ByteString>,

    /// The checksum used to validate the integrity of the artifact.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub checksum: Option<ByteString>,

    /// Algorithm used to calculate the artifact checksum (e.g. MD5, SHA [Ref]). Shall be specified
    /// if checksum is specified for an artifact.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub checksum_algorithm: Option<ByteString>,

    /// The optional map of property assignments associated with the artifact.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub properties: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<ArtifactDefinition<AnnotatedT>> for ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        parent: &ArtifactDefinition<AnnotatedT>,
        depot: &mut Depot,
        source_id: &SourceID,
        scope: &Scope,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        validate_type_name(&self.type_name, &parent.type_name, depot, errors)?;

        if_none_call(&mut self.file, || Some(parent.file.clone()));
        if_none_clone(&mut self.repository, &parent.repository);
        if_none_clone(&mut self.artifact_version, &parent.artifact_version);
        if_none_clone(&mut self.checksum, &parent.checksum);
        if_none_clone(&mut self.checksum_algorithm, &parent.checksum_algorithm);

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &parent.properties, depot, source_id, scope, errors)?;
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ArtifactAssignment<AnnotatedT>> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: &Scope) -> ArtifactAssignment<AnnotatedT> {
        ArtifactAssignment {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            file: Some(self.file.clone()),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.into_scoped(scope),
            ..Default::default()
        }
    }
}

impl<'own, AnnotatedT> Assignment<ArtifactDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_assignment_entity_name() -> &'static str {
        "artifact"
    }

    fn complete<ErrorRecipientT>(
        &self,
        context: AssignmentCompleteContext<'_, ArtifactDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: self.type_name.clone(),
            file: complete(&self.file, || Some(context.definition.file.clone())),
            repository: complete_clone(&self.repository, &context.definition.repository),
            description: complete_clone(&self.description, &context.definition.description),
            metadata: self.metadata.clone(),
            artifact_version: complete_clone(&self.artifact_version, &context.definition.artifact_version),
            checksum: complete_clone(&self.checksum, &context.definition.checksum),
            checksum_algorithm: complete_clone(&self.checksum_algorithm, &context.definition.checksum_algorithm),
            properties: self.properties.complete(&context.definition.properties),
            annotations: self.annotations.clone(),
        })
    }

    fn from_definition<ErrorRecipientT>(
        context: AssignmentFromDefinitionContext<'_, ArtifactDefinition<AnnotatedT>, OldCatalog<'own, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        Ok(Self {
            type_name: context.definition.type_name.clone(),
            file: Some(context.definition.file.clone()),
            repository: context.definition.repository.clone(),
            description: context.definition.description.clone(),
            metadata: context.definition.metadata.clone(),
            artifact_version: context.definition.artifact_version.clone(),
            checksum: context.definition.checksum.clone(),
            checksum_algorithm: context.definition.checksum_algorithm.clone(),
            properties: context.definition.properties.clone(),
            annotations: Default::default(),
        })
    }
}

//
// ArtifactAssignments
//

/// Map of [ArtifactAssignment].
pub type ArtifactAssignments<AnnotatedT> = BTreeMap<ByteString, ArtifactAssignment<AnnotatedT>>;
