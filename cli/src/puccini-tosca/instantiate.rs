use super::{cli::*, errors::*};

use {
    floria::{plugins::*, *},
    kutil::std::error::*,
};

// TODO:
// TOSCA inputs
// TOSCA outputs
// call operation

impl Compile {
    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        service_template_id: &ID,
        scope: &Scope,
        store: &StoreT,
        errors: &mut Errors<FloriaError>,
    ) -> Result<Option<Node>, MainError>
    where
        StoreT: 'static + Store,
    {
        let node_template = store
            .get_node_template(service_template_id)?
            .ok_or_else(|| StoreError::ID(service_template_id.to_string()))?;

        const PLUGIN_NAME: &str = "TOSCA 2.0";
        const TOSCA_2_0: &[u8] =
            include_bytes!("../../../target/wasm32-wasip2/release/puccini_plugin_tosca_2_0_functions.wasm");

        let environment = Environment::new(store.clone());
        let mut library = Library::new(&environment);
        library.add_dispatch_plugin(PLUGIN_NAME, TOSCA_2_0)?;

        Ok(Some(node_template.instantiate(&scope, None, &mut library, PLUGIN_NAME, errors)?))
    }
}
