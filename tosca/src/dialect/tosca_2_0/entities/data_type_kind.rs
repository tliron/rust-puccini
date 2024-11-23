use kutil::std::*;

//
// DataTypeKind
//

/// Data type kind.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum DataTypeKind {
    /// Primitive.
    Primitive,

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

    /// Complex.
    ///
    /// Allows "properties".
    Complex,
}
