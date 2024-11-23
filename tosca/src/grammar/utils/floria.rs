use super::super::{data::*, errors::*, old_entities::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        std::{error::*, zerocopy::*},
        unwrap_or_give_and_return,
    },
};

//
// FloriaToscaMetadata
//

/// Set Floria metadata for TOSCA.
pub trait FloriaToscaMetadata {
    /// Set `tosca:entity` metadata.
    fn set_tosca_entity(&mut self, name: &str);

    /// Set `tosca:parent` metadata.
    fn set_tosca_parent(&mut self, name: &str);

    /// Set `tosca:parent` metadata.
    fn set_tosca_parent_old(&mut self, name: Option<&ID>);

    /// Set `tosca:description` metadata.
    fn set_tosca_description(&mut self, description: Option<&ByteString>);

    /// Merge in the TOSCA [Metadata].
    fn merge_tosca_metadata<AnnotatedT>(&mut self, from_metadata: &Metadata<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default;

    /// Set `tosca:version` metadata.
    fn set_tosca_version(&mut self, version: Option<&Version>);

    /// Set `tosca:directives` metadata.
    fn set_tosca_directives(&mut self, directives: &Vec<ByteString>);
}

impl FloriaToscaMetadata for floria::Metadata {
    fn set_tosca_entity(&mut self, name: &str) {
        self.into_insert("tosca:entity", name);
    }

    fn set_tosca_parent(&mut self, name: &str) {
        self.into_insert("tosca:parent", name);
    }

    fn set_tosca_parent_old(&mut self, id: Option<&ID>) {
        if let Some(id) = id {
            self.into_insert("tosca:parent", id.to_string());
        }
    }

    fn set_tosca_description(&mut self, description: Option<&ByteString>) {
        if let Some(description) = description {
            if !description.is_empty() {
                self.into_insert("tosca:description", description.clone());
            }
        }
    }

    fn merge_tosca_metadata<AnnotatedT>(&mut self, from_metadata: &Metadata<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        self.inner
            .extend(from_metadata.iter().map(|(key, value)| (key.clone().into(), value.clone().into_annotated())));
    }

    fn set_tosca_version(&mut self, version: Option<&Version>) {
        if let Some(version) = version {
            self.into_insert("tosca:version", version.to_string());
        }
    }

    fn set_tosca_directives(&mut self, directives: &Vec<ByteString>) {
        if !directives.is_empty() {
            let directives: Vec<Variant<_>> =
                directives.into_iter().map(|directive| directive.clone().into()).collect();
            self.into_insert("tosca:directives", directives);
        }
    }
}

/// Add TOSCA type.
pub fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
    template: &mut floria::Template,
    type_name: ByteString,
    floria_prefix: &floria::Prefix,
    floria_store: floria::StoreRef,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<AnnotatedT>>
where
    ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
{
    let mut id = floria::ID::new_for(floria::Kind::Group, floria_prefix.clone(), type_name);

    loop {
        match unwrap_or_give_and_return!(floria_store.get_group(&id), errors, Ok(())) {
            Some(group) => {
                template.group_ids.push(group.id.clone());
                match group.metadata.inner.get(&"tosca:parent".into()) {
                    Some(parent) => {
                        id.id = parent.to_string().into();
                    }
                    None => {
                        break;
                    }
                }
            }

            None => {
                // TODO
                break;
            }
        }
    }

    Ok(())
}
