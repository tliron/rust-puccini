use super::{super::super::super::grammar::*, relationship_assignment::*, requirement_definition::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    smart_default::*,
};

//
// RequirementAssignment
//

/// A requirement assignment is used by node template authors to provide assignments for the
/// corresponding requirement definition in the node template's node type. This includes specifying
/// target nodes, either by providing symbolic names of target nodes or by providing selection
/// criteria for TOSCA orchestrators to find candidate nodes that can be used to fulfill the
/// requirement. In addition, requirement assignments must uniquely identify the specific target
/// capability in the target node for the requirement. Requirement assignments must also assign
/// values to properties and attributes defined in the relationship definition that is part of the
/// requirement definition, and provide values for the input parameters defined by the relationship
/// definition's interfaces.
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RequirementAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional keyname used to identify the target node of the requirement:
    /// - This can either be the symbolic name of a node template, where the TOSCA processor will
    ///   select a node representation created from that template. If the count of the node
    ///   template is 1 then the potential target is unique, otherwise the processor can select
    ///   from several node representations.
    /// - It can also be a 2-entry list, where the first entry is a string denoting the symbolic
    ///   name of a node template, while the second entry is an index, thus uniquely identifying
    ///   the node representation when multiple representations are created from the same node
    ///   template. The index is a non-negative integer, with 0 being the first index. Note that
    ///   functions like $node_index or $relationship_index may be used to match the target index
    ///   with the source/relationship index. More information on multiplicity and node and
    ///   relationship indexes can be found in Chapter 14.
    /// - Finally, it can also be the name of a node type that the TOSCA processor will use to
    ///   select a type-compatible target node to fulfill the requirement.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub node: Option<IndexedFullName>,

    /// The optional keyname used to identify the target capability of the requirement. This can
    /// either be the name of a capability defined within a target node or the name of a target
    /// capability type that the TOSCA orchestrator will use to select a type-compatible target
    /// node to fulfill the requirement at runtime.
    #[resolve]
    #[depict(option, as(depict))]
    pub capability: Option<FullName>,

    /// The conditional keyname used to provide values for the relationship definition in the
    /// corresponding requirement definition. This keyname can also be overloaded to define a
    /// symbolic name that references a relationship template defined elsewhere in the service
    /// template.
    #[resolve]
    #[depict(option, as(depict))]
    pub relationship: Option<RelationshipAssignment<AnnotatedT>>,

    /// The optional keyname that allows the inclusion of an allocation block. The allocation
    /// block contains a map of property assignments that semantically represent allocations from
    /// the property with the same name in the target capability. The allocation acts as a capacity
    /// filter for the target capability in the target node. When the requirement is resolved, a
    /// capability in a node is a valid target for the requirement relationship if for each
    /// property of the target capability, the sum of all existing allocations plus the current
    /// allocation is less than or equal to the property value.
    #[resolve]
    #[depict(option, as(depict))]
    pub allocation: Option<Variant<AnnotatedT>>,

    /// An optional keyname that sets the cardinality of the requirement assignment, that is how
    /// many relationships must be established from this requirement assignment. If not defined,
    /// the default count for an assignment is 1. Note that there can be multiple requirement
    /// assignments for a requirement with a specific symbolic name. The sum of all count values
    /// of assignments for a requirement with a specific symbolic name must be within the
    /// count_range defined in the requirement definition. Moreover, the sum of all count values
    /// of non-optional assignments for a requirement with a specific symbolic name must also be
    /// within the count_range defined in the requirement definition.
    #[default(1)]
    #[resolve]
    #[depict(style(number))]
    pub count: u64,

    /// The optional filter definition that TOSCA orchestrators will use to select a
    /// type-compatible target node that can fulfill the requirement at runtime.
    #[resolve]
    #[depict(option, as(depict))]
    pub node_filter: Option<Variant<AnnotatedT>>,

    /// An optional list of directive values to provide processing instructions to
    /// orchestrators and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    /// Describes if the fulfillment of this requirement assignment is optional (true) or not
    /// (false). If not specified, the requirement assignment must be fulfilled, i.e. the default
    /// value is false. Note also, that non-optional requirements have precedence, thus during a
    /// service deployment, the optional requirements for all nodes should be resolved only after
    /// the non-optional requirements for all nodes have been resolved.
    #[resolve]
    #[depict(style(symbol))]
    pub optional: bool,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> RequirementAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        _edge_template: &mut floria::EdgeTemplate,
        _errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        // TODO
        Ok(())
    }

    // /// Compile to Floria.
    // pub fn compile_to_floria<StoreT, ErrorRecipientT>(
    //     &self,
    //     context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
    //     requirement_name: &str,
    //     node_template_id: floria::ID,
    //     node_type: &NodeType<AnnotatedT>,
    //     errors: &mut ErrorRecipientT,
    // ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    // where
    //     StoreT: floria::Store,
    //     ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    // {
    //     let finder = match self.new_finder(context.catalog, context.index) {
    //         Ok(finder) => finder,
    //         Err(error) => {
    //             errors.give(error)?;
    //             return Ok(None);
    //         }
    //     };

    //     let mut floria_namespace = context.floria_directory.clone();
    //     floria_namespace.push(requirement_name.into());

    //     let mut floria_relationship_template = match floria::RelationshipTemplate::new(
    //         floria_namespace.clone(),
    //         node_template_id,
    //         floria::NodeSelector::new_finder(finder),
    //         context.store,
    //     ) {
    //         Ok(floria_relationship_template) => floria_relationship_template,
    //         Err(error) => {
    //             errors.give(error)?;
    //             return Ok(None);
    //         }
    //     };

    //     floria_relationship_template.template.metadata.set_tosca_entity("RequirementAssignment");
    //     floria_relationship_template.template.metadata.set_tosca_directives(&self.directives);

    //     if let Some(relationship_assignment) = &self.relationship {
    //         match node_type.requirements.get_first(&requirement_name.into()) {
    //             Some(requirement_definition) => {
    //                 // Properties and attributes
    //                 floria_relationship_template.template.property_templates =
    //                     relationship_assignment.properties.compile_to_floria_as_properties(
    //                         &requirement_definition.relationship.properties,
    //                         context.catalog,
    //                         context.index,
    //                         errors,
    //                     )?;
    //                 floria_relationship_template.template.property_templates.extend(
    //                     relationship_assignment.attributes.compile_to_floria_as_attributes(
    //                         &requirement_definition.relationship.attributes,
    //                         context.catalog,
    //                         context.index,
    //                         errors,
    //                     )?,
    //                 );

    //                 context.catalog.relationship_types.add_floria_group_ids(
    //                     &mut floria_relationship_template.template.group_ids,
    //                     &"relationship".into(),
    //                     context.index.index.get(&requirement_definition.relationship.type_name).unwrap(),
    //                 );
    //             }

    //             None => tracing::warn!("requirement definition not found: {}", requirement_name),
    //         }
    //     }

    //     let id = floria_relationship_template.template.id.clone();
    //     if let Err(error) = context.store.add_relationship_template(floria_relationship_template) {
    //         errors.give(error)?;
    //         return Ok(None);
    //     }

    //     Ok(Some(id))
    // }

    // fn new_finder(
    //     &self,
    //     catalog: &OldCatalog<'_, AnnotatedT>,
    //     index: &Index,
    // ) -> Result<floria::Call, ToscaError<AnnotatedT>> {
    //     let mut finder = floria::Call::new(get_dispatch_name("select_capability"), Default::default());

    //     let mut argument = Map::default();

    //     if let Some(capability) = self.get_capability(catalog)? {
    //         match capability {
    //             RequirementAssignmentCapability::TypeName(type_name) => {
    //                 argument.into_insert("capability_type_name", type_name.to_string());
    //             }

    //             RequirementAssignmentCapability::Name(name) => {
    //                 argument.into_insert("capability_name", name.to_string());
    //             }
    //         }
    //     }

    //     if let Some(node) = self.get_node(catalog, index)? {
    //         match node {
    //             RequirementAssignmentNode::TypeName(type_name) => {
    //                 argument.into_insert("node_type_name", type_name.to_string());
    //             }

    //             RequirementAssignmentNode::Template((template_name, index)) => {
    //                 argument.into_insert("node_template_name", template_name.to_string());
    //                 argument.into_insert("node_template_index", index);
    //             }
    //         }
    //     }

    //     if !argument.inner.is_empty() {
    //         finder.arguments.push(floria::Expression::Literal(argument.into()));
    //     }

    //     Ok(finder)
    // }

    // fn get_capability(
    //     &self,
    //     _catalog: &OldCatalog<'_, AnnotatedT>,
    // ) -> Result<Option<RequirementAssignmentCapability>, ToscaError<AnnotatedT>> {
    //     // TODO: capability type name or capability name
    //     Ok(self.capability.as_ref().map(|name| RequirementAssignmentCapability::Name(name.name.clone())))
    // }

    // fn get_node(
    //     &self,
    //     catalog: &OldCatalog<'_, AnnotatedT>,
    //     index: &Index,
    // ) -> Result<Option<RequirementAssignmentNode>, ToscaError<AnnotatedT>> {
    //     match &self.node {
    //         Some(node) => {
    //             match node.index {
    //                 // node type name or node template name
    //                 // TODO
    //                 None => match index.index.get(&node.full_name) {
    //                     Some(node_id) => match catalog.node_types.has_id(node_id) {
    //                         true => Ok(Some(RequirementAssignmentNode::TypeName(node.full_name.clone()))),

    //                         false => Err(UnknownTypeError::new(
    //                             node.full_name.to_string(),
    //                             "RequirementAssignment.get_node".into(),
    //                         )
    //                         .into()),
    //                     },

    //                     None => Ok(None),
    //                 },

    //                 // node template name and index
    //                 Some(index) => Ok(Some(RequirementAssignmentNode::Template((node.full_name.name.clone(), index)))),
    //             }
    //         }

    //         None => Ok(None),
    //     }
    // }
}

