use super::{
    super::{super::super::grammar, entities::*},
    dialect::*,
    entity_kind::*,
};

use {
    compris::annotate::*,
    kutil::{std::error::*, unwrap_or_give_and_return},
};

impl Dialect {
    /// To Floria.
    pub fn create_floria_groups<StoreT, ErrorRecipientT, AnnotatedT>(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: &StoreT,
        source_id: &grammar::SourceID,
        _scope: &grammar::Scope,
        depot: &grammar::Depot,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), grammar::ToscaError<AnnotatedT>>
    where
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<grammar::ToscaError<AnnotatedT>>,
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let source = unwrap_or_give_and_return!(depot.get_source(source_id), errors, Ok(()));

        for (entity_kind, name) in source.entity_names() {
            let floria_type = floria::Group::new_for(floria_prefix.clone(), name.into());

            match entity_kind {
                ARTIFACT_TYPE => {}
                CAPABILITY_TYPE => {}
                DATA_TYPE => {}
                GROUP_TYPE => {}
                INTERFACE_TYPE => {}
                NODE_TYPE => {}
                POLICY_TYPE => {}
                RELATIONSHIP_TYPE => {}
                _ => {}
            }

            unwrap_or_give_and_return!(floria_store.add_group(floria_type), errors, Ok(()));
        }

        for (source_id, scope) in &source.dependencies {
            self.create_floria_groups(floria_prefix, floria_store, source_id, scope, depot, errors)?;
        }

        Ok(())
    }

    /// To Floria.
    pub fn create_floria_service_template<StoreT, ErrorRecipientT, AnnotatedT>(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: &StoreT,
        source_id: &grammar::SourceID,
        depot: &grammar::Depot,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, grammar::ToscaError<AnnotatedT>>
    where
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<grammar::ToscaError<AnnotatedT>>,
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        self.create_floria_groups(floria_prefix, floria_store, source_id, &Default::default(), depot, errors)?;

        let source = unwrap_or_give_and_return!(depot.get_source(source_id), errors, Ok(None));

        let mut floria_service_template = unwrap_or_give_and_return!(
            floria::NodeTemplate::new(floria_prefix.clone(), floria_store),
            errors,
            Ok(None)
        );

        let floria_service_template_id = floria_service_template.template.id.clone();
        let group_template_kind_name = self.implementation.entity_kinds.represent(GROUP_TEMPLATE);
        let node_template_kind_name = self.implementation.entity_kinds.represent(NODE_TEMPLATE);
        let policy_template_kind_name = self.implementation.entity_kinds.represent(POLICY_TEMPLATE);

        for (entity_kind, name) in source.entity_names() {
            match entity_kind {
                GROUP_TEMPLATE => {
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

                            node_template.to_floria(&mut floria_node_template, errors)?;

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
                    match source.get_entity::<PolicyTemplate<AnnotatedT>, _>(
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
