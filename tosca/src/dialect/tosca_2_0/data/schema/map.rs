use super::{
    super::{data_kind::*, expression::*},
    value::*,
};

use {kutil::cli::depict::*, std::collections::*};

//
// MapSchema
//

/// Map schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct MapSchema<AnnotatedT> {
    /// Key schema reference.
    #[depict(style(number))]
    pub key: SchemaReference,

    /// Entry schema reference.
    #[depict(style(number))]
    pub entry: SchemaReference,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validator.
    #[depict(option, as(depict))]
    pub validator: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> PartialEq for MapSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key)
            && (self.entry == other.entry)
            && (self.default == other.default)
            && (self.validator == other.validator)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for MapSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Map.as_str().into());
        map.insert("key".into(), (self.key as u64).into());
        map.insert("entry".into(), (self.entry as u64).into());

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validator) = self.validator {
            map.insert("validator".into(), validator);
        }

        map.into()
    }
}
