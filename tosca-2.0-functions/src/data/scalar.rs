use super::{comparator::*, number::*, schema::*};

use {
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, str::*},
};

const NOTATION_ERROR: &str = "scalar not \"<number> <unit>\"";

//
// Scalar
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA scalar types can be used to define scalar values along with an associated unit.
#[derive(Clone, Debug, Default)]
pub struct Scalar {
    /// Number.
    pub number: Number,

    /// Unit.
    pub unit: String,

    // Schema.
    pub schema: ScalarSchema,
}

impl Scalar {
    /// Constructor.
    pub fn new(number: Number, unit: String, schema: ScalarSchema) -> Self {
        Self { number, unit, schema }
    }

    /// Constructor.
    pub fn new_from_expression(expression: Expression, schema: &ScalarSchema) -> Result<Self, String> {
        match expression {
            Expression::Text(text) => Self::new_from_str(&text, schema),
            Expression::Custom(custom_resource) => custom_resource.custom().try_into(),
            _ => Err(format!("scalar not a string or custom: {}", expression.type_name())),
        }
    }

    /// Constructor.
    pub fn new_from_str(representation: &str, schema: &ScalarSchema) -> Result<Self, String> {
        let mut split = representation.split_whitespace();

        let Some(number) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };

        let Some(unit) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };

        if split.next().is_some() {
            return Err(NOTATION_ERROR.into());
        }

        let mut number = Number::from_str(number)?;

        let unit_factor = schema.unit_factor(unit)?;
        let unit = schema.canonical_unit()?;

        if !unit_factor.is_one() {
            number = number.multiply(unit_factor)?;
        }

        if schema.is_integer() {
            Ok(Self::new(Number::Integer(number.try_into()?), unit, schema.clone()))
        } else {
            Ok(Self::new(Number::Float(number.try_into()?), unit, schema.clone()))
        }
    }
}

impl Comparator for Scalar {
    fn comparator(&self) -> Expression {
        // TODO: canonicalize
        self.number.into()
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.number, self.unit)
    }
}

// Conversions

impl TryFrom<Expression> for Scalar {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Custom(custom_resource) => custom_resource.custom().try_into(),
            _ => Err(format!("scalar not a custom: {}", expression.type_name())),
        }
    }
}

impl TryFrom<&Custom> for Scalar {
    type Error = String;

    fn try_from(custom: &Custom) -> Result<Self, Self::Error> {
        if custom.kind != "scalar" {
            return Err(format!("custom kind not \"scalar\": {}", custom.kind));
        }

        let Expression::Map(map) = &custom.inner else {
            return Err("scalar not a map".into());
        };
        let map = &map.map().inner;

        let Some(schema) = map.get(&"schema".into()) else {
            return Err("scalar missing \"schema\" key".into());
        };
        let schema: ScalarSchema = schema.clone().try_into()?;

        let Some(number) = map.get(&"number".into()) else {
            return Err("scalar missing \"number\" key".into());
        };
        let number: Number = number.try_into().map_err(|error| format!("scalar \"number\" key value: {}", error))?;

        let Some(unit) = map.get(&"unit".into()) else {
            return Err("scalar missing \"unit\" key".into());
        };
        let Expression::Text(unit) = unit else {
            return Err("scalar \"unit\" key not a string".into());
        };
        let _ = schema.unit_factor(unit)?;

        Ok(Self::new(number, unit.clone(), schema))
    }
}

impl Into<Expression> for Scalar {
    fn into(self) -> Expression {
        let number: Expression = self.number.into();
        let map = BTreeMap::from([
            ("schema".into(), self.schema.into()),
            ("number".into(), number),
            ("unit".into(), self.unit.into()),
        ]);
        Custom::new("scalar".into(), map.into()).into()
    }
}
