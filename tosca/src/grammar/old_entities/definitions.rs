use super::{
    super::{data::*, errors::*, index::*},
    definition::*,
    r#type::*,
    types::*,
};

use {
    kutil::std::{error::*, zerocopy::*},
    std::collections::*,
    tynm::*,
};

///
#[derive(Debug)]
pub struct DefinitionsCompleteContext<'own, DefinitionsT, TypeT, CatalogT, AnnotatedT> {
    ///
    pub parent_definitions: Option<&'own DefinitionsT>,
    ///
    pub types: &'own Types<'own, TypeT, CatalogT, AnnotatedT>,
    ///
    pub catalog: &'own CatalogT,
    ///
    pub index: &'own Index,
}

//
// Definitions
//

/// Definitions.
pub trait Definitions<DefinitionT, TypeT, CatalogT, AnnotatedT>
where
    Self: Sized,
    DefinitionT: Definition<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    /// Complete.
    fn complete<ErrorRecipientT>(
        &self,
        context: DefinitionsCompleteContext<Self, TypeT, CatalogT, AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl<DefinitionT, TypeT, CatalogT, AnnotatedT> Definitions<DefinitionT, TypeT, CatalogT, AnnotatedT>
    for BTreeMap<ByteString, DefinitionT>
where
    DefinitionT: Clone + Definition<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    fn complete<ErrorRecipientT>(
        &self,
        context: DefinitionsCompleteContext<Self, TypeT, CatalogT, AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_definitions = BTreeMap::default();

        tracing::info!("complete: {}", type_name::<DefinitionT>());

        match context.parent_definitions {
            Some(parent_definitions) => {
                for (definition_name, definition) in self {
                    if parent_definitions.contains_key(definition_name) {
                        tracing::info!("complete: already contains: {}", definition_name);
                    } else {
                        match definition.get_type_name() {
                            Some(type_name) => match context.index.index.get(type_name) {
                                Some(type_id) => match context.types.get_by_id(type_id) {
                                    Some(type_) => {
                                        tracing::info!("complete: entype: {} {}", definition_name, type_name);
                                        complete_definitions.insert(
                                            definition_name.clone(),
                                            definition.entype(
                                                DefinitionEntypeContext {
                                                    definition_name,
                                                    type_name,
                                                    type_,
                                                    catalog: context.catalog,
                                                    index: context.index,
                                                },
                                                errors,
                                            )?,
                                        );
                                    }

                                    None => {
                                        tracing::warn!("complete: type not found: {} {}", definition_name, type_id);
                                        complete_definitions.insert(definition_name.clone(), definition.clone());
                                    }
                                },

                                None => {
                                    tracing::warn!("complete: type not found: {} {}", definition_name, type_name);
                                    complete_definitions.insert(definition_name.clone(), definition.clone());
                                }
                            },

                            None => {
                                tracing::warn!("complete: keep: {}", definition_name);
                                complete_definitions.insert(definition_name.clone(), definition.clone());
                            }
                        }
                    }
                }

                for (parent_definition_name, parent_definition) in parent_definitions {
                    match self.get(parent_definition_name) {
                        Some(definition) => match parent_definition.get_type_name() {
                            Some(type_name) => {
                                tracing::info!("complete: derive: {} {}", parent_definition_name, type_name);
                                complete_definitions.insert(
                                    parent_definition_name.clone(),
                                    definition.derive(
                                        DefinitionDeriveContext {
                                            definition_name: parent_definition_name,
                                            parent_definition: parent_definition,
                                            catalog: context.catalog,
                                            index: context.index,
                                        },
                                        errors,
                                    )?,
                                );
                            }

                            None => {
                                tracing::warn!("complete: no parent type name: {}", parent_definition_name);
                            }
                        },

                        None => {
                            // tracing::info!(
                            //     "complete: clone: {} into {}",
                            //     parent_definition_name,
                            //     parent_definitions.namespace.join(":"),
                            // );
                            let definition = parent_definition.clone();
                            // definition.to_namespace(&parent_definitions.namespace);
                            complete_definitions.insert(parent_definition_name.clone(), definition);
                        }
                    }
                }
            }

            None => {
                tracing::info!("complete: no parent definitions");
                complete_definitions = self.clone();
            }
        }

        Ok(complete_definitions)
    }
}

impl<DefinitionT, TypeT, CatalogT, AnnotatedT> Definitions<DefinitionT, TypeT, CatalogT, AnnotatedT>
    for TaggedValues<ByteString, DefinitionT>
where
    DefinitionT: Clone + Definition<TypeT, CatalogT, AnnotatedT>,
    TypeT: Type<CatalogT, AnnotatedT>,
{
    fn complete<ErrorRecipientT>(
        &self,
        context: DefinitionsCompleteContext<Self, TypeT, CatalogT, AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut complete_definitions = TaggedValues::default();

        // TODO: this is probably wrong

        match context.parent_definitions {
            Some(parent_definitions) => {
                for (definition_name, definition) in self {
                    if parent_definitions.contains_tag(definition_name) {
                        tracing::info!("complete: already contains: {}", definition_name);
                    } else {
                        match definition.get_type_name() {
                            Some(type_name) => match context.index.index.get(type_name) {
                                Some(type_id) => match context.types.get_by_id(type_id) {
                                    Some(type_) => {
                                        tracing::info!("complete: entype: {} {}", definition_name, type_name);
                                        complete_definitions.add(
                                            definition_name.clone(),
                                            definition.entype(
                                                DefinitionEntypeContext {
                                                    definition_name,
                                                    type_name,
                                                    type_,
                                                    catalog: context.catalog,
                                                    index: context.index,
                                                },
                                                errors,
                                            )?,
                                        );
                                    }

                                    None => {
                                        tracing::warn!("complete: type not found: {} {}", definition_name, type_id);
                                        complete_definitions.add(definition_name.clone(), definition.clone());
                                    }
                                },

                                None => {
                                    tracing::warn!("complete: type not found: {} {}", definition_name, type_name);
                                    complete_definitions.add(definition_name.clone(), definition.clone());
                                }
                            },

                            None => {
                                tracing::info!("complete: keep: {}", definition_name);
                                complete_definitions.add(definition_name.clone(), definition.clone());
                            }
                        }
                    }
                }

                for (parent_definition_name, parent_definition) in parent_definitions {
                    match self.get_first(parent_definition_name) {
                        Some(definition) => match parent_definition.get_type_name() {
                            Some(type_name) => {
                                tracing::info!("complete: derive: {} {}", parent_definition_name, type_name);
                                complete_definitions.add(
                                    parent_definition_name.clone(),
                                    definition.derive(
                                        DefinitionDeriveContext {
                                            definition_name: parent_definition_name,
                                            parent_definition: parent_definition,
                                            catalog: context.catalog,
                                            index: context.index,
                                        },
                                        errors,
                                    )?,
                                );
                            }

                            None => {
                                tracing::warn!("complete: no parent type name: {}", parent_definition_name);
                            }
                        },

                        None => {
                            // tracing::info!(
                            //     "complete: clone: {} into {}",
                            //     parent_definition_name,
                            //     parent_definition_name.namespace.join(":"),
                            // );
                            let definition = parent_definition.clone();
                            // definition.to_namespace(&parent_definitions.namespace);
                            complete_definitions.add(parent_definition_name.clone(), definition);
                        }
                    }
                }
            }

            None => {
                tracing::info!("complete: no parent definitions");
                complete_definitions = self.clone();
            }
        }

        Ok(complete_definitions)
    }
}
