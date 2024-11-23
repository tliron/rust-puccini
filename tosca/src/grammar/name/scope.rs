use super::name::*;

use {
    kutil::cli::depict::*,
    std::{fmt, io},
};

//
// Scope
//

/// Name scope.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scope {
    /// Segments.
    pub segments: Vec<Name>,
}

impl Scope {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// To Floria directory.
    pub fn to_floria_directory(&self) -> floria::Directory {
        self.segments.iter().map(|segment| segment.clone().into()).collect()
    }
}

impl Depict for Scope {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for segment in &self.segments {
            context.theme.write_name(writer, segment)?;
            context.theme.write_delimiter(writer, ':')?;
        }
        Ok(())
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            write!(formatter, "{}:", segment)?;
        }
        Ok(())
    }
}

impl From<Vec<Name>> for Scope {
    fn from(segments: Vec<Name>) -> Self {
        Self { segments }
    }
}

impl From<Name> for Scope {
    fn from(name: Name) -> Self {
        Self { segments: vec![name] }
    }
}
