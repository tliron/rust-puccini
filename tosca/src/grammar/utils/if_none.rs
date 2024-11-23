/// Clone if [None].
pub fn if_none_clone<DataT>(data: &mut Option<DataT>, parent_data: &Option<DataT>)
where
    DataT: Clone,
{
    if data.is_none() && parent_data.is_some() {
        *data = parent_data.clone();
    }
}

/// Call function if [None].
pub fn if_none_call<DataT, ParentDataT>(data: &mut Option<DataT>, parent_data: ParentDataT)
where
    DataT: Clone,
    ParentDataT: Fn() -> Option<DataT>,
{
    if data.is_none() {
        *data = parent_data();
    }
}
