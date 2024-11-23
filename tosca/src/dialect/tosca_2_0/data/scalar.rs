use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, normal::*},
    kutil::std::immutable::*,
    std::{collections::*, fmt, str::*},
};

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
    pub unit: ByteString,
}

impl Scalar {
    /// Parse.
    pub fn parse<AnnotatedT>(
        representation: &str,
        integer: bool,
        units: &BTreeMap<ByteString, Variant<AnnotatedT>>,
        canonical_unit: &Option<ByteString>,
        prefixes: &BTreeMap<ByteString, Variant<AnnotatedT>>,
    ) -> Result<Self, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut split = representation.split_whitespace();

        let Some(number) = split.next() else {
            return Err(MalformedError::new("scalar".into(), "not \"<number> <unit>\"".into()).into());
        };

        let Some(unit) = split.next() else {
            return Err(MalformedError::new("scalar".into(), "not \"<number> <unit>\"".into()).into());
        };

        if split.next().is_some() {
            return Err(MalformedError::new("scalar".into(), "not \"<number> <unit>\"".into()).into());
        }

        let mut number =
            Number::from_str(number).map_err(|error| MalformedError::new("scalar".into(), error.to_string()))?;

        let unit_factor = Self::unit_factor(unit, units, prefixes)?;
        let unit = Self::canonical_unit(units, canonical_unit, prefixes)?;

        if !unit_factor.is_one() {
            number = number.multiply(unit_factor)?;
        }

        if integer {
            let integer: Result<_, NumberOverflowError<WithoutAnnotations>> = number.try_into();
            let integer = integer.map_err(|error| error.into_annotated())?;
            Ok(Self { number: Number::Integer(integer), unit })
        } else {
            let float: Result<_, NumberOverflowError<WithoutAnnotations>> = number.try_into();
            let float = float.map_err(|error| error.into_annotated())?;
            Ok(Self { number: Number::Float(float), unit })
        }
    }

    /// Find factor for unit.
    pub fn unit_factor<AnnotatedT>(
        unit: &str,
        units: &BTreeMap<ByteString, Variant<AnnotatedT>>,
        prefixes: &BTreeMap<ByteString, Variant<AnnotatedT>>,
    ) -> Result<Number, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
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

        Err(MalformedError::new("scalar".into(), format!("unsupported unit: {}", unit)).into())
    }

    /// Find canonical unit.
    fn canonical_unit<AnnotatedT>(
        units: &BTreeMap<ByteString, Variant<AnnotatedT>>,
        canonical_unit: &Option<ByteString>,
        prefixes: &BTreeMap<ByteString, Variant<AnnotatedT>>,
    ) -> Result<ByteString, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match canonical_unit {
            Some(canonical_unit) => {
                let _ = Self::unit_factor(canonical_unit, units, prefixes)?;
                Ok(canonical_unit.clone())
            }

            None => {
                let mut canonical_unit: Option<&ByteString> = None;
                for (unit, factor) in units {
                    if Number::try_from(factor)?.is_one() {
                        if canonical_unit.is_some() {
                            return Err(MalformedError::new(
                                "scalar".into(),
                                "multiple candidates for canonical unit".into(),
                            )
                            .into());
                        }

                        canonical_unit = Some(unit);
                    }
                }

                match canonical_unit {
                    Some(canonical_unit) => Ok(canonical_unit.clone()),
                    None => Err(MalformedError::new("scalar".into(), "no canonical unit".into()).into()),
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

impl<AnnotatedT> Into<Variant<AnnotatedT>> for Scalar
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let number: Variant<_> = self.number.into();
        normal_map!(("number", number), ("unit", self.unit))
    }
}
