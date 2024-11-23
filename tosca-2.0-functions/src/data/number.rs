use {
    floria_plugin_sdk::data::*,
    num_traits::*,
    std::{fmt, str::*},
};

const OVERFLOW_ERROR: &str = "number overflow";

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
    pub fn multiply(self, right: Self) -> Result<Self, String> {
        Ok(match (self, right) {
            (Self::Integer(left), Self::Integer(right)) => {
                let Some(product) = left.checked_mul(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Integer(product)
            }

            (Self::Integer(left), Self::UnsignedInteger(right)) => {
                let Some(right) = cast(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                let Some(product) = left.checked_mul(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Integer(product)
            }

            (Self::Integer(left), Self::Float(right)) => {
                let Some(left) = cast::<_, f64>(left) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Float(left * right)
            }

            (Self::UnsignedInteger(left), Self::UnsignedInteger(right)) => {
                let Some(product) = left.checked_mul(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::UnsignedInteger(product)
            }

            (Self::UnsignedInteger(left), Self::Integer(right)) => {
                let Some(left) = cast::<_, i64>(left) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                let Some(product) = left.checked_mul(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Integer(product)
            }

            (Self::UnsignedInteger(left), Self::Float(right)) => {
                let Some(left) = cast::<_, f64>(left) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Float(left * right)
            }

            (Self::Float(left), Self::Float(right)) => Self::Float(left * right),

            (Self::Float(left), Self::Integer(right)) => {
                let Some(right) = cast::<_, f64>(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Float(left * right)
            }

            (Self::Float(left), Self::UnsignedInteger(right)) => {
                let Some(right) = cast::<_, f64>(right) else {
                    return Err(OVERFLOW_ERROR.into());
                };
                Self::Float(left * right)
            }
        })
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Integer(0)
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
    type Err = String;

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

impl TryFrom<&Expression> for Number {
    type Error = String;

    fn try_from(expression: &Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Integer(integer) => Ok(Self::Integer(*integer)),
            Expression::UnsignedInteger(unsigned_integer) => Ok(Self::UnsignedInteger(*unsigned_integer)),
            Expression::Float(float) => Ok(Self::Float(*float)),
            _ => Err(format!("not an integer, unsigned integer, or float: {}", expression.type_name())),
        }
    }
}

impl TryInto<i64> for Number {
    type Error = String;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Self::Integer(integer) => Ok(integer),
            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(integer) => Ok(integer),
                None => Err(OVERFLOW_ERROR.into()),
            },
            Self::Float(float) => match cast(float) {
                Some(integer) => Ok(integer),
                None => Err(OVERFLOW_ERROR.into()),
            },
        }
    }
}

impl TryInto<u64> for Number {
    type Error = String;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(OVERFLOW_ERROR.into()),
            },
            Self::UnsignedInteger(unsigned_integer) => Ok(unsigned_integer),
            Self::Float(float) => match cast(float) {
                Some(unsigned_integer) => Ok(unsigned_integer),
                None => Err(OVERFLOW_ERROR.into()),
            },
        }
    }
}

impl TryInto<f64> for Number {
    type Error = String;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::Integer(integer) => match cast(integer) {
                Some(float) => Ok(float),
                None => Err(OVERFLOW_ERROR.into()),
            },
            Self::UnsignedInteger(unsigned_integer) => match cast(unsigned_integer) {
                Some(float) => Ok(float),
                None => Err(OVERFLOW_ERROR.into()),
            },
            Self::Float(float) => Ok(float),
        }
    }
}

impl Into<Expression> for Number {
    fn into(self) -> Expression {
        match self {
            Self::Integer(integer) => integer.into(),
            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.into(),
            Self::Float(float) => float.into(),
        }
    }
}
