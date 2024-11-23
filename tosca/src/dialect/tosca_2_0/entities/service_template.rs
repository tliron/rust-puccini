use super::{
    super::{super::super::grammar::*, catalog::OldCatalog},
    group_template::*,
    node_template::*,
    parameter_definition::*,
    policy_template::*,
    relationship_template::*,
    workflow_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, zerocopy::*},
    },
};

//
// ServiceTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// This section defines the service template of a TOSCA file. The main ingredients of the service
/// template are node templates representing components of the application and relationship
/// templates representing links between the components. These elements are defined in the nested
/// node_templates section and the nested relationship_templates sections, respectively.
/// Furthermore, a service template allows for defining input parameters, output parameters,
/// workflows as well as grouping of node templates and associated policies.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description for the service template.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information about this service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of input parameters (i.e., as parameter definitions) for the service
    /// template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitionMap<AnnotatedT>,

    /// An optional map of output parameters (i.e., as parameter definitions) for the service
    /// template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub outputs: ParameterDefinitionMap<AnnotatedT>,

    /// A mandatory map of node template definitions for the service template.
    #[resolve(required)]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub node_templates: NodeTemplates<AnnotatedT>,

    /// An optional map of relationship templates for the service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub relationship_templates: RelationshipTemplates<AnnotatedT>,

    /// An optional map of group definitions whose members are node templates defined within
    /// this same service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub groups: GroupTemplates<AnnotatedT>,

    /// An optional list of policy definitions for the service template.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub policies: PolicyTemplateVector<AnnotatedT>,

    /// An optional declaration that exports the service template as an implementation of a Node
    /// type. This also includes the mappings between the external node type's capabilities and
    /// requirements to existing implementations of those capabilities and requirements on node
    /// templates declared within the service template.
    #[resolve]
    #[depict(option, as(depict))]
    pub substitution_mappings: Option<Variant<AnnotatedT>>,

    /// An optional map of workflow definitions for the service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub workflows: WorkflowDefinitions,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile_to_floria<StoreT, ErrorRecipientT>(
        &self,
        context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
        floria_group_id: &floria::ID,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        StoreT: Clone + floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        match floria::NodeTemplate::new(context.floria_prefix.clone(), context.store) {
            Ok(mut floria_node_template) => {
                floria_node_template.template.group_ids.push(floria_group_id.clone());

                floria_node_template.template.metadata.set_tosca_entity("ServiceTemplate");
                floria_node_template.template.metadata.set_tosca_description(self.description.as_ref());
                floria_node_template.template.metadata.merge_tosca_metadata(&self.metadata);

                let id = floria_node_template.template.id.clone();

                // Node templates
                for (name, node_template) in &context.catalog.node_templates.complete {
                    match node_template.compile_to_floria(context.clone(), name.name.as_ref(), errors) {
                        Ok(id) => {
                            if let Some(id) = id {
                                floria_node_template.contained_node_template_ids.push(id);
                            }
                        }

                        Err(error) => errors.give(error)?,
                    }
                }

                match context.store.add_node_template(floria_node_template) {
                    Ok(_) => return Ok(Some(id)),
                    Err(error) => errors.give(error)?,
                }
            }

            Err(error) => {
                errors.give(error)?;
            }
        }

        Ok(None)
    }
}
