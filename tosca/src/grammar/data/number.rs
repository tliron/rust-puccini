use super::super::errors::*;

use {
    compris::{annotate::*, normal::*},
    kutil::std::string::*,
    num_traits::*,
    std::{fmt, str::*},
};

//
// Number
//

/// Number.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    /// Integer.
    Integer(i64),

    /// Unsigned integer.
    UnsignedInteger(u64),

    /// Float.
    Float(f64),
}

impl Number {
    /// True if 1.
    pub fn is_one(self) -> bool {
        match self {
            Self::Integer(integer) => integer == 1,
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer == 1,
            Self::Float(float) => float == 1.,
        }
    }

    /// Multiply.
    pub fn multiply<AnnotatedT>(self, right: Self) -> Result<Self, NumberOverflowError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        Ok(match (self, right) {
            (Self::Integer(left), Self::Integer(right)) => {
                let Some(product) = left.checked_mul(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Integer(product)
            }

            (Self::Integer(left), Self::UnsignedInteger(right)) => {
                let Some(right) = cast(right) else {
                    return Err(NumberOverflowError::new());
                };
                let Some(product) = left.checked_mul(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Integer(product)
            }

            (Self::Integer(left), Self::Float(right)) => {
                let Some(left) = cast::<_, f64>(left) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Float(left * right)
            }

            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => {
                let Some(product) = left.checked_mul(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::UnsignedInteger(product)
            }

            (Self::UnsignedInteger(left), Self::Integer(right)) => {
                let Some(left) = cast::<_, i64>(left) else {
                    return Err(NumberOverflowError::new());
                };
                let Some(product) = left.checked_mul(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Integer(product)
            }

            (Self::UnsignedInteger(left), Self::Float(right)) => {
                let Some(left) = cast::<_, f64>(left) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Float(left * right)
            }

            (Self::Float(left), Self::Float(right)) => Self::Float(left * right),

            (Self::Float(left), Self::Integer(right)) => {
                let Some(right) = cast::<_, f64>(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Float(left * right)
            }

            (Self::Float(left), Self::UnsignedInteger(right)) => {
                let Some(right) = cast::<_, f64>(right) else {
                    return Err(NumberOverflowError::new());
                };
                Self::Float(left * right)
            }
        })
    }
}

impl fmt::Display for Number {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),
            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),
            Self::Float(float) => fmt::Display::fmt(float, formatter),
        }
    }
}

impl FromStr for Number {
    type Err = ParseError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        if let Ok(unsigned_integer) = representation.parse() {
            return Ok(Self::UnsignedInteger(unsigned_integer));
        }

        if let Ok(integer) = representation.parse() {
            return Ok(Self::Integer(integer));
        }

        if let Ok(float) = representation.parse() {
            return Ok(Self::Float(float));
        }

        Err(format!("not a number: {}", representation).into())
    }
}

impl<AnnotatedT> TryFrom<&Variant<AnnotatedT>> for Number
where
    AnnotatedT: Annotated + Clone + Default,
{
    type Error = IncompatibleVariantTypeError<AnnotatedT>;

    fn try_from(variant: &Variant<AnnotatedT>) -> Result<Self, Self::Error> {
        match variant {
            Variant::Integer(integer) => Ok(Self::Integer(integer.into())),
            Variant::UnsignedInteger(unsigned_integer) => Ok(Self::UnsignedInteger(unsigned_integer.into())),
            Variant::Float(float) => Ok(Self::Float(float.into())),
            _ => Err(IncompatibleVariantTypeError::new_from(variant, &["integer", "unsigned integer", "float"])),
        }
    }
}

impl TryInto<i64> for Number {
    type Error = NumberOverflowError<WithoutAnnotations>;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Self::Integer(integer) => Ok(integer),
            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(integer) => Ok(integer),
                None => Err(NumberOverflowError::new()),
            },
            Self::Float(float) => match cast(float) {
                Some(integer) => Ok(integer),
                None => Err(NumberOverflowError::new()),
            },
        }
    }
}

impl TryInto<u64> for Number {
    type Error = NumberOverflowError<WithoutAnnotations>;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(NumberOverflowError::new()),
            },
            Self::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer),
            Self::Float(float) => match cast(float) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(NumberOverflowError::new()),
            },
        }
    }
}

impl TryInto<f64> for Number {
    type Error = NumberOverflowError<WithoutAnnotations>;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(float) => Ok(float),
                None => Err(NumberOverflowError::new()),
            },
            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(float) => Ok(float),
                None => Err(NumberOverflowError::new()),
            },
            Self::Float(float) => Ok(float),
        }
    }
}

impl<AnnotatedT> Into<Variant<AnnotatedT>> for Number
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        match self {
            Self::Integer(integer) => integer.into(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.into(),
            Self::Float(float) => float.into(),
        }
    }
}
