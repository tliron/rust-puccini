use floria_plugin_sdk::data::*;

//
// Comparator
//

/// Comparator.
pub trait Comparator {
    /// Comparator.
    fn comparator(&self) -> Expression;
}
