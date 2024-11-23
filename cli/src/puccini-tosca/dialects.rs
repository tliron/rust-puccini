use super::cli::*;

use {
    compris::annotate::*,
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    std::fmt,
};

#[cfg(feature = "plugins")]
use floria::{plugins::*, *};

impl Compile {
    /// TOSCA [Catalog] with supported dialects.
    pub fn catalog<AnnotatedT>() -> Catalog
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut catalog = Catalog::default();
        catalog.add_dialect_ref(tosca_2_0::Dialect::default().into());
        catalog.add_source(tosca_2_0::Dialect::implicit_source::<AnnotatedT>());
        catalog
    }

    /// Floria [Library] with the plugins for supported dialects.
    #[cfg(feature = "plugins")]
    pub fn library<'env, StoreT>(environment: &'env Environment<StoreT>) -> Result<Library<'env, StoreT>, FloriaError>
    where
        StoreT: Clone + Send + Store,
    {
        let mut library = Library::new(environment);

        library.add_dispatch_plugin(
            tosca_2_0::DIALECT_ID.into(),
            include_bytes!("../../../target/wasm32-wasip2/release/puccini_plugin_tosca_2_0_functions.wasm"),
        )?;

        Ok(library)
    }
}
