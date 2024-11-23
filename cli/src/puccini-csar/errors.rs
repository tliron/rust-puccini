use {compris::parse::*, kutil::cli::run::*, read_url::*, std::io, thiserror::*};

//
// MainError
//

/// Main error.
#[derive(Debug, Error)]
pub enum MainError {
    #[error("exit: {0}")]
    #[allow(dead_code)]
    Exit(#[from] ExitError),

    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    #[error("parse: {0}")]
    Parse(#[from] ParseError),

    #[error("URL: {0}")]
    URL(#[from] UrlError),
}

impl RunError for MainError {
    fn handle(&self) -> (bool, u8) {
        (
            false,
            match self {
                MainError::Exit(exit) => exit.code,
                _ => 1,
            },
        )
    }
}
