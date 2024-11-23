use super::super::{dialect::*, source::*};

use kutil::std::collections::*;

//
// Catalog
//

/// Container for entities organized by source.
#[derive(Debug, Default)]
pub struct Catalog {
    /// Dialects.
    pub dialects: FastHashMap<DialectID, DialectRef>,

    /// Sources.
    pub sources: FastHashMap<SourceID, Source>,
}
