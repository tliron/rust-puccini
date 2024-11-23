#![allow(unreachable_code, unused_assignments)]

use super::{cli::*, errors::*};

use {
    anstream::println,
    compris::annotate::*,
    floria::*,
    kutil::{
        cli::{debug::*, log::*, run::*},
        std::error::*,
    },
    puccini_tosca::{dialect::tosca_2_0, grammar::*, parse::Parser},
    read_url::*,
    std::fmt,
};

impl Compile {
    /// Run compile subcommand.
    pub fn run(&self) -> Result<(), MainError> {
        if !self.quiet {
            self.output_colorize.initialize();
            initialize_tracing(self.verbose + 2, self.log_path.as_ref())?;
        }

        if self.no_annotations {
            self.run_annotated::<WithoutAnnotations>()
        } else {
            self.run_annotated::<WithAnnotations>()
        }
    }

    /// Run compile subcommand.
    pub fn run_annotated<AnnotatedT>(&self) -> Result<(), MainError>
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default + Send + Sync,
    {
        let url_context = self.url_context()?;
        let source_id = self.source_id();
        let mut depot = Self::depot();

        let mut errors = Errors::default();

        // Load

        if self.no_annotations {
            depot.load_source_without_annotations(&source_id, &url_context, &mut errors)?;
        } else {
            depot.load_source_with_annotations(&source_id, &url_context, &mut errors)?;
        }

        // Complete

        depot.complete_entities::<AnnotatedT, _>(&mut errors)?;

        // Output

        if !self.quiet {
            let mut print_result = true;
            let mut first = true;

            if let Err(errors) = errors.check() {
                print_result = false;
                first = false;

                errors.annotated_debuggables(Some("Puccini Errors".into())).print_debug();
            }

            if let Some(debug) = &self.debug {
                match debug {
                    Debug::Partial => {
                        print_result = true;
                    }

                    Debug::Namespaces => {
                        if !first {
                            println!();
                        }
                        first = false;

                        depot.to_debuggable_namespaces().print_debug();
                    }

                    Debug::Entities => {
                        if !first {
                            println!();
                        }
                        first = false;

                        depot.to_debuggable_entities().print_debug();
                    }
                }
            }

            if print_result {
                // TODO
            }
        }

        return Ok(());

        // Parse

        let input_path_or_url = self.input_path_or_url.clone().map(|url| url.into());

        let files = Parser::<AnnotatedT>::new(url_context)
            .parse::<_, tosca_2_0::OldCatalog<'_, _>, _>(input_path_or_url, &mut errors.to_resolve_error_recipient())?;

        let index = Index::default();
        let catalog = tosca_2_0::OldCatalog::new_for_package(&files, &index, &mut errors)?;

        // Compile

        let store = InMemoryStoreClient::new(InMemoryStoreImplementation::default().into());

        catalog.compile_types_to_floria(&store, &mut errors)?;

        let floria_scope = self.floria_scope();
        let service_template_id = files.compile_to_floria(
            CompileToFloriaContext { floria_scope: &floria_scope, catalog: &catalog, index: &index, store: &store },
            &mut errors,
        )?;

        // Instantiate

        #[cfg(feature = "plugins")]
        let mut floria_errors = Errors::default();

        #[cfg(feature = "plugins")]
        let instance = {
            if self.instantiate {
                match &service_template_id {
                    Some(service_template_id) => {
                        self.instantiate(&service_template_id, &floria_scope, &store, &mut floria_errors)?
                    }
                    None => None,
                }
            } else {
                None
            }
        };

        // Output

        if !self.quiet {
            let mut print_result = true;
            let mut first = true;

            if let Err(resolve_errors) = errors.check() {
                print_result = false;
                first = false;

                resolve_errors.annotated_debuggables(Some("Puccini Errors".into())).eprint_debug();
            }

            #[cfg(feature = "plugins")]
            if let Err(floria_errors) = floria_errors.check() {
                print_result = false;

                if !first {
                    println!();
                }
                first = false;

                floria_errors.to_debuggable("Floria Errors").eprint_debug();
            }

            if let Some(debug) = &self.debug {
                match debug {
                    Debug::Partial => {
                        print_result = true;
                    }

                    Debug::Namespaces => {
                        print_result = false;

                        if !first {
                            println!();
                        }
                        first = false;

                        catalog.print_debug_with_format(DebugFormat::Verbose);
                    }

                    Debug::Entities => {
                        print_result = false;

                        for file in files.packaged_files {
                            if !first {
                                println!();
                            }
                            first = false;

                            file.file.print_debug_with_format(DebugFormat::Verbose);
                        }
                    }
                }
            }

            if print_result {
                #[cfg(feature = "plugins")]
                if let Some(instance) = instance {
                    print_result = false;

                    if !first {
                        println!();
                    }

                    match self.get_output_format() {
                        Some(output_format) => {
                            compris::ser::Serializer::new(output_format)
                                .with_pretty(true)
                                .print(&instance.to_variant::<_, WithoutAnnotations>(true, &store)?)
                                .expect("print");
                        }

                        None => {
                            instance.to_debuggable(&store).print_debug_with_format(DebugFormat::Verbose);
                        }
                    }
                }

                if print_result {
                    if let Some(node_template_id) = service_template_id {
                        if let Some(service_template) = store.get_node_template(&node_template_id)? {
                            if !first {
                                println!();
                            }

                            match self.get_output_format() {
                                Some(output_format) => {
                                    compris::ser::Serializer::new(output_format)
                                        .with_pretty(true)
                                        .print(&service_template.to_variant::<_, WithoutAnnotations>(true, &store)?)
                                        .expect("print");
                                }

                                None => {
                                    service_template
                                        .to_debuggable(&store)
                                        .print_debug_with_format(DebugFormat::Verbose);
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(not(feature = "plugins"))]
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Exit::new(1, None).into())
        }

        #[cfg(feature = "plugins")]
        if errors.is_empty() && floria_errors.is_empty() { Ok(()) } else { Err(ExitError::new(1, None).into()) }
    }

    /// Compris format.
    pub fn get_output_format(&self) -> Option<compris::Format> {
        self.output_format.as_ref().and_then(|format| format.to_compris())
    }

    /// Depot.
    pub fn depot() -> Depot {
        let mut depot = Depot::default();
        depot.add_dialect_ref(tosca_2_0::Dialect::default().into());
        depot
    }

    /// URL context.
    pub fn url_context(&self) -> Result<UrlContextRef, MainError> {
        let url_context = UrlContext::new();

        #[cfg(feature = "filesystem")]
        let base_urls = url_context.working_dir_url_vec()?;
        #[cfg(not(feature = "filesystem"))]
        let base_urls = Vec::default();

        Ok(url_context.with_base_urls(base_urls))
    }

    /// Source ID.
    pub fn source_id(&self) -> SourceID {
        SourceID::url_or_default(self.input_path_or_url.clone().map(|input_path_or_url| input_path_or_url.into()))
    }

    /// Floria scope.
    pub fn floria_scope(&self) -> floria::Scope {
        self.namespace.as_ref().map(|scope| floria::ID::parse_scope(scope)).unwrap_or_else(|| Default::default())
    }
}
