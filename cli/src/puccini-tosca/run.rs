use super::{cli::*, errors::*};

use clap::*;

pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    match &cli.subcommand {
        None => {}
        Some(subcommand) => match subcommand {
            SubCommand::Compile(compile) => compile.run()?,
            SubCommand::Version(version) => version.run::<CLI>(),
            SubCommand::Completion(completion) => completion.run::<CLI>(),
            SubCommand::Manual(manual) => manual.run::<CLI>()?,
        },
    }

    Ok(())
}
