#![warn(missing_docs)]

/*!
Puccini
*/

mod cli;
mod compile;
mod errors;
#[cfg(feature = "plugins")]
mod instantiate;
mod run;

use run::*;

use std::process::*;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
