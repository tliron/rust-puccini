use super::{
    super::{
        super::super::grammar::{self, IntoAnnotated, ToResolveErrorRecipient},
        entities::*,
    },
    dialect::*,
    entity_kind::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{std::error::*, unwrap_or_give_and_return},
};

impl Dialect {
    /// Initialize source.
    pub fn initialize_source<AnnotatedT, ErrorRecipientT>(
        &self,
        source: &mut grammar::Source,
        variant: Variant<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), grammar::ToscaError<AnnotatedT>>
    where
        AnnotatedT: 'static + Annotated + Default + Clone,
        ErrorRecipientT: ErrorRecipient<grammar::ToscaError<AnnotatedT>>,
    {
        let file: Option<File<_>> =
            variant.resolve_with_errors(&mut errors.into_annotated().to_resolve_error_recipient())?;

        if let Some(file) = file {
            //file.print_debug();

            for import in file.imports {
                if let Some(url) = import.url {
                    source.add_dependency(
                        grammar::SourceID::URL(url),
                        import.namespace.map(|name| name.into()).unwrap_or_default(),
                    );
                }
            }

            let mut errors = errors.into_annotated();

            for (name, artifact_type) in file.artifact_types {
                unwrap_or_give_and_return!(source.add_entity(ARTIFACT_TYPE, name, artifact_type), errors, Ok(()));
            }

            for (name, capability_type) in file.capability_types {
                unwrap_or_give_and_return!(source.add_entity(CAPABILITY_TYPE, name, capability_type), errors, Ok(()));
            }

            for (name, data_type) in file.data_types {
                unwrap_or_give_and_return!(source.add_entity(DATA_TYPE, name, data_type), errors, Ok(()));
            }

            for (name, group_type) in file.group_types {
                unwrap_or_give_and_return!(source.add_entity(GROUP_TYPE, name, group_type), errors, Ok(()));
            }

            for (name, interface_type) in file.interface_types {
                unwrap_or_give_and_return!(source.add_entity(INTERFACE_TYPE, name, interface_type), errors, Ok(()));
            }

            for (name, node_type) in file.node_types {
                unwrap_or_give_and_return!(source.add_entity(NODE_TYPE, name, node_type), errors, Ok(()));
            }

            for (name, policy_type) in file.policy_types {
                unwrap_or_give_and_return!(source.add_entity(POLICY_TYPE, name, policy_type), errors, Ok(()));
            }

            for (name, relationship_type) in file.relationship_types {
                unwrap_or_give_and_return!(
                    source.add_entity(RELATIONSHIP_TYPE, name, relationship_type),
                    errors,
                    Ok(())
                );
            }

            if let Some(service_template) = file.service_template {
                for (name, group_template) in service_template.groups {
                    unwrap_or_give_and_return!(source.add_entity(GROUP_TEMPLATE, name, group_template), errors, Ok(()));
                }

                for (name, node_template) in service_template.node_templates {
                    unwrap_or_give_and_return!(source.add_entity(NODE_TEMPLATE, name, node_template), errors, Ok(()));
                }

                let mut name = 0;
                for policy_template in service_template.policies {
                    name += 1;
                    let name = name.to_string().parse().expect("policy template name");
                    unwrap_or_give_and_return!(
                        source.add_entity(POLICY_TEMPLATE, name, policy_template),
                        errors,
                        Ok(())
                    );
                }

                for (name, relationship_template) in service_template.relationship_templates {
                    unwrap_or_give_and_return!(
                        source.add_entity(RELATIONSHIP_TEMPLATE, name, relationship_template),
                        errors,
                        Ok(())
                    );
                }
            }

            for (name, repository) in file.repositories {
                unwrap_or_give_and_return!(source.add_entity(REPOSITORY, name, repository), errors, Ok(()));
            }
        }

        // source.add_entity(NODE_TYPE, "child".into(), MyNodeType::new("Child", Some("parent")));
        // source.add_entity(NODE_TYPE, "parent".into(), MyNodeType::new("Parent", None));
        // source.add_entity(NODE_TYPE, "grandchild".into(), MyNodeType::new("Grandchild", Some("child")));

        Ok(())
    }
}
