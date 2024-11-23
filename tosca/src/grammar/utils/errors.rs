/// Errors with field annotations.
#[macro_export]
macro_rules! errors_with_field_annotations (
    ( $errors:ident, $self:ident, $field:expr, $( $code:tt )* ) => {
        {
            let annotations = $self.field_annotations($field).cloned();
            let $errors = &mut $errors.with_annotations(annotations.as_ref());
            $( $code )*
        }
    };
);

#[allow(unused_imports)]
pub use errors_with_field_annotations;