impl<AnnotatedT> Subentity<RequirementDefinition<AnnotatedT>> for RequirementAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parent: Option<(&RequirementDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        // TODO: validate node (template) adheres to parent's node (type)

        // TODO: validate capability adheres to parent's capability type

        if let Some((parent, scope)) = parent {
            if_none_else!(relationship, self, parent, Some(parent.relationship.clone().convert_into_scope(scope)));
        }

        if let Some(relationship) = &mut self.relationship {
            relationship.complete(
                name,
                parent.map(|(parent, scope)| (&parent.relationship, scope)),
                catalog,
                source_id,
                errors,
            )?;
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<RequirementAssignment<AnnotatedT>> for RequirementDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> RequirementAssignment<AnnotatedT> {
        RequirementAssignment {
            node: self.node.clone().map(|full_name| IndexedFullName::new(full_name, None)),
            capability: Some(self.capability.clone()),
            relationship: Some(self.relationship.convert_into_scope(scope)),
            node_filter: self.node_filter.clone(),
            annotations: clone_struct_annotations(
                &self.annotations,
                &["node", "capability", "relationship", "node_filter"],
            ),
            ..Default::default()
        }
    }
}

//
// RequirementAssignmentCapability
//

/// Requirement assignment capability.
pub enum RequirementAssignmentCapability {
    /// Capability type name.
    TypeName(FullName),

    /// Capability name.
    Name(Name),
}

/// Requirement assignment node.
pub enum RequirementAssignmentNode {
    /// Node type name.
    TypeName(FullName),

    /// Node template.
    Template((Name, usize)),
}

//
// RequirementAssignments
//

/// [TaggedValues] of [RequirementAssignment].
pub type RequirementAssignments<AnnotatedT> = TaggedValues<ByteString, RequirementAssignment<AnnotatedT>>;
