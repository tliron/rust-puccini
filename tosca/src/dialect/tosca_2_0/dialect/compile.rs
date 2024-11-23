use super::{
    super::{super::super::grammar::*, entities::*},
    dialect::*,
    entity_kind::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl super::Dialect {
    /// Compile service template to Floria.
    pub fn compile_service_template<ErrorRecipientT, AnnotatedT>(
        &self,
        directory: &floria::Directory,
        store: &floria::StoreRef,
        source_id: &SourceID,
        catalog: &Catalog,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        tracing::info!(directory = directory.to_string(), source = source_id.to_string(), "compiling service template");

        let source = unwrap_or_give_and_return!(catalog.get_source(source_id), errors, Ok(None));

        let mut id = floria::ID::new(floria::Kind::VertexTemplate, directory.clone());
        unwrap_or_give_and_return!(store.create_id(&mut id), errors, Ok(None));
        let mut floria_service_template = floria::VertexTemplate::new_with(id, None);

        let service_template_kind_name = self.implementation.entity_kinds.represent(SERVICE_TEMPLATE);

        match source.get_entity::<ServiceTemplate<AnnotatedT>, _>(
            SERVICE_TEMPLATE,
            &service_template_kind_name,
            &Default::default(),
        ) {
            Ok(service_template) => service_template.compile(&mut floria_service_template, errors)?,
            Err(ToscaError::Undeclared(_)) => return Ok(None),
            Err(error) => errors.give(error)?,
        }

        // Types
        let type_entity_compiler = TypeEntityCompiler::new(directory, store, source_id, self, catalog);
        for (entity_kind, full_name, entity_source_id) in source.namespace() {
            match entity_kind {
                ARTIFACT_TYPE => type_entity_compiler.compile::<ArtifactType<AnnotatedT>, _, _>(
                    ARTIFACT_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                CAPABILITY_TYPE => type_entity_compiler.compile::<CapabilityType<AnnotatedT>, _, _>(
                    CAPABILITY_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                DATA_TYPE => type_entity_compiler.compile::<DataType<AnnotatedT>, _, _>(
                    DATA_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                GROUP_TYPE => type_entity_compiler.compile::<GroupType<AnnotatedT>, _, _>(
                    GROUP_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                INTERFACE_TYPE => type_entity_compiler.compile::<InterfaceType<AnnotatedT>, _, _>(
                    INTERFACE_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                NODE_TYPE => type_entity_compiler.compile::<NodeType<AnnotatedT>, _, _>(
                    NODE_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                POLICY_TYPE => type_entity_compiler.compile::<PolicyType<AnnotatedT>, _, _>(
                    POLICY_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                RELATIONSHIP_TYPE => type_entity_compiler.compile::<RelationshipType<AnnotatedT>, _, _>(
                    RELATIONSHIP_TYPE,
                    &full_name,
                    &entity_source_id,
                    DIALECT_ID,
                    errors,
                )?,

                _ => {}
            }
        }

        let floria_service_template_id = floria_service_template.template.id.clone();

        let group_template_kind_name = self.implementation.entity_kinds.represent(GROUP_TEMPLATE);
        let node_template_kind_name = self.implementation.entity_kinds.represent(NODE_TEMPLATE);
        let policy_template_kind_name = self.implementation.entity_kinds.represent(POLICY_TEMPLATE);

        for (entity_kind, name) in source.entity_names() {
            match entity_kind {
                GROUP_TEMPLATE => {
                    tracing::debug!(
                        source = source_id.to_string(),
                        name = name.to_string(),
                        type = GROUP_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.get_entity::<GroupTemplate<AnnotatedT>, _>(
                        GROUP_TEMPLATE,
                        &group_template_kind_name,
                        &name,
                    ) {
                        Ok(_group_template) => {
                            let floria_group_template = floria::Class::new_for(directory.clone(), name.into());

                            // TODO

                            unwrap_or_give_and_return!(store.add_class(floria_group_template), errors, Ok(None));
                        }

                        Err(error) => errors.give(error)?,
                    }
                }

                NODE_TEMPLATE => {
                    tracing::debug!(
                        source = source_id.to_string(),
                        name = name.to_string(),
                        type = NODE_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.get_entity::<NodeTemplate<AnnotatedT>, _>(
                        NODE_TEMPLATE,
                        &node_template_kind_name,
                        &name,
                    ) {
                        Ok(node_template) => {
                            let mut floria_node_template = floria::VertexTemplate::new_for(
                                directory.clone(),
                                name.into(),
                                Some(floria_service_template_id.clone()),
                            );

                            node_template.compile(&mut floria_node_template, directory, store.clone(), errors)?;

                            // Capabilities
                            for (name, capability) in &node_template.capabilities {
                                let mut floria_capability = floria::VertexTemplate::new_for(
                                    directory.clone(),
                                    name.clone(),
                                    Some(floria_node_template.template.id.clone()),
                                );

                                capability.compile(&mut floria_capability, directory, store.clone(), errors)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_capability.template.id.clone());

                                unwrap_or_give_and_return!(
                                    store.add_vertex_template(floria_capability),
                                    errors,
                                    Ok(None)
                                );
                            }

                            // Requirements
                            for (name, requirement) in &node_template.requirements {
                                // TODO
                                let node_selector =
                                    floria::VertexSelector::new_vertex(floria_node_template.template.id.clone());

                                let mut floria_requirement = floria::EdgeTemplate::new_for(
                                    directory.clone(),
                                    name.clone(),
                                    floria_node_template.template.id.clone(),
                                    node_selector,
                                );

                                requirement.compile(&mut floria_requirement, errors)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_requirement.template.id.clone());

                                unwrap_or_give_and_return!(
                                    store.add_edge_template(floria_requirement),
                                    errors,
                                    Ok(None)
                                );
                            }

                            // Artifacts
                            for (name, artifact) in &node_template.artifacts {
                                let mut floria_artifact = floria::VertexTemplate::new_for(
                                    directory.clone(),
                                    name.clone(),
                                    Some(floria_node_template.template.id.clone()),
                                );

                                artifact.compile(&mut floria_artifact, directory, store.clone(), errors)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_artifact.template.id.clone());

                                unwrap_or_give_and_return!(
                                    store.add_vertex_template(floria_artifact),
                                    errors,
                                    Ok(None)
                                );
                            }

                            floria_service_template
                                .contained_vertex_template_ids
                                .push(floria_node_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                store.add_vertex_template(floria_node_template),
                                errors,
                                Ok(None)
                            );
                        }

                        Err(error) => errors.give(error)?,
                    }
                }

                POLICY_TEMPLATE => {
                    tracing::debug!(
                        source = source_id.to_string(),
                        name = name.to_string(),
                        type = POLICY_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.get_entity::<PolicyTemplate<WithAnnotations>, _>(
                        POLICY_TEMPLATE,
                        &policy_template_kind_name,
                        &name,
                    ) {
                        Ok(_policy_template) => {
                            let floria_policy_template = floria::VertexTemplate::new_for(
                                directory.clone(),
                                name.into(),
                                Some(floria_service_template_id.clone()),
                            );

                            // TODO

                            floria_service_template
                                .contained_vertex_template_ids
                                .push(floria_policy_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                store.add_vertex_template(floria_policy_template),
                                errors,
                                Ok(None)
                            );
                        }

                        Err(error) => errors.give(error)?,
                    }
                }

                _ => {}
            }
        }

        unwrap_or_give_and_return!(store.add_vertex_template(floria_service_template), errors, Ok(None));

        Ok(Some(floria_service_template_id.clone()))
    }
}
