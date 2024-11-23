use super::super::{depot::*, errors::*, name::*};

use compris::annotate::*;

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_type_name<ErrorRecipientT>(
    type_name: &FullName,
    parent_type_name: &FullName,
    depot: &Depot,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Call [validate_type_name] on the entity's type.
#[allow(unused_variables)]
pub fn validate_entity_type<ErrorRecipientT>(
    name: &Name,
    type_names: &Option<Vec<FullName>>,
    depot: &Depot,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_entities_types<ErrorRecipientT>(
    names: &Vec<Name>,
    type_names: &Option<Vec<FullName>>,
    depot: &Depot,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    for name in names {
        validate_entity_type(name, type_names, depot, errors)?
    }
    Ok(())
}
