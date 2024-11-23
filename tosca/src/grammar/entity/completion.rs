//
// Completion
//

/// Completion.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Completion {
    /// Incomplete.
    #[default]
    Incomplete,

    /// Complete.
    Complete,

    /// Cannot complete.
    Cannot,
}
