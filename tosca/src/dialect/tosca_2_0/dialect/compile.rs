use {
    super::{
        super::{super::super::grammar, entities::*},
        dialect::*,
        entity_kind::*,
    },
    crate::compile_type_2_0,
};

use {
    compris::annotate::*,
    kutil::{std::error::*, unwrap_or_give_and_return},
};

impl Dialect {
    /// To Floria.
    pub fn compile_service_template<AnnotatedT, ErrorRecipientT>(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: floria::StoreRef,
        source_id: &grammar::SourceID,
        depot: &grammar::Depot,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, grammar::ToscaError<AnnotatedT>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<grammar::ToscaError<AnnotatedT>>,
    {
        tracing::info!(source = source_id.to_string(), "compiling service template");

        let source = unwrap_or_give_and_return!(depot.get_source(source_id), errors, Ok(None));

        for (entity_kind, full_name, _source_id) in source.namespace() {
            let mut floria_type = floria::Group::new_for(floria_prefix.clone(), full_name.to_string().into());

            compile_type_2_0!(
                ARTIFACT_TYPE,
                "ArtifactType",
                ArtifactType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                CAPABILITY_TYPE,
                "CapabilityType",
                CapabilityType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                DATA_TYPE,
                "DataType",
                DataType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                GROUP_TYPE,
                "GroupType",
                GroupType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                INTERFACE_TYPE,
                "InterfaceType",
                InterfaceType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                NODE_TYPE,
                "NodeType",
                NodeType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                POLICY_TYPE,
                "PolicyType",
                PolicyType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );

            compile_type_2_0!(
                RELATIONSHIP_TYPE,
                "RelationshipType",
                RelationshipType,
                entity_kind,
                full_name,
                floria_store,
                floria_type,
                depot,
                source_id,
                errors,
            );
        }

        let mut id = floria::ID::new(floria::Kind::NodeTemplate, floria_prefix.clone());
        unwrap_or_give_and_return!(floria_store.create_id(&mut id), errors, Ok(None));
        let mut floria_service_template = floria::NodeTemplate::new_with(id, None);

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
                        type = "GroupTemplate",
                        "compiling"
                    );

                    match source.get_entity::<GroupTemplate<AnnotatedT>, _>(
                        GROUP_TEMPLATE,
                        &group_template_kind_name,
                        &name,
                    ) {
                        Ok(_group_template) => {
                            let floria_group_template = floria::Group::new_for(floria_prefix.clone(), name.into());

                            // TODO

                            unwrap_or_give_and_return!(floria_store.add_group(floria_group_template), errors, Ok(None));
                        }

                        Err(error) => errors.give(error)?,
                    }
                }

                NODE_TEMPLATE => {
                    tracing::debug!(
                        source = source_id.to_string(),
                        name = name.to_string(),
                        type = "NodeTemplate",
                        "compiling"
                    );

                    match source.get_entity::<NodeTemplate<AnnotatedT>, _>(
                        NODE_TEMPLATE,
                        &node_template_kind_name,
                        &name,
                    ) {
                        Ok(node_template) => {
                            let mut floria_node_template = floria::NodeTemplate::new_for(
                                floria_prefix.clone(),
                                name.into(),
                                Some(floria_service_template_id.clone()),
                            );

                            node_template.compile(
                                &mut floria_node_template,
                                floria_prefix,
                                floria_store.clone(),
                                errors,
                            )?;

                            for (name, capability) in &node_template.capabilities {
                                let mut floria_capability = floria::NodeTemplate::new_for(
                                    floria_prefix.clone(),
                                    name.clone(),
                                    Some(floria_node_template.template.id.clone()),
                                );

                                capability.to_floria(&mut floria_capability, errors)?;

                                floria_node_template
                                    .contained_node_template_ids
                                    .push(floria_capability.template.id.clone());

                                unwrap_or_give_and_return!(
                                    floria_store.add_node_template(floria_capability),
                                    errors,
                                    Ok(None)
                                );
                            }

                            for (name, requirement) in &node_template.requirements {
                                // TODO
                                let node_selector =
                                    floria::NodeSelector::new_node(floria_node_template.template.id.clone());

                                let mut floria_requirement = floria::RelationshipTemplate::new_for(
                                    floria_prefix.clone(),
                                    name.clone(),
                                    floria_node_template.template.id.clone(),
                                    node_selector,
                                );

                                requirement.to_floria(&mut floria_requirement, errors)?;

                                floria_node_template
                                    .contained_node_template_ids
                                    .push(floria_requirement.template.id.clone());

                                unwrap_or_give_and_return!(
                                    floria_store.add_relationship_template(floria_requirement),
                                    errors,
                                    Ok(None)
                                );
                            }

                            floria_service_template
                                .contained_node_template_ids
                                .push(floria_node_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                floria_store.add_node_template(floria_node_template),
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
                        type = "PolicyTemplate",
                        "compiling"
                    );

                    match source.get_entity::<PolicyTemplate<WithAnnotations>, _>(
                        POLICY_TEMPLATE,
                        &policy_template_kind_name,
                        &name,
                    ) {
                        Ok(_policy_template) => {
                            let floria_policy_template = floria::NodeTemplate::new_for(
                                floria_prefix.clone(),
                                name.into(),
                                Some(floria_service_template_id.clone()),
                            );

                            // TODO

                            floria_service_template
                                .contained_node_template_ids
                                .push(floria_policy_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                floria_store.add_node_template(floria_policy_template),
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

        unwrap_or_give_and_return!(floria_store.add_node_template(floria_service_template), errors, Ok(None));

        Ok(Some(floria_service_template_id.clone()))
    }
}
