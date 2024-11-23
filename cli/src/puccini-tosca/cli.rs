use {clap::*, kutil::cli::clap::*, std::path::*};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// TOSCA parser and compiler
#[derive(Parser)]
#[command(
    name = "puccini-tosca",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    arg_required_else_help = true,
    styles = clap_styles())
]
pub struct CLI {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// compile a TOSCA service template to a Floria template
    Compile(Compile),

    /// show the version of puccini-tosca
    Version(Version),

    /// output the shell auto-completion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}

//
// Compile
//

/// Compile subcommand.
#[derive(Args)]
pub struct Compile {
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(verbatim_doc_comment)]
    pub input_path_or_url: Option<String>,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_path: Option<PathBuf>,

    /// output format;
    /// when absent will be set to input format
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub output_format: Option<OutputFormat>,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum)]
    pub output_colorize: Colorize,

    /// plain output;
    /// avoid whitespace and colors
    #[arg(long = "plain", short = 'p')]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long = "base64", short = 'b', verbatim_doc_comment)]
    pub output_base64: bool,

    /// compile into Floria directory
    #[arg(long = "directory")]
    pub directory: Option<String>,

    /// simulate instantiation into Floria directory;
    /// only if there are no compilation errors
    #[arg(long = "instantiate", short = 'i', verbatim_doc_comment)]
    pub instantiate: bool,

    /// simulate update of the Floria instance;
    /// requires `--instantiate`
    #[arg(long = "update", short = 'u', verbatim_doc_comment)]
    pub update: bool,

    /// disable annotations
    #[arg(long = "no-annotations")]
    pub no_annotations: bool,

    /// output debug information
    #[arg(long, short = 'd', value_enum)]
    pub debug: Option<Debug>,

    /// suppress console output
    #[arg(long, short = 'q')]
    pub quiet: bool,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub verbose: u8,

    /// log to file path;
    /// defaults to stderr
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub log_path: Option<PathBuf>,

    // TODO not used
    /// timeout in seconds;
    /// 0 for no timeout
    #[arg(long, short = 't', verbatim_doc_comment, default_value_t = 0.0)]
    pub timeout: f64,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    YAML,
    JSON,
    XJSON,
    //XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
    Depict,
}

impl OutputFormat {
    /// To Compris format.
    pub fn to_compris(&self) -> Option<compris::Format> {
        match self {
            OutputFormat::YAML => Some(compris::Format::YAML),
            OutputFormat::JSON => Some(compris::Format::JSON),
            OutputFormat::XJSON => Some(compris::Format::XJSON),
            //OutputFormat::XML => Some(compris::Format::XML),
            OutputFormat::CBOR => Some(compris::Format::CBOR),
            OutputFormat::MessagePack => Some(compris::Format::MessagePack),
            OutputFormat::Depict => None,
        }
    }
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}

//
// Debug
//

/// Debug flag.
#[derive(Clone, ValueEnum)]
pub enum Debug {
    /// output namespaces (stops before completion phase)
    Namespaces,

    /// output parsed entities (stops before completion phase)
    Parsed,

    /// output completed entities (stops before compilation phase)
    Completed,

    /// output compiled Floria templates (even if there are errors)
    Compiled,
}

impl Compile {
    /// True if we should run completion phase.
    pub fn should_complete(&self) -> bool {
        match self.debug {
            Some(Debug::Namespaces | Debug::Parsed) => false,
            _ => true,
        }
    }

    /// True if we should run compilation phase.
    pub fn should_compile(&self) -> bool {
        match self.debug {
            Some(Debug::Completed | Debug::Namespaces | Debug::Parsed) => false,
            _ => true,
        }
    }

    /// True if we should run instantiation phase.
    pub fn should_instantiate(&self) -> bool {
        match self.debug {
            Some(_) => false,
            _ => self.instantiate,
        }
    }
}
