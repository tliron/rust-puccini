use super::super::{catalog::*, errors::*, name::*};

use {
    compris::{annotate::*, resolve::*},
    kutil::std::error::*,
};

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_type<TypeT, ErrorRecipientT>(
    type_: &TypeT,
    parent_type_name: &FullName,
    catalog: &Catalog,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    Ok(())
}

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_type_name<ErrorRecipientT>(
    type_name: &FullName,
    parent_type_name: &FullName,
    catalog: &Catalog,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    if type_name.is_empty() {
        errors.give(ResolveError::from(InvalidKeyError::new("type_name".into())))?;
    }

    Ok(())
}

/// Call [validate_type_name] on the entity's type.
#[allow(unused_variables)]
pub fn validate_entity_type<ErrorRecipientT>(
    name: &Name,
    type_names: &Option<Vec<FullName>>,
    catalog: &Catalog,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_entities_types<ErrorRecipientT>(
    names: &Vec<Name>,
    type_names: &Option<Vec<FullName>>,
    catalog: &Catalog,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    for name in names {
        validate_entity_type(name, type_names, catalog, errors)?
    }
    Ok(())
}
