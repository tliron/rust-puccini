use super::{super::expression::*, coerce::*};

use floria_plugin_sdk::data::*;

//
// PrimitiveSchema
//

/// Primitive schema.
#[derive(Clone, Debug, Default)]
pub struct PrimitiveSchema {
    /// Kind.
    pub kind: String,

    /// Default.
    pub default: Option<Expression>,

    /// Validator.
    pub validator: Option<Expression>,
}

impl PrimitiveSchema {
    /// Constructor.
    pub fn new(kind: String, default: Option<Expression>, validator: Option<Expression>) -> Self {
        Self { kind, default, validator }
    }
}

impl Coerce for PrimitiveSchema {
    fn coerce(&self, expression: Expression) -> Result<Expression, String> {
        if let Some(_validator) = &self.validator {
            // TODO
        }

        expression.must_coerce(&self.kind)
    }

    fn coerce_option(&self, mut expression: Option<Expression>) -> Result<Option<Expression>, String> {
        if expression.is_none() && self.default.is_some() {
            expression = self.default.clone();
        }

        Ok(match expression {
            Some(expression) => {
                if let Some(_validator) = &self.validator {
                    // TODO
                }

                expression.coerce_option(&self.kind)?
            }

            None => expression,
        })
    }
}

impl TryFrom<Expression> for PrimitiveSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let Some(kind) = map.into_get("kind") else {
                    return Err("primitive schema missing \"kind\" key".into());
                };

                let Expression::Text(kind) = kind else {
                    return Err(format!("primitive schema \"kind\" key not a string: {}", kind.type_name()));
                };

                let default = map.into_get("default").cloned();
                let validator = map.into_get("validator").cloned();

                Ok(Self::new(kind.clone(), default, validator))
            }

            Expression::Text(text) => Ok(Self::new(text, None, None)),

            _ => Err(format!("primitive schema not a map or string: {}", expression.type_name())),
        }
    }
}
