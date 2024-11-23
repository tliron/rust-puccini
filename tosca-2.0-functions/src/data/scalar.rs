#![allow(unused)]

use super::number::*;

use {
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, str::*},
};

const NOTATION_ERROR: &str = "not \"<number> <unit>\"";

//
// Scalar
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA scalar types can be used to define scalar values along with an associated unit.
pub struct Scalar {
    /// Number.
    pub number: Number,

    /// Unit.
    pub unit: String,
}

impl Scalar {
    /// Parse.
    pub fn parse(
        representation: &str,
        integer: bool,
        units: &BTreeMap<String, Any>,
        canonical_unit: &Option<String>,
        prefixes: &BTreeMap<String, Any>,
    ) -> Result<Self, String> {
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

        let unit_factor = Self::unit_factor(unit, units, prefixes)?;
        let unit = Self::canonical_unit(units, canonical_unit, prefixes)?;

        if !unit_factor.is_one() {
            number = number.multiply(unit_factor)?;
        }

        if integer {
            let integer = number.try_into()?;
            Ok(Self { number: Number::Integer(integer), unit })
        } else {
            let float = number.try_into()?;
            Ok(Self { number: Number::Float(float), unit })
        }
    }

    /// Find factor for unit.
    pub fn unit_factor(
        unit: &str,
        units: &BTreeMap<String, Any>,
        prefixes: &BTreeMap<String, Any>,
    ) -> Result<Number, String> {
        if !prefixes.is_empty() {
            for (prefix, prefix_factor) in prefixes {
                for (unit_, unit_factor) in units {
                    let unit_ = format!("{}{}", prefix, unit_);
                    if unit == unit_ {
                        let prefix_factor = Number::try_from(prefix_factor)?;
                        let unit_factor = Number::try_from(unit_factor)?;
                        let factor = prefix_factor.multiply(unit_factor)?;
                        return Ok(factor.into());
                    }
                }
            }
        } else {
            for (unit_, unit_factor) in units {
                if unit == (unit_ as &str) {
                    let unit_factor = Number::try_from(unit_factor)?;
                    return Ok(unit_factor.into());
                }
            }
        }

        Err(format!("unsupported unit: {}", unit))
    }

    /// Find canonical unit.
    pub fn canonical_unit(
        units: &BTreeMap<String, Any>,
        canonical_unit: &Option<String>,
        prefixes: &BTreeMap<String, Any>,
    ) -> Result<String, String> {
        match canonical_unit {
            Some(canonical_unit) => {
                let _ = Self::unit_factor(canonical_unit, units, prefixes)?;
                Ok(canonical_unit.clone())
            }

            None => {
                let mut canonical_unit: Option<&String> = None;
                for (unit, factor) in units {
                    if Number::try_from(factor)?.is_one() {
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
}

impl fmt::Display for Scalar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.number, self.unit)
    }
}

impl Into<Any> for Scalar {
    fn into(self) -> Any {
        let number: Any = self.number.into();
        normal_map!(("number", number), ("unit", self.unit))
    }
}
