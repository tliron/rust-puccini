use super::super::{data::*, dialect::*, errors::*, name::*};

use {
    compris::{annotate::*, normal::*},
    kutil::std::{error::*, immutable::*},
};

//
// FloriaToscaMetadata
//

/// Set Floria metadata for TOSCA.
pub trait FloriaToscaMetadata {
    /// Set `tosca:dialect` and `tosca:entity` metadata.
    fn set_tosca_entity(&mut self, dialect: DialectID, name: ByteString);

    /// Set `tosca:dialect` and `tosca:entity` metadata.
    fn set_tosca_entity_static(&mut self, dialect: DialectID, name: &'static str);

    /// Set `tosca:parent` metadata.
    fn set_tosca_parent(&mut self, full_name: &FullName);

    /// Set `tosca:description` metadata.
    fn set_tosca_description(&mut self, description: Option<&ByteString>);

    /// Merge in the TOSCA [Metadata].
    fn merge_tosca_metadata<AnnotatedT>(&mut self, from_metadata: &Metadata<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default;

    /// Set `tosca:internal` metadata.
    fn set_tosca_internal(&mut self, internal: bool);

    /// Set `tosca:version` metadata.
    fn set_tosca_version(&mut self, version: Option<String>);

    /// Set `tosca:directives` metadata.
    fn set_tosca_directives(&mut self, directives: &Vec<ByteString>);
}

impl FloriaToscaMetadata for floria::Metadata {
    fn set_tosca_entity(&mut self, dialect: DialectID, name: ByteString) {
        self.into_insert("tosca:dialect", dialect);
        self.into_insert("tosca:entity", name);
    }

    fn set_tosca_entity_static(&mut self, dialect: DialectID, name: &'static str) {
        self.set_tosca_entity(dialect, ByteString::from_static(name));
    }

    fn set_tosca_parent(&mut self, full_name: &FullName) {
        self.into_insert("tosca:parent", full_name.to_string());
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
        self.inner.extend(
            from_metadata
                .iter()
                .map(|(key, value)| ((String::from("custom:") + key).into(), value.clone().into_annotated())),
        );
    }

    fn set_tosca_internal(&mut self, internal: bool) {
        self.into_insert("tosca:internal", internal);
    }

    fn set_tosca_version(&mut self, version: Option<String>) {
        if let Some(version) = version {
            self.into_insert("tosca:version", version);
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

//
// FloriaToscaType
//

/// Add TOSCA type and its ancestors as Floria classes.
pub trait FloriaToscaType {
    /// Add TOSCA type and its ancestors as Floria classes.
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        floria_directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut id = floria::ID::new_for(floria::Kind::Class, floria_directory.clone(), type_name.to_string().into());

        loop {
            match unwrap_or_give_and_return!(store.get_class(&id), errors, Ok(())) {
                Some(class) => {
                    self.push(class.id.clone());
                    match class.metadata.inner.get(&"tosca:parent".into()) {
                        Some(parent) => id.id = parent.to_string().into(),
                        None => break,
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
}
