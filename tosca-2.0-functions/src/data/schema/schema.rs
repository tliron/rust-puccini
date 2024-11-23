use super::{coerce::*, value::*};

use floria_plugin_sdk::data::*;

//
// Schema
//

/// Schema.
#[derive(Clone, Debug, Default)]
pub struct Schema {
    /// Values.
    pub values: Vec<ValueSchema>,
}

impl Schema {
    /// Root schema.
    pub fn root(&self) -> Result<&ValueSchema, String> {
        self.get(0)
    }

    /// Get a schema. Follow references.
    pub fn get(&self, reference: SchemaReference) -> Result<&ValueSchema, String> {
        match self.values.get(reference as usize) {
            Some(schema) => match schema {
                ValueSchema::Reference(reference) => self.get(*reference),
                _ => Ok(schema),
            },

            None => Err(format!("schema value not found: {}", reference)),
        }
    }
}

impl Coerce for Schema {
    fn coerce(&self, expression: Expression) -> Result<Expression, String> {
        self.root()?.coerce(expression)
    }

    fn coerce_option(&self, expression: Option<Expression>) -> Result<Option<Expression>, String> {
        self.root()?.coerce_option(expression)
    }
}

impl TryFrom<Expression> for Schema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::List(list_resource) => {
                let list = list_resource.list();

                let mut schemas = Vec::with_capacity(list.inner.len());
                for item in &list.inner {
                    schemas.push(ValueSchema::try_from(item.clone())?);
                }

                Ok(Schema { values: schemas })
            }

            _ => Ok(Schema { values: vec![ValueSchema::try_from(expression)?] }),
        }
    }
}
