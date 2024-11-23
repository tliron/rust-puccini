use super::{
    super::{super::super::grammar::*, dialect::*},
    artifact_assignment::*,
    capability_assignment::*,
    interface_assignment::*,
    node_type::*,
    requirement_assignment::*,
    value_assignment::*,
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
// NodeTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A node template specifies the occurrence of one or more instances of a component of a given type
/// in an application or service. A node template defines application-specific values for the
/// properties, relationships, or interfaces defined by its node type.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the node type on which the node template is based.
    #[resolve(key = "type")]
    #[depict(as(display), style(name))]
    pub type_name: FullName,

    /// An optional description for the node template.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional list of directive values to provide processing instructions to orchestrators
    /// and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    /// An optional map of property value assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute value assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of requirement assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub requirements: RequirementAssignments<AnnotatedT>,

    /// An optional map of capability assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub capabilities: CapabilityAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    /// An optional map of artifact definitions for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub artifacts: ArtifactAssignments<AnnotatedT>,

    /// An optional keyname that specifies how many node representations must be created from
    /// this node template. If not defined, the assumed count value is 1.
    #[resolve]
    #[depict(option, style(number))]
    pub count: Option<u64>,

    /// The optional filter definition that TOSCA orchestrators will use to select an already
    /// existing node if this node template is marked with the "select" directive.
    #[resolve]
    #[depict(option, as(depict))]
    pub node_filter: Option<Variant<AnnotatedT>>,

    /// The optional (symbolic) name of another node template from which to copy all keynames and
    /// values into this node template.
    #[resolve]
    #[depict(option, as(depict))]
    pub copy: Option<Name>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion: Completion,
}

impl<AnnotatedT> NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        vertex_template.template.class_ids.add_tosca_type(&self.type_name, directory, store.clone(), errors)?;

        vertex_template.template.metadata.set_tosca_entity(NODE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.merge_tosca_metadata(&self.metadata);
        vertex_template.template.metadata.set_tosca_directives(&self.directives);

        for (name, value_assignment) in &self.properties {
            vertex_template
                .template
                .property_templates
                .insert(name.clone(), value_assignment.compile(PROPERTY_NAME, true, directory, store.clone(), errors)?);
        }

        // TODO: name collisions?

        for (name, value_assignment) in &self.attributes {
            vertex_template.template.property_templates.insert(
                name.clone(),
                value_assignment.compile(ATTRIBUTE_NAME, false, directory, store.clone(), errors)?,
            );
        }

        Ok(())
    }

    // /// Compile to Floria.
    // pub fn compile_to_floria<StoreT, ErrorRecipientT>(
    //     &self,
    //     context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
    //     node_template_name: &str,
    //     errors: &mut ErrorRecipientT,
    // ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    // where
    //     StoreT: Clone + floria::Store,
    //     ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    // {
    //     let mut floria_node_template =
    //         floria::NodeTemplate::new_for(context.floria_directory.clone(), node_template_name.into(), None);

    //     floria_node_template.template.metadata.set_tosca_entity("NodeTemplate");
    //     floria_node_template.template.metadata.set_tosca_description(self.description.as_ref());
    //     floria_node_template.template.metadata.set_tosca_directives(&self.directives);
    //     floria_node_template.template.metadata.merge_tosca_metadata(&self.metadata);

    //     let node_type_id = context.index.index.get(&self.type_name).unwrap();

    //     context.catalog.node_types.add_floria_group_ids(
    //         &mut floria_node_template.template.group_ids,
    //         &"node".into(),
    //         node_type_id,
    //     );

    //     let id = floria_node_template.template.id.clone();

    //     match context.catalog.node_types.get_complete(node_type_id) {
    //         Some(node_type) => {
    //             // Properties
    //             floria_node_template.template.property_templates = self.properties.compile_to_floria_as_properties(
    //                 &node_type.properties,
    //                 context.catalog,
    //                 context.index,
    //                 errors,
    //             )?;

    //             // Attributes
    //             floria_node_template.template.property_templates.extend(
    //                 self.attributes.compile_to_floria_as_attributes(
    //                     &node_type.attributes,
    //                     context.catalog,
    //                     context.index,
    //                     errors,
    //                 )?,
    //             );

    //             // Capability assignments
    //             for (capability_name, capability_assignment) in &self.capabilities {
    //                 match capability_assignment.compile_to_floria(
    //                     context.clone(),
    //                     capability_name,
    //                     id.clone(),
    //                     node_type,
    //                     errors,
    //                 ) {
    //                     Ok(id) => {
    //                         if let Some(id) = id {
    //                             floria_node_template.contained_node_template_ids.push(id);
    //                         }
    //                     }

    //                     Err(error) => errors.give(error)?,
    //                 }
    //             }

    //             // Requirement assignments
    //             for (requirement_name, requirement_assignment) in &self.requirements {
    //                 match requirement_assignment.compile_to_floria(
    //                     context.clone(),
    //                     requirement_name,
    //                     id.clone(),
    //                     node_type,
    //                     errors,
    //                 ) {
    //                     Ok(id) => {
    //                         if let Some(id) = id {
    //                             floria_node_template.outgoing_relationship_template_ids.push(id);
    //                         }
    //                     }

    //                     Err(error) => errors.give(error)?,
    //                 }
    //             }
    //         }

    //         None => tracing::warn!("node type not found: {}", self.type_name),
    //     }

    //     if let Err(error) = context.store.add_node_template(floria_node_template) {
    //         errors.give(error)?;
    //     }

    //     Ok(Some(id))
    // }
}

