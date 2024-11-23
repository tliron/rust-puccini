use super::{
    super::{errors::*, source::*},
    catalog::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl Catalog {
    /// Compile service template to Floria.
    pub fn compile_service_template<AnnotatedT, ErrorRecipientT>(
        &self,
        directory: &floria::Directory,
        store: floria::StoreRef,
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
            .compile_source(directory, store, source_id, self, errors.into_annotated().to_ref())
            .map_err(|error| error.into_annotated())
    }
}
