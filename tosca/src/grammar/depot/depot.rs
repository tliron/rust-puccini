use super::super::{dialect::*, source::*};

use kutil::std::collections::*;

//
// Depot
//

/// Intended as the sole owner of sources.
#[derive(Debug, Default)]
pub struct Depot {
    /// Dialects.
    pub dialects: FastHashMap<DialectID, DialectRef>,

    /// Sources.
    pub sources: FastHashMap<SourceID, Source>,
}
