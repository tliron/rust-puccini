use super::super::super::super::grammar::*;

use {kutil::std::immutable::*, std::fmt};

//
// DataKind
//

/// Data kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataKind {
    /// String.
    String,

    /// Integer.
    Integer,

    /// Float.
    Float,

    /// Boolean.
    Boolean,

    /// Bytes.
    Bytes,

    /// Nil.
    Nil,

    /// Timestamp.
    Timestamp,

    /// Version.
    Version,

    /// Scalar.
    ///
    /// Allows "data_type", "units", "canonical_unit", and "prefixes".
    Scalar,

    /// List.
    ///
    /// Allows "entry_schema".
    List,

    /// Map.
    ///
    /// Allows "key_schema" and "entry_schema".
    Map,

    /// Struct.
    ///
    /// Allows "properties".
    Struct,
}

impl DataKind {
    /// As string.
    pub fn as_str(&self) -> &'static str {
        match self {
            DataKind::String => "string",
            DataKind::Integer => "integer",
            DataKind::Float => "float",
            DataKind::Boolean => "boolean",
            DataKind::Bytes => "bytes",
            DataKind::Nil => "nil",
            DataKind::Timestamp => "timestamp",
            DataKind::Version => "version",
            DataKind::Scalar => "scalar",
            DataKind::List => "list",
            DataKind::Map => "map",
            DataKind::Struct => "struct",
        }
    }
}

impl fmt::Display for DataKind {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), formatter)
    }
}

impl Into<&'static str> for DataKind {
    fn into(self) -> &'static str {
        self.as_str()
    }
}

impl Into<Name> for DataKind {
    fn into(self) -> Name {
        Name(ByteString::from_static(self.as_str()))
    }
}
