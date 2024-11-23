use super::super::name::*;

/// Call function if [None].
pub fn complete<DataT, GetParentDataT>(data: &Option<DataT>, get_parent_data: GetParentDataT) -> Option<DataT>
where
    DataT: Clone,
    GetParentDataT: Fn() -> Option<DataT>,
{
    data.clone().or_else(get_parent_data)
}

/// Clone if [None].
pub fn complete_clone<DataT>(data: &Option<DataT>, parent_data: &Option<DataT>) -> Option<DataT>
where
    DataT: Clone,
{
    complete(data, || parent_data.clone())
}

/// Clone from container if [None].
pub fn complete_clone_from<'parent, DataT, ParentContainerT, GetParentDataT>(
    data: &Option<DataT>,
    parent_container: Option<ParentContainerT>,
    get_parent_data: GetParentDataT,
) -> Option<DataT>
where
    DataT: 'parent + Clone,
    GetParentDataT: Fn(ParentContainerT) -> &'parent Option<DataT>,
{
    data.clone().or_else(|| parent_container.and_then(|parent_container| get_parent_data(parent_container).clone()))
}

/// Complete type names.
///
/// Clone if [None].
///
/// Otherwise, validate that the types are equal to or derived from any of the parent type names.
pub fn complete_type_names(
    type_names: &Option<Vec<FullName>>,
    parent_type_names: &Option<Vec<FullName>>,
) -> Option<Vec<FullName>> {
    // TODO: refinement rules; must not break contract with parent
    match type_names {
        Some(type_names) => match parent_type_names {
            Some(parent_type_names) => {
                let mut refined_type_names = Vec::default();
                refined_type_names.extend_from_slice(type_names);
                refined_type_names.extend_from_slice(parent_type_names);
                Some(refined_type_names)
            }

            None => None,
        },

        None => parent_type_names.clone(),
    }
}

/// Complete type names from container.
///
/// See [complete_type_names].
pub fn complete_type_names_from<'parent, ParentContainerT, GetParentTypeNamesT>(
    type_names: &Option<Vec<FullName>>,
    parent_container: Option<ParentContainerT>,
    get_parent_type_names: GetParentTypeNamesT,
) -> Option<Vec<FullName>>
where
    GetParentTypeNamesT: Fn(ParentContainerT) -> &'parent Option<Vec<FullName>>,
{
    match type_names {
        Some(_) => complete_type_names(
            type_names,
            match parent_container {
                Some(parent_container) => get_parent_type_names(parent_container),
                None => &None,
            },
        ),

        None => parent_container.and_then(|parent_container| get_parent_type_names(parent_container).clone()),
    }
}
