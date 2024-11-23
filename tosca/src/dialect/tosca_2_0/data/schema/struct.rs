use super::{
    super::{data_kind::*, expression::*},
    value::*,
};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// StructSchema
//

/// Struct schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct StructSchema<AnnotatedT> {
    /// Fields.
    #[depict(iter(kv), style(number), key_style(string))]
    pub fields: BTreeMap<ByteString, SchemaReference>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validator.
    #[depict(option, as(depict))]
    pub validator: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> PartialEq for StructSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.fields == other.fields) && (self.default == other.default) && (self.validator == other.validator)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for StructSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Struct.as_str().into());

        let fields: BTreeMap<_, _> =
            self.fields.into_iter().map(|(key, value)| (key.into(), (value as u64).into())).collect();
        map.insert("fields".into(), fields.into());

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validator) = self.validator {
            map.insert("validator".into(), validator);
        }

        map.into()
    }
}
