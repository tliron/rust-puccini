use crate::grammar::IntoAnnotated;

use super::{super::super::super::grammar, entity_kind::*};

use compris::{annotate::*, normal::*};
use kutil::std::error::ToErrorRecipient;

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
        self.initialize_source(source, variant, &mut errors.to_error_recipient().into_annotated())
    }

    fn initialize_source_without_annotations(
        &self,
        source: &mut grammar::Source,
        variant: Variant<WithoutAnnotations>,
        errors: grammar::ToscaErrorRecipientRef,
    ) -> Result<(), grammar::ToscaError<WithoutAnnotations>> {
        self.initialize_source(source, variant, &mut errors.to_error_recipient().into_annotated())
    }

    fn compile_source(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: floria::StoreRef,
        source_id: &grammar::SourceID,
        depot: &grammar::Depot,
        errors: grammar::ToscaErrorRecipientRef,
    ) -> Result<Option<floria::ID>, grammar::ToscaError<WithAnnotations>> {
        self.compile_service_template(
            floria_prefix,
            floria_store,
            source_id,
            depot,
            &mut errors.to_error_recipient().into_annotated(),
        )
    }
}
