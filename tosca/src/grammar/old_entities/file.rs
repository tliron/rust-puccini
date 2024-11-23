use super::{super::errors::*, assignment::*, importable::*};

use kutil::std::{error::*, zerocopy::*};

//
// File
//

/// File.
pub trait File<CatalogT, AnnotatedT> {
    /// Get importables.
    fn get_importables(&self) -> Vec<Importable>;

    /// Compile to Floria.
    fn compile_to_floria<StoreT, ErrorRecipientT>(
        &self,
        context: CompileToFloriaContext<CatalogT, StoreT>,
        source: ByteString,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    where
        StoreT: floria::Store,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}
