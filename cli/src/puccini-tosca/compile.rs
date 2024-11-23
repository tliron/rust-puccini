#![allow(unreachable_code, unused_assignments)]

use super::{cli::*, errors::*};

use {
    anstream::println,
    compris::annotate::*,
    floria::*,
    kutil::{
        cli::{depict::*, log::*, run::*},
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

        if self.instantiate && self.parse_only {
            return Err(ExitError::from("cannot use `--instantiate` with `--no-completion`").into());
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

        let mut tosca_errors = Errors::default();
        let floria_errors = Errors::<FloriaError>::default();

        // Load

        if self.no_annotations {
            depot.load_source_without_annotations(&source_id, &url_context, &mut tosca_errors)?;
        } else {
            depot.load_source_with_annotations(&source_id, &url_context, &mut tosca_errors)?;
        }

        let floria_store = InMemoryStoreClient::new(InMemoryStoreImplementation::default().into());
        let mut service_template_id = None;

        if !self.parse_only {
            // Complete

            depot.complete_entities(&mut tosca_errors)?;

            // Compile

            let floria_prefix = self.floria_prefix();

            service_template_id =
                depot.compile_service_template(&floria_prefix, floria_store.to_ref(), &source_id, &mut tosca_errors)?;
        }

        // Output

        if !self.quiet {
            let mut print_floria = true;
            let mut first = true;

            if let Err(errors) = tosca_errors.check() {
                print_floria = false;
                first = false;

                errors.annotated_depictions(Some("Puccini Errors".into())).print_default_depiction();
            }

            if let Some(debug) = &self.debug {
                match debug {
                    Debug::Partial => {
                        print_floria = true;
                    }

                    Debug::Namespaces => {
                        print_floria = false;

                        if !first {
                            println!();
                        }
                        first = false;

                        depot.namespaces_depiction().print_default_depiction();
                    }

                    Debug::Entities => {
                        print_floria = false;

                        if !first {
                            println!();
                        }
                        first = false;

                        depot.entities_depiction().print_default_depiction();
                    }
                }
            }

            if print_floria {
                if let Some(service_template_id) = service_template_id
                    && let Some(service_template) = floria_store.get_node_template(&service_template_id)?
                {
                    if !first {
                        println!();
                    }

                    match self.get_output_format() {
                        Some(output_format) => {
                            compris::ser::Serializer::new(output_format)
                                .with_pretty(true)
                                .print(&service_template.to_variant::<_, AnnotatedT>(true, &floria_store)?)
                                .expect("print");
                        }

                        None => {
                            service_template.to_depict(&floria_store).print_depiction(
                                &DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose),
                            );
                        }
                    }
                }
            }
        }

        #[cfg(not(feature = "plugins"))]
        let has_errors = tosca_errors.is_empty();

        #[cfg(feature = "plugins")]
        let has_errors = tosca_errors.is_empty() && floria_errors.is_empty();

        return if has_errors { Ok(()) } else { Err(ExitError::new(1, None).into()) };

        // Parse

        let input_path_or_url = self.input_path_or_url.clone().map(|url| url.into());

        let files = Parser::<AnnotatedT>::new(url_context).parse::<_, tosca_2_0::OldCatalog<'_, _>, _>(
            input_path_or_url,
            &mut tosca_errors.to_resolve_error_recipient(),
        )?;

        let index = Index::default();
        let catalog = tosca_2_0::OldCatalog::new_for_package(&files, &index, &mut tosca_errors)?;

        // Compile

        let store = InMemoryStoreClient::new(InMemoryStoreImplementation::default().into());

        catalog.compile_types_to_floria(&store, &mut tosca_errors)?;

        let floria_scope = self.floria_prefix();
        let service_template_id = files.compile_to_floria(
            CompileToFloriaContext { floria_prefix: &floria_scope, catalog: &catalog, index: &index, store: &store },
            &mut tosca_errors,
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

            if let Err(resolve_errors) = tosca_errors.check() {
                print_result = false;
                first = false;

                resolve_errors.annotated_depictions(Some("Puccini Errors".into())).eprint_default_depiction();
            }

            #[cfg(feature = "plugins")]
            if let Err(floria_errors) = floria_errors.check() {
                print_result = false;

                if !first {
                    println!();
                }
                first = false;

                floria_errors.to_depict("Floria Errors").eprint_default_depiction();
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

                        catalog
                            .print_depiction(&DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose));
                    }

                    Debug::Entities => {
                        print_result = false;

                        for file in files.packaged_files {
                            if !first {
                                println!();
                            }
                            first = false;

                            file.file.print_depiction(
                                &DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose),
                            );
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
                            instance.to_depict(&store).print_depiction(
                                &DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose),
                            );
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
                                    service_template.to_depict(&store).print_depiction(
                                        &DEFAULT_DEPICTION_CONTEXT.child().with_format(DepictionFormat::Verbose),
                                    );
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
        if tosca_errors.is_empty() && floria_errors.is_empty() { Ok(()) } else { Err(ExitError::new(1, None).into()) }
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

    /// Floria prefix.
    pub fn floria_prefix(&self) -> floria::Prefix {
        self.prefix.as_ref().map(|prefix| floria::ID::parse_prefix(prefix)).unwrap_or_else(|| Default::default())
    }
}
