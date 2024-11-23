use super::super::{call::*, dispatch::*};

use compris::annotate::*;

/// Complete validation by merging expressions with `$and`.
pub fn complete_validation<AnnotatedT>(
    validation: &mut Option<super::Expression<AnnotatedT>>,
    parent_validation: Option<&super::Expression<AnnotatedT>>,
    struct_annotations: &mut StructAnnotations,
    parent_struct_annotations: &StructAnnotations,
) where
    AnnotatedT: Clone + Default,
{
    match validation {
        Some(my_validation) => {
            if let Some(parent_validation) = parent_validation {
                if my_validation != parent_validation {
                    *validation = Some(
                        Call::new(
                            get_dispatch_name("and").into(),
                            vec![parent_validation.clone(), my_validation.clone()],
                        )
                        .into(),
                    );
                }
            }
        }

        None => {
            if parent_validation.is_some() {
                *validation = parent_validation.cloned();
                if let Some(annotations) = parent_struct_annotations.get("validation") {
                    struct_annotations.insert("validation".into(), annotations.clone());
                }
            }
        }
    }
}
