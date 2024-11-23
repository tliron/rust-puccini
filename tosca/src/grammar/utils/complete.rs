use super::super::{data::*, depot::*, entity::*, errors::*, name::*, source::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, zerocopy::*},
    std::collections::*,
};

/// Complete [BTreeMap].
pub fn complete_map<EntityT, ParentEntityT, ErrorRecipientT>(
    map: &mut BTreeMap<ByteString, EntityT>,
    parent_map: &BTreeMap<ByteString, ParentEntityT>,
    depot: &mut Depot,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Subentity<ParentEntityT>,
    ParentEntityT: IntoScoped<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    for (name, parent_entity) in parent_map {
        match map.get_mut(name) {
            Some(entity) => {
                entity.complete(parent_entity, depot, source_id, scope, errors.clone())?;
            }

            None => {
                map.insert(name.clone(), parent_entity.into_scoped(scope));
            }
        }
    }

    Ok(())
}

/// Complete [TaggedValues].
#[allow(unused_variables)]
pub fn complete_tagged_values<EntityT, ParentEntityT, ErrorRecipientT>(
    tagged_values: &mut TaggedValues<ByteString, EntityT>,
    parent_tagged_values: &TaggedValues<ByteString, ParentEntityT>,
    depot: &mut Depot,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Subentity<ParentEntityT>,
    ParentEntityT: IntoScoped<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    // TODO: what if parent has the same name repeated?

    for (name, parent_entity) in parent_tagged_values {
        match tagged_values.get_first_mut(name) {
            Some(entity) => {
                entity.complete(parent_entity, depot, source_id, scope, errors.clone())?;
            }

            None => {
                tagged_values.add(name.clone(), parent_entity.into_scoped(scope));
            }
        }
    }

    Ok(())
}

///
#[allow(unused_variables)]
pub fn complete_types<ErrorRecipientT>(
    types: &mut Option<Vec<FullName>>,
    parent_types: &Option<Vec<FullName>>,
    depot: &mut Depot,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

///
#[allow(unused_variables)]
pub fn complete_instances<ErrorRecipientT>(
    instances: &mut Option<Vec<Name>>,
    types: &Option<Vec<FullName>>,
    depot: &mut Depot,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}
