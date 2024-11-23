use super::{name::*, old_entities::*};

use kutil::std::collections::*;

//
// Index
//

/// Maps [Name] to [ID].
#[derive(Clone, Debug, Default)]
pub struct Index {
    /// Index.
    pub index: FastHashMap<FullName, ID>,
}
