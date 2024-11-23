use super::{cli::*, errors::*, utils::*};

use {
    anstream::println,
    compris::{annotate::*, normal::*},
    floria::*,
    kutil::{
        cli::{depict::*, log::*, run::*},
        std::error::*,
    },
    puccini_tosca::grammar::*,
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

        #[cfg(not(feature = "plugins"))]
        if self.instantiate {
            return Err(ExitError::from("to use `--instantiate` you must enable \"plugins\" feature in build").into());
        }

        if self.update && !self.instantiate {
            return Err(ExitError::from("cannot use `--update` without `--instantiate`").into());
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
        let mut catalog = Self::catalog::<AnnotatedT>();

        let mut tosca_errors = Errors::<ToscaError<AnnotatedT>>::default();

        #[cfg(feature = "plugins")]
        let mut floria_errors = Errors::<FloriaError>::default();

        // Load

        if self.no_annotations {
            catalog.load_source_without_annotations(&source_id, &url_context, &mut tosca_errors)?;
        } else {
            catalog.load_source_with_annotations(&source_id, &url_context, &mut tosca_errors)?;
        }

        // Complete
        if self.should_complete() {
            catalog.complete_entities(&mut tosca_errors)?;
        }

        let store = InMemoryStore::default();
        let mut floria_service_template_id = None;

        let directory = self.floria_directory();

        // Compile
        if self.should_compile() {
            floria_service_template_id =
                catalog.compile_service_template(&directory, store.to_ref(), &source_id, &mut tosca_errors)?;
        }

        #[cfg(feature = "plugins")]
        let mut floria_instance = None;

        // Instantiate
        #[cfg(feature = "plugins")]
        if self.should_instantiate()
            && let Some(floria_service_template_id) = &floria_service_template_id
        {
            floria_instance = self.instantiate(floria_service_template_id, &directory, &store, &mut floria_errors)?;
        }

        // Output

        if !self.quiet {
            let mut print_floria = true;
            let mut first = true;

            // Depict TOSCA errors

            if let Err(tosca_errors) = tosca_errors.check() {
                print_floria = false;
                first = false;

                tosca_errors.annotated_depictions(Some("TOSCA Errors".into())).print_default_depiction();
            }

            // Depict Floria errors

            #[cfg(feature = "plugins")]
            if let Err(floria_errors) = floria_errors.check() {
                print_floria = false;

                if !first {
                    println!();
                }
                first = false;

                floria_errors.to_depict("Floria Errors").print_default_depiction();
            }

            // Depict debug

            match self.debug {
                Some(Debug::Namespaces) => {
                    print_floria = false;

                    if !first {
                        println!();
                    }
                    first = false;

                    catalog.namespaces_depiction().print_default_depiction();
                }

                Some(Debug::Parsed | Debug::Completed) => {
                    print_floria = false;

                    if !first {
                        println!();
                    }
                    first = false;

                    catalog.entities_depiction().print_default_depiction();
                }

                Some(Debug::Compiled) => {
                    // Force it to be true, even if there were compilation errors
                    print_floria = true;
                }

                _ => {}
            }

            // Output Floria instance

            #[cfg(feature = "plugins")]
            if print_floria
                && !matches!(self.debug, Some(Debug::Compiled))
                && let Some(floria_instance) = floria_instance
            {
                print_floria = false;

                if !first {
                    println!();
                }

                output!(self, store, floria_instance);
            }

            // Output Floria template

            if print_floria
                && let Some(floria_service_template_id) = floria_service_template_id
                && let Some(floria_service_template) = store.get_vertex_template(&floria_service_template_id)?
            {
                if !first {
                    println!();
                }

                output!(self, store, floria_service_template);
            }
        }

        #[cfg(not(feature = "plugins"))]
        let has_errors = tosca_errors.is_empty();

        #[cfg(feature = "plugins")]
        let has_errors = tosca_errors.is_empty() && floria_errors.is_empty();

        return if has_errors { Ok(()) } else { Err(ExitError::new(1, None).into()) };
    }

    /// Compris format.
    pub fn get_output_format(&self) -> Option<compris::Format> {
        self.output_format.as_ref().and_then(|format| format.to_compris())
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

    /// Floria directory.
    pub fn floria_directory(&self) -> floria::Directory {
        self.directory
            .as_ref()
            .map(|directory| floria::ID::parse_directory(directory))
            .unwrap_or_else(|| Default::default())
    }
}
