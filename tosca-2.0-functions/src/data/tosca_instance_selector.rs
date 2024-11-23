use std::fmt;

//
// ToscaInstanceSelector
//

/// TOSCA instance selector.
#[derive(Clone, Debug)]
pub enum ToscaInstanceSelector {
    /// Index.
    Index(usize),

    /// All.
    All,
}

impl Default for ToscaInstanceSelector {
    fn default() -> Self {
        Self::Index(0)
    }
}

impl fmt::Display for ToscaInstanceSelector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Index(index) => fmt::Display::fmt(index, formatter),
            Self::All => fmt::Display::fmt("ALL", formatter),
        }
    }
}
