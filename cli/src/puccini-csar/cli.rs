use {clap::*, kutil::cli::clap::*, std::path::*};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// CSAR packaging tool
#[derive(Parser)]
#[command(
    name = "puccini-csar",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    arg_required_else_help = true,
    styles = clap_styles())
]
pub struct CLI {
    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,

    /// suppress output;
    /// if you only need the exit code
    #[arg(long, short = 'q', verbatim_doc_comment, default_value_t)]
    pub quiet: bool,

    /// log to file path;
    /// defaults to stderr, applying --colorize
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub log_path: Option<PathBuf>,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub verbose: u8,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum)]
    pub output_colorize: Colorize,

    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

//
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// show the version of puccini-csar
    #[command(action = ArgAction::Version)]
    Version(Version),

    /// output the shell autocompletion script
    Completion(Completion),
}
