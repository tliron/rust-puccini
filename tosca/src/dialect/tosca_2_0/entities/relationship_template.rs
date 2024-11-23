use {
    super::{
        super::{super::super::grammar::*, catalog::OldCatalog, dialect::*},
        interface_assignment::*,
        relationship_type::*,
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
// RelationshipTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A relationship template specifies the occurrence of a relationship of a given type between
/// nodes in an application or service. A relationship template defines application-specific values
/// for the properties, relationships, or interfaces defined by its relationship type.
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
#[debuggable(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the relationship type on which the relationship template is based.
    #[resolve(required, key = "type")]
    #[debuggable(as(debuggable), tag = tag::span)]
    pub type_name: FullName,

    /// An optional description for the relationship template.
    #[resolve]
    #[debuggable(option, style(string), tag = tag::span)]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property assignments for the relationship template.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute assignments for the relationship template.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_as(display), key_style(name), tag = tag::span)]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the relationship template.
    #[resolve]
    #[debuggable(iter(kv), as(debuggable), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    /// The optional (symbolic) name of another relationship template from which to copy all
    /// keynames and values into this relationship template.
    #[resolve]
    #[debuggable(option, as(debuggable))]
    pub copy: Option<Name>,

    #[resolve(annotations)]
    #[debuggable(skip)]
    annotations: StructAnnotations,

    #[debuggable(skip)]
    completion: Completion,
}

impl<AnnotatedT> Entity for RelationshipTemplate<AnnotatedT>
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
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        if let Some(copy) = &self.copy {
            let Some(copy) = depot.get_complete_entity::<RelationshipTemplate<AnnotatedT>, _, _>(
                RELATIONSHIP_TEMPLATE,
                &copy.clone().into(),
                source_id,
                errors,
            )?
            else {
                return Ok(());
            };

            if self.type_name.is_empty() && !copy.type_name.is_empty() {
                self.type_name = copy.type_name.clone();
            }
            if_none_clone(&mut self.description, &copy.description);
            if self.metadata.is_empty() && !copy.metadata.is_empty() {
                self.metadata = copy.metadata.clone();
            }
            if self.properties.is_empty() && !copy.properties.is_empty() {
                self.properties = copy.properties.clone();
            }
            if self.attributes.is_empty() && !copy.attributes.is_empty() {
                self.attributes = copy.attributes.clone();
            }
            if self.interfaces.is_empty() && !copy.interfaces.is_empty() {
                self.interfaces = copy.interfaces.clone();
            }
        }

        if self.type_name.is_empty() {
            errors.give(MissingRequiredError::new("relationship type name".into(), "type_name".into()))?;
            return Ok(());
        }

        let Some(relationship_type) = depot
            .get_complete_entity_next::<RelationshipType<_>, _, _>(
                RELATIONSHIP_TYPE,
                &self.type_name,
                source_id,
                callstack,
                errors,
            )?
            .cloned()
        else {
            return Ok(());
        };

        let scope = &self.type_name.scope;

        errors_with_field_annotations!(
            errors, self, "properties",
            complete_map(&mut self.properties, &relationship_type.properties, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "attributes",
            complete_map(&mut self.attributes, &relationship_type.attributes, depot, source_id, scope, errors)?;
        );

        errors_with_field_annotations!(
            errors, self, "interfaces",
            complete_map(&mut self.interfaces, &relationship_type.interfaces, depot, source_id, scope, errors)?;
        );

        self.completion = Completion::Complete;
        Ok(())
    }
}

impl<'own, AnnotatedT> Template<RelationshipType<AnnotatedT>, OldCatalog<'own, AnnotatedT>, AnnotatedT>
    for RelationshipTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn get_type_name(&self) -> &FullName {
        &self.type_name
    }

    fn complete<ErrorRecipientT>(
        &self,
        _context: TemplateCompleteContext<'_, RelationshipType<AnnotatedT>, OldCatalog<'_, AnnotatedT>>,
        _errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        // TODO
        Ok(self.clone())
    }
}

//
// RelationshipTemplates
//

/// Map of [RelationshipTemplate].
pub type RelationshipTemplates<AnnotatedT> = BTreeMap<Name, RelationshipTemplate<AnnotatedT>>;
