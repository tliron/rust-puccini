use super::super::source::*;

use {
    kutil::cli::debug::*,
    std::{fmt, io},
    thiserror::*,
};

//
// UnsupportedSourceError
//

/// Unsupported source error.
#[derive(Debug, Error)]
pub struct UnsupportedSourceError {
    /// Source ID.
    pub source_id: SourceID,
}

impl UnsupportedSourceError {
    /// Constructor.
    pub fn new(source_id: SourceID) -> Self {
        Self { source_id }
    }
}

impl Debuggable for UnsupportedSourceError {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "unsupported source: {}", context.theme.error(&self.source_id))
    }
}

impl fmt::Display for UnsupportedSourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.source_id, formatter)
    }
}
