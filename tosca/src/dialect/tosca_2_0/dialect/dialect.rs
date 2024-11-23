use super::super::super::super::grammar::{Dialect as DialectTrait, *};

use {
    compris::{annotate::*, normal::*},
    kutil::std::error::*,
};

/// Dialect ID.
pub const DIALECT_ID: DialectID = DialectID::from_static("tosca_2_0");

//
// Dialect
//

/// TOSCA 2.0 dialect.
#[derive(Clone, Debug)]
pub struct Dialect {
    /// Implementation.
    pub implementation: DialectImplementation,
}

impl Default for Dialect {
    fn default() -> Self {
        Self { implementation: DialectImplementation::new(DIALECT_ID, Self::entity_kinds()) }
    }
}

impl DialectTrait for Dialect {
    fn dialect_id(&self) -> DialectID {
        self.implementation.dialect_id()
    }

    fn entity_kinds(&self) -> &EntityKinds {
        &self.implementation.entity_kinds
    }

    fn initialize_source_with_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        self.initialize_source(source, variant, &mut errors.to_error_recipient().into_annotated())
    }

    fn initialize_source_without_annotations(
        &self,
        source: &mut Source,
        variant: Variant<WithoutAnnotations>,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithoutAnnotations>> {
        self.initialize_source(source, variant, &mut errors.to_error_recipient().into_annotated())
    }

    fn compile_source(
        &self,
        directory: &floria::Directory,
        store: floria::StoreRef,
        source_id: &SourceID,
        catalog: &Catalog,
        errors: ToscaErrorRecipientRef,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
        self.compile_service_template(
            directory,
            &store,
            source_id,
            catalog,
            &mut errors.to_error_recipient().into_annotated(),
        )
    }
}
