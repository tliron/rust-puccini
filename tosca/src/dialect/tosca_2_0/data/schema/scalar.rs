use super::super::{data_kind::*, expression::*};

use {
    compris::normal::*,
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// ScalarSchema
//

/// Scalar schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct ScalarSchema<AnnotatedT> {
    /// Data kind.
    #[depict(option, style(symbol))]
    pub data_kind: Option<DataKind>,

    /// Units.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub units: BTreeMap<ByteString, Variant<AnnotatedT>>,

    /// Canonical unit.
    #[depict(option, style(string))]
    pub canonical_unit: Option<ByteString>,

    /// Prefixes.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub prefixes: BTreeMap<ByteString, Variant<AnnotatedT>>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validator.
    #[depict(option, as(depict))]
    pub validator: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> PartialEq for ScalarSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.data_kind == other.data_kind)
            && (self.units == other.units)
            && (self.canonical_unit == other.canonical_unit)
            && (self.prefixes == other.prefixes)
            && (self.default == other.default)
            && (self.validator == other.validator)
    }
}

impl<AnnotatedT> ScalarSchema<AnnotatedT> {
    /// Constructor.
    pub fn new(
        data_kind: Option<DataKind>,
        units: BTreeMap<ByteString, Variant<AnnotatedT>>,
        canonical_unit: Option<ByteString>,
        prefixes: BTreeMap<ByteString, Variant<AnnotatedT>>,
    ) -> Self {
        Self { data_kind, units, canonical_unit, prefixes, default: None, validator: None }
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for ScalarSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Scalar.as_str().into());

        if let Some(data_kind) = self.data_kind {
            map.insert("data_kind".into(), data_kind.as_str().into());
        }

        if !self.units.is_empty() {
            let units: BTreeMap<_, _> = self.units.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
            map.insert("units".into(), units.into());
        }

        if let Some(canonical_unit) = self.canonical_unit {
            map.insert("canonical_unit".into(), canonical_unit.into());
        }

        if !self.prefixes.is_empty() {
            let prefixes: BTreeMap<_, _> =
                self.prefixes.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
            map.insert("prefixes".into(), prefixes.into());
        }

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validator) = self.validator {
            map.insert("validator".into(), validator);
        }

        map.into()
    }
}