impl<AnnotatedT> Entity for NodeTemplate<AnnotatedT>
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
        _callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        let errors = &mut errors.to_error_recipient();

        if let Some(copy) = &self.copy {
            let Some(copy) = catalog.get_complete_entity::<NodeTemplate<AnnotatedT>, _, _>(
                NODE_TEMPLATE,
                &copy.clone().into(),
                source_id,
                errors,
            )?
            else {
                return Ok(());
            };

            if_empty_clone!(self.type_name, copy.type_name, self.annotations, copy.annotations, "type_name");

            if_none_clone(
                &mut self.description,
                &copy.description,
                &mut self.annotations,
                &copy.annotations,
                "description",
            );

            if_empty_clone!(self.metadata, copy.metadata, self.annotations, copy.annotations, "metadata");
            if_empty_clone!(self.directives, copy.directives, self.annotations, copy.annotations, "directives");
            if_empty_clone!(self.properties, copy.properties, self.annotations, copy.annotations, "properties");
            if_empty_clone!(self.attributes, copy.attributes, self.annotations, copy.annotations, "attributes");
            if_empty_clone!(self.requirements, copy.requirements, self.annotations, copy.annotations, "requirements");
            if_empty_clone!(self.capabilities, copy.capabilities, self.annotations, copy.annotations, "capabilities");
            if_empty_clone!(self.interfaces, copy.interfaces, self.annotations, copy.annotations, "interfaces");
            if_empty_clone!(self.artifacts, copy.artifacts, self.annotations, copy.annotations, "artifacts");
            if_none_clone(&mut self.count, &copy.count, &mut self.annotations, &copy.annotations, "count");

            if_none_clone(
                &mut self.node_filter,
                &copy.node_filter,
                &mut self.annotations,
                &copy.annotations,
                "node_filter",
            );
        }

        if self.type_name.is_empty() {
            errors.give(MissingRequiredError::new("node type name".into(), "type_name".into()))?;
            return Ok(());
        }

        let node_type = get_complete_entity!(NODE_TYPE, NodeType, self, type_name, catalog, source_id, errors);

        complete_map!(self, properties, node_type, catalog, source_id, errors);
        complete_map!(self, attributes, node_type, catalog, source_id, errors);
        complete_tagged_values!(self, requirements, node_type, catalog, source_id, errors);
        complete_map!(self, capabilities, node_type, catalog, source_id, errors);
        complete_map!(self, interfaces, node_type, catalog, source_id, errors);
        complete_map!(self, artifacts, node_type, catalog, source_id, errors);

        self.completion = Completion::Complete;
        Ok(())
    }
}

//
// NodeTemplates
//

/// Map of [NodeTemplate].
pub type NodeTemplates<AnnotatedT> = BTreeMap<Name, NodeTemplate<AnnotatedT>>;
