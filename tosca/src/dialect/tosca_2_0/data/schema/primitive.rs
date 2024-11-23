use super::super::{data_kind::*, expression::*};

use {kutil::cli::depict::*, std::collections::*};

//
// PrimitiveSchema
//

/// Primitive schema.
#[derive(Clone, Debug, Depict)]
pub struct PrimitiveSchema<AnnotatedT> {
    /// Data kind.
    #[depict(style(symbol))]
    pub data_kind: DataKind,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validator.
    #[depict(option, as(depict))]
    pub validator: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> PartialEq for PrimitiveSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.data_kind == other.data_kind) && (self.default == other.default) && (self.validator == other.validator)
    }
}

impl<AnnotatedT> From<DataKind> for PrimitiveSchema<AnnotatedT> {
    fn from(data_kind: DataKind) -> Self {
        Self { data_kind, default: None, validator: None }
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for PrimitiveSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        if self.default.is_none() && self.validator.is_none() {
            // The only value that can be represented as text
            return self.data_kind.as_str().into();
        }

        let mut map = BTreeMap::default();

        map.insert("kind".into(), self.data_kind.as_str().into());

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validator) = self.validator {
            map.insert("validator".into(), validator);
        }

        map.into()
    }
}
