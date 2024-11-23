use super::super::{call::*, expression::*, schema::*};

use compris::{annotate::*, normal::*};

//
// ValidationExpression
//

/// Validation expression.
pub trait ValidationExpression<AnnotatedT> {
    /// Returns an `$_apply` with other before self.
    ///
    /// If self or other are already `$_apply` will flatten to a single `$_apply`.
    fn apply_validation(&mut self, other: Expression<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default;

    /// Finalize validation as a flattened `$_apply`.
    ///
    /// Expressions are wrapped in `$_assert` if necessary.
    ///
    /// If there is a schema it will be wrapped in `$_schema`.
    fn finalize_validation(&mut self, schema: Option<Schema<AnnotatedT>>)
    where
        AnnotatedT: Annotated + Clone + Default;

    /// Complete validation as a flattened `$_apply`.
    ///
    /// Expressions are wrapped in `$_assert` if necessary.
    fn complete_validation(
        &mut self,
        parent_validation: Option<&Expression<AnnotatedT>>,
        struct_annotations: &mut StructAnnotations,
        parent_struct_annotations: &StructAnnotations,
    ) where
        AnnotatedT: Annotated + Clone + Default;
}

impl<AnnotatedT> ValidationExpression<AnnotatedT> for Option<Expression<AnnotatedT>> {
    fn apply_validation(&mut self, other: Expression<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match self.take() {
            Some(validation) => {
                if validation.is_apply() {
                    // Add to existing $_apply
                    if let Expression::Call(mut apply_call) = validation {
                        apply_call.arguments.insert(0, other);
                    } else {
                        panic!("should be a call");
                    }
                } else if other.is_apply() {
                    // Add to existing $_apply
                    if let Expression::Call(mut apply_call) = other {
                        apply_call.arguments.push(validation);
                        *self = Some(apply_call.into());
                    } else {
                        panic!("should be a call");
                    }
                } else {
                    // No need to join if they are the same
                    if validation != other {
                        // Join with $_apply
                        *self = Some(
                            Call::new_native(
                                Text::from("_apply").with_annotations_from(&validation),
                                vec![other, validation],
                            )
                            .into(),
                        );
                    }
                }
            }

            None => {
                if other.is_apply() {
                    *self = Some(other);
                } else {
                    *self =
                        Some(Call::new_native(Text::from("_apply").with_annotations_from(&other), vec![other]).into());
                }
            }
        }
    }

    fn finalize_validation(&mut self, schema: Option<Schema<AnnotatedT>>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let must_assert = self.as_ref().map(|validation| validation.must_assert()).unwrap_or(false);

        if must_assert && let Some(validation) = self.take() {
            *self = Some(
                Call::new_native(Text::from("_assert").with_annotations_from(&validation), vec![validation]).into(),
            );
        }

        if let Some(schema) = schema {
            self.apply_validation(schema.into_validation());
        }
    }

    fn complete_validation(
        &mut self,
        parent_validation: Option<&Expression<AnnotatedT>>,
        struct_annotations: &mut StructAnnotations,
        parent_struct_annotations: &StructAnnotations,
    ) where
        AnnotatedT: Annotated + Clone + Default,
    {
        // Clone annotations
        if self.is_none() && parent_validation.is_some() {
            if let Some(annotations) = parent_struct_annotations.get("validation") {
                struct_annotations.insert("validation".into(), annotations.clone());
            }
        }

        if let Some(parent_validation) = parent_validation {
            self.apply_validation(parent_validation.clone());
        }
    }
}

/// Complete validation as a flattened `$_apply`.
///
/// Expressions are wrapped in `$_assert` if necessary.
#[macro_export]
macro_rules! complete_validation (
    (
        $self:ident,
        $parent:ident $(,)?
    ) => {
        $self.validation.complete_validation(
            $parent.validation.as_ref(),
            &mut $self.annotations,
            &$parent.annotations,
        )
    }
);

#[allow(unused_imports)]
pub use complete_validation;
