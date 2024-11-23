use super::{
    super::{
        super::super::grammar::{self, IntoAnnotated, ResolveWithToscaErrors},
        entities::*,
    },
    entity_kind::*,
};

use {
    compris::{annotate::*, normal::*},
    kutil::{std::error::*, unwrap_or_give_and_return},
};

/// Dialect ID.
pub const DIALECT_ID: grammar::DialectID = grammar::DialectID::from_static("tosca_2_0");

//
// Dialect
//

/// TOSCA 2.0 dialect.
#[derive(Clone, Debug)]
pub struct Dialect {
    /// Implementation.
    pub implementation: grammar::DialectImplementation,
}

impl Dialect {
    fn initialize_source<AnnotatedT>(
        &self,
        source: &mut grammar::Source,
        variant: Variant<AnnotatedT>,
        errors: grammar::ToscaErrorRecipientRef,
    ) -> Result<(), grammar::ToscaError<AnnotatedT>>
    where
        AnnotatedT: 'static + Annotated + Default + Clone,
    {
        let file: Option<File<_>> = variant.resolve_with_tosca_errors(errors.clone())?;

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

            let errors = &mut errors.to_error_recipient();
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

impl Default for Dialect {
    fn default() -> Self {
        Self { implementation: grammar::DialectImplementation::new(DIALECT_ID.clone(), entity_kinds()) }
    }
}

impl grammar::Dialect for Dialect {
    fn dialect_id(&self) -> grammar::DialectID {
        self.implementation.dialect_id()
    }

    fn entity_kinds(&self) -> &grammar::EntityKinds {
        &self.implementation.entity_kinds
    }

    fn initialize_source_with_annotations(
        &self,
        source: &mut grammar::Source,
        variant: Variant<WithAnnotations>,
        errors: grammar::ToscaErrorRecipientRef,
    ) -> Result<(), grammar::ToscaError<WithAnnotations>> {
        self.initialize_source(source, variant, errors)
    }

    fn initialize_source_without_annotations(
        &self,
        source: &mut grammar::Source,
        variant: Variant<WithoutAnnotations>,
        errors: grammar::ToscaErrorRecipientRef,
    ) -> Result<(), grammar::ToscaError<WithoutAnnotations>> {
        self.initialize_source(source, variant, errors)
    }
}
