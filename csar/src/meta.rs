use super::version::*;

//
// Meta
//

/// CSAR meta.
pub struct Meta {
    /// Version.
    pub version: Version,

    /// CSAR version.
    pub csar_version: Version,

    /// Created by.
    pub created_by: String,

    /// Other definitions.
    pub other_definitions: String,
}
