use super::{coerce::*, primitive::*, scalar::*, r#struct::*};

use floria_plugin_sdk::data::*;

/// Schema reference.
pub type SchemaReference = u64;

//
// ValueSchema
//

/// Value schema.
#[derive(Clone, Debug)]
pub enum ValueSchema {
    Reference(SchemaReference),
    Primitive(PrimitiveSchema),
    Scalar(ScalarSchema),
    Struct(StructSchema),
}

impl Coerce for ValueSchema {
    fn coerce(&self, expression: Expression) -> Result<Expression, String> {
        match self {
            Self::Primitive(primitive) => primitive.coerce(expression),
            Self::Scalar(scalar) => scalar.coerce(expression),
            Self::Struct(struct_) => struct_.coerce(expression),

            _ => Ok(expression),
        }
    }

    fn coerce_option(&self, expression: Option<Expression>) -> Result<Option<Expression>, String> {
        match self {
            Self::Primitive(primitive) => primitive.coerce_option(expression),
            Self::Scalar(scalar) => scalar.coerce_option(expression),
            Self::Struct(struct_) => struct_.coerce_option(expression),

            _ => Ok(expression),
        }
    }
}

impl TryFrom<Expression> for ValueSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        if let Expression::Map(map_resource) = &expression {
            let map = map_resource.map();

            let Some(kind) = map.into_get("kind") else {
                return Err("value schema missing \"kind\" key".into());
            };

            let Expression::Text(kind) = kind else {
                return Err(format!("value schema \"kind\" key not a string: {}", kind.type_name()));
            };

            match kind.as_str() {
                "scalar" => {
                    let scalar = ScalarSchema::try_from(expression)?;
                    return Ok(Self::Scalar(scalar));
                }

                "struct" => {
                    let r#struct = StructSchema::try_from(expression)?;
                    return Ok(Self::Struct(r#struct));
                }

                _ => {
                    let primitive: PrimitiveSchema = expression.try_into()?;
                    return Ok(Self::Primitive(primitive));
                }
            }
        }

        match expression {
            Expression::UnsignedInteger(unsigned_integer) => Ok(Self::Reference(unsigned_integer)),
            Expression::Text(text) => Ok(Self::Primitive(PrimitiveSchema::new(text, None, None))),

            _ => Err(format!("value schema not a map, string, or unsigned integer: {}", expression.type_name())),
        }
    }
}
