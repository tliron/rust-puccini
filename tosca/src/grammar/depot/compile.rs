use super::{
    super::{errors::*, source::*},
    depot::*,
};

use {
    compris::annotate::*,
    kutil::{std::error::*, unwrap_or_give_and_return},
};

impl Depot {
    /// To Floria.
    pub fn compile_service_template<AnnotatedT, ErrorRecipientT>(
        &self,
        floria_prefix: &floria::Prefix,
        floria_store: floria::StoreRef,
        source_id: &SourceID,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let source = unwrap_or_give_and_return!(self.get_source(source_id), errors, Ok(None));
        let dialect = unwrap_or_give_and_return!(self.get_dialect_ref(&source.dialect_id), errors, Ok(None));
        dialect
            .compile_source(floria_prefix, floria_store, source_id, self, errors.into_annotated().to_ref())
            .map_err(|error| error.into_annotated())
    }
}
