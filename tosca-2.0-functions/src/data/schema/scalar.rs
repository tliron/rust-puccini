use super::{
    super::{number::*, scalar::*},
    coerce::*,
};

use {floria_plugin_sdk::data::*, std::collections::*};

//
// ScalarSchema
//

/// Scalar schema.
#[derive(Clone, Debug, Default)]
pub struct ScalarSchema {
    /// Data kind.
    pub data_kind: Option<String>,

    /// Units.
    pub units: BTreeMap<String, Number>,

    /// Canonical unit.
    pub canonical_unit: Option<String>,

    /// Prefixes.
    pub prefixes: BTreeMap<String, Number>,

    /// Default.
    pub default: Option<Expression>,

    /// Validator.
    pub validator: Option<Expression>,
}

impl ScalarSchema {
    /// Constructor.
    pub fn new(
        data_kind: Option<String>,
        units: BTreeMap<String, Number>,
        canonical_unit: Option<String>,
        prefixes: BTreeMap<String, Number>,
        default: Option<Expression>,
        validator: Option<Expression>,
    ) -> Self {
        Self { data_kind, units, canonical_unit, prefixes, default, validator }
    }

    /// True if data kind is integer.
    pub fn is_integer(&self) -> bool {
        self.data_kind.as_ref().map_or(false, |data_kind| data_kind == "integer")
    }

    /// Find canonical unit.
    pub fn canonical_unit(&self) -> Result<String, String> {
        match &self.canonical_unit {
            Some(canonical_unit) => {
                let _ = self.unit_factor(canonical_unit)?;
                Ok(canonical_unit.clone())
            }

            None => {
                let mut canonical_unit = None;
                for (unit, factor) in &self.units {
                    if factor.is_one() {
                        if canonical_unit.is_some() {
                            return Err("multiple candidates for canonical unit".into());
                        }

                        canonical_unit = Some(unit);
                    }
                }

                match canonical_unit {
                    Some(canonical_unit) => Ok(canonical_unit.clone()),
                    None => Err("no canonical unit".into()),
                }
            }
        }
    }

    /// Find factor for unit.
    pub fn unit_factor(&self, unit: &str) -> Result<Number, String> {
        if !self.prefixes.is_empty() {
            for (prefix, prefix_factor) in &self.prefixes {
                for (unit_, unit_factor) in &self.units {
                    let unit_ = format!("{}{}", prefix, unit_);
                    if unit == unit_ {
                        let factor = prefix_factor.multiply(*unit_factor)?;
                        return Ok(factor.into());
                    }
                }
            }
        } else {
            for (unit_, unit_factor) in &self.units {
                if unit == unit_ {
                    return Ok(*unit_factor);
                }
            }
        }

        Err(format!("unsupported unit: {}", unit))
    }
}

impl Coerce for ScalarSchema {
    fn coerce(&self, expression: Expression) -> Result<Expression, String> {
        if let Some(_validator) = &self.validator {
            // TODO
        }

        let scalar = Scalar::new_from_expression(expression, self)?;
        Ok(scalar.into())
    }

    fn coerce_option(&self, mut expression: Option<Expression>) -> Result<Option<Expression>, String> {
        if expression.is_none() && self.default.is_some() {
            expression = self.default.clone();
        }

        Ok(match expression {
            Some(expression) => Some(self.coerce(expression)?),
            None => expression,
        })
    }
}

impl TryFrom<Expression> for ScalarSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let data_kind = get_string_option(map, "data_kind")?;
                let units = get_map(map, "units")?;
                let canonical_unit = get_string_option(map, "canonical_unit")?;
                let prefixes = get_map(map, "prefixes")?;
                let default = map.into_get("default").cloned();
                let validator = map.into_get("validator").cloned();

                Ok(Self::new(data_kind, units, canonical_unit, prefixes, default, validator))
            }

            _ => Err(format!("scalar schema not a map: {}", expression.type_name())),
        }
    }
}

impl Into<Expression> for ScalarSchema {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        if let Some(data_kind) = self.data_kind {
            map.insert("data_kind".into(), data_kind.into());
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

        map.into()
    }
}

fn get_string_option(map: &Map, name: &'static str) -> Result<Option<String>, String> {
    match map.into_get(name) {
        Some(Expression::Text(text)) => Ok(Some(text.clone())),
        Some(value) => Err(format!("scalar schema {:?} key not a string: {}", name, value.type_name())),
        None => Ok(None),
    }
}

fn get_map(map: &Map, name: &'static str) -> Result<BTreeMap<String, Number>, String> {
    match map.into_get(name) {
        Some(Expression::Map(map)) => {
            let mut result = BTreeMap::default();

            for (key, value) in &map.map().inner {
                let Expression::Text(key) = key else {
                    return Err(format!("scalar schema {:?} key has non-string map key: {}", name, key.type_name()));
                };

                let value =
                    value.try_into().map_err(|error| format!("scalar schema {:?} key map value: {}", name, error))?;

                result.insert(key.clone(), value);
            }

            Ok(result)
        }

        Some(value) => return Err(format!("scalar schema {:?} key not a map: {}", name, value.type_name())),

        None => Ok(Default::default()),
    }
}
