use std::sync::PoisonError;

use super::super::grammar::*;

use {
    compris::{parse::*, resolve::*},
    kutil::cli::depict::*,
    read_url::*,
    std::io,
    thiserror::*,
};

//
// PucciniError
//

/// Puccini error.
#[derive(Debug, Depict, Error)]
pub enum PucciniError<AnnotatedT> {
    /// Parse.
    #[error("parse: {0}")]
    Parse(#[from] ParseError),

    /// Resolve.
    #[error("resolve: {0}")]
    #[depict(as(depict))]
    Resolve(#[from] ResolveError<AnnotatedT>),

    /// TOSCA.
    #[error("TOSCA: {0}")]
    #[depict(as(depict))]
    TOSCA(#[from] ToscaError<AnnotatedT>),

    /// URL.
    #[error("URL: {0}")]
    URL(#[from] UrlError),

    /// I/O.
    #[error("I/O: {0}")]
    IO(#[from] io::Error),

    /// Concurrency.
    #[error("concurrency: {0}")]
    Concurrency(String),
}

impl<AnnotatedT, PoisonT> From<PoisonError<PoisonT>> for PucciniError<AnnotatedT> {
    fn from(error: PoisonError<PoisonT>) -> Self {
        PucciniError::Concurrency(error.to_string())
    }
}
