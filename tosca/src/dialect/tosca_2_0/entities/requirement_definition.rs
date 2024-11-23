use super::{super::super::super::grammar::*, relationship_definition::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
};

//
// RequirementDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The requirement definition describes a requirement of a TOSCA node that needs to be fulfilled by
/// a matching capability declared by another TOSCA node. A requirement is defined as part of a node
/// type definition and may be refined during node type derivation.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RequirementDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description of the requirement definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// The mandatory keyname used to define the relationship created as a result of fulfilling
    /// the requirement.
    #[resolve(required)]
    #[depict(as(depict))]
    pub relationship: RelationshipDefinition<AnnotatedT>,

    /// The optional keyname used to provide the name of a valid node type that contains the
    /// capability definition that can be used to fulfill the requirement.
    #[resolve]
    #[depict(option, as(depict))]
    pub node: Option<FullName>,

    /// The mandatory keyname used to specify the capability type for capabilities that can be
    /// used to fulfill this requirement. If the requirement definition defines a target node
    /// type, the capability keyname can also be used instead to specify the symbolic name of a
    /// capability defined by that target node type.
    #[resolve(required)]
    #[depict(as(depict))]
    pub capability: FullName,

    /// The optional filter definition that TOSCA orchestrators will use to select a
    /// type-compatible target node that can fulfill this requirement at runtime.
    #[resolve]
    #[depict(option, as(depict))]
    pub node_filter: Option<Variant<AnnotatedT>>,

    /// The optional minimum required and maximum allowed number of relationships created by the
    /// requirement. If this key is not specified, the implied default of [ 0, UNBOUNDED ] will be
    /// used. Note: the value UNBOUNDED is also supported to represent any positive integer.
    #[resolve]
    #[depict(as(depict))]
    pub count_range: Range,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RequirementDefinition<AnnotatedT>> for RequirementDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parent: Option<(&Self, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        self.relationship.complete(
            name,
            parent.map(|(parent, scope)| (&parent.relationship, scope)),
            catalog,
            source_id,
            errors.clone(),
        )?;

        let errors = &mut errors.to_error_recipient();

        if let Some((parent, scope)) = parent {
            if_none_else!(node, self, parent, parent.node.clone().map(|node| node.in_scope(scope.clone())));
            if_none_clone!(node_filter, self, parent);

            validate_type_name(&self.capability, &parent.capability, catalog, errors)?;
        }

        // TODO: validate that count range is within parent count range?

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<RequirementDefinition<AnnotatedT>> for RequirementDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            relationship: self.relationship.convert_into_scope(scope),
            node: self.node.clone().map(|node| node.in_scope(scope.clone())),
            capability: self.capability.clone().in_scope(scope.clone()),
            node_filter: self.node_filter.clone(),
            count_range: self.count_range.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

//
// RequirementDefinitions
//

/// [TaggedValues] of [RequirementDefinition].
pub type RequirementDefinitions<AnnotatedT> = TaggedValues<ByteString, RequirementDefinition<AnnotatedT>>;
