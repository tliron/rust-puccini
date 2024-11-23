use super::super::{catalog::*, data::*, entity::*, errors::*, name::*, source::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, zerocopy::*},
    std::collections::*,
};

/// Complete [BTreeMap].
pub fn complete_map<EntityT, ParentEntityT, ErrorRecipientT>(
    map: &mut BTreeMap<ByteString, EntityT>,
    parent_map: Option<(&BTreeMap<ByteString, ParentEntityT>, &Scope)>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Subentity<ParentEntityT>,
    ParentEntityT: IntoScoped<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    match parent_map {
        Some((parent_map, scope)) => {
            for (name, parent_entity) in parent_map {
                match map.get_mut(name) {
                    Some(entity) => {
                        entity.complete(Some((parent_entity, scope)), catalog, source_id, errors.clone())?;
                    }

                    None => {
                        let mut entity = parent_entity.into_scoped(scope);
                        entity.complete(Some((parent_entity, scope)), catalog, source_id, errors.clone())?;
                        map.insert(name.clone(), entity);
                    }
                }
            }
        }

        None => {
            for entity in map.values_mut() {
                entity.complete(None, catalog, source_id, errors.clone())?;
            }
        }
    }

    Ok(())
}

/// Complete [TaggedValues].
#[allow(unused_variables)]
pub fn complete_tagged_values<EntityT, ParentEntityT, ErrorRecipientT>(
    tagged_values: &mut TaggedValues<ByteString, EntityT>,
    parent_tagged_values: Option<(&TaggedValues<ByteString, ParentEntityT>, &Scope)>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Subentity<ParentEntityT>,
    ParentEntityT: IntoScoped<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    // TODO: what if parent has the same name repeated?

    match parent_tagged_values {
        Some((parent_tagged_values, scope)) => {
            for (name, parent_entity) in parent_tagged_values {
                match tagged_values.get_first_mut(name) {
                    Some(entity) => {
                        entity.complete(Some((parent_entity, scope)), catalog, source_id, errors.clone())?;
                    }

                    None => {
                        let mut entity = parent_entity.into_scoped(scope);
                        entity.complete(Some((parent_entity, scope)), catalog, source_id, errors.clone())?;
                        tagged_values.add(name.clone(), entity);
                    }
                }
            }
        }

        None => {
            for (_name, entity) in tagged_values {
                entity.complete(None, catalog, source_id, errors.clone())?;
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
    catalog: &mut Catalog,
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
    catalog: &mut Catalog,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Complete map.
#[macro_export]
macro_rules! complete_map (
    (
        $self:ident,
        $field:tt,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        errors_with_field_annotations!(
            $errors, $self, stringify!($field),
            complete_map(
                &mut $self.$field,
                $entity.as_ref().map(|(entity, scope)| (&entity.$field, *scope)),
                $catalog,
                $source_id,
                $errors,
            )?;
        );
    }
);

/// Complete tagged values.
#[macro_export]
macro_rules! complete_tagged_values (
    (
        $self:ident,
        $field:tt,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        errors_with_field_annotations!(
            $errors, $self, stringify!($field),
            complete_tagged_values(
                &mut $self.$field,
                $entity.as_ref().map(|(entity, scope)| (&entity.$field, *scope)),
                $catalog,
                $source_id,
                $errors,
            )?;
        );
    }
);

/// Get complete entity and scope from field.
#[macro_export]
macro_rules! get_complete_entity (
    (
        $kind:ident,
        $tosca_type:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match $catalog
            .get_complete_entity::<$tosca_type<AnnotatedT>, _, _>(
                $kind,
                &$self.$field,
                $source_id,
                &mut $errors.with_field_annotations($self, stringify!($field)),
            )?
            .cloned()
        {
            Some(entity) => Some((entity, &$self.$field.scope)),
            None => None,
        }
    }
);

/// Get complete parent and scope from field.
#[macro_export]
macro_rules! get_complete_parent (
    (
        $kind:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $callstack:ident,
        $errors:ident $(,)?
    ) => {
        match &$self.$field {
            Some(field) => {
                match $catalog
                    .get_complete_entity_next::<Self, _, _>(
                        $kind,
                        field,
                        $source_id,
                        $callstack,
                        &mut $errors.with_field_annotations($self, stringify!($field)),
                    )?
                    .cloned()
                {
                    Some(parent) => Some((parent, &field.scope)),
                    None => None,
                }
            }

            None => None,
        }
    }
);
