use floria_plugin_sdk::data::*;

//
// Coerce
//

/// Coerce.
pub trait Coerce {
    /// Coerce into the schema.
    fn coerce(&self, expression: Expression) -> Result<Expression, String>;

    /// Coerce into the schema.
    ///
    /// Returns [None] if the expression was not modified.
    fn coerce_option(&self, expression: Option<Expression>) -> Result<Option<Expression>, String>;
}
