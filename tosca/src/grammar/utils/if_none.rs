use compris::annotate::*;

/// Clone [StructAnnotations].
pub fn clone_struct_annotations(struct_annotations: &StructAnnotations, field_names: &[&str]) -> StructAnnotations {
    let mut clone = StructAnnotations::default();
    for field_name in field_names {
        if let Some(annotations) = struct_annotations.get(*field_name) {
            clone.insert((*field_name).into(), annotations.clone());
        }
    }
    clone
}

/// Clone if [None].
pub fn if_none_clone<DataT>(
    data: &mut Option<DataT>,
    parent_data: &Option<DataT>,
    struct_annotations: &mut StructAnnotations,
    parent_struct_annotations: &StructAnnotations,
    field_name: &str,
) where
    DataT: Clone,
{
    if data.is_none() && parent_data.is_some() {
        *data = parent_data.clone();
        if let Some(annotations) = parent_struct_annotations.get(field_name) {
            struct_annotations.insert(field_name.into(), annotations.clone());
        }
    }
}

/// Clone if `is_empty`.
#[macro_export]
macro_rules! if_empty_clone {
    ( $data:expr, $parent_data:expr, $struct_annotations:expr, $parent_struct_annotations:expr, $field_name:expr $(,)? ) => {{
        if $data.is_empty() && !$parent_data.is_empty() {
            $data = $parent_data.clone();
            if let Some(annotations) = $parent_struct_annotations.get($field_name) {
                $struct_annotations.insert($field_name.into(), annotations.clone());
            }
        }
    }};
}

/// Call function if [None].
pub fn if_none_call<DataT, ParentDataT>(
    data: &mut Option<DataT>,
    parent_data: ParentDataT,
    struct_annotations: &mut StructAnnotations,
    parent_struct_annotations: &StructAnnotations,
    field_name: &str,
) where
    DataT: Clone,
    ParentDataT: Fn() -> Option<DataT>,
{
    if data.is_none() {
        *data = parent_data();
        if let Some(annotations) = parent_struct_annotations.get(field_name) {
            struct_annotations.insert(field_name.into(), annotations.clone());
        }
    }
}
