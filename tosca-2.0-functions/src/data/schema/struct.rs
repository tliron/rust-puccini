use super::{coerce::*, value::*};

use {floria_plugin_sdk::data::*, std::collections::*};

//
// StructSchema
//

/// Struct schema.
#[derive(Clone, Debug, Default)]
pub struct StructSchema {
    /// Fields.
    pub fields: BTreeMap<String, ValueSchema>,

    /// Default.
    pub default: Option<Expression>,

    /// Validator.
    pub validator: Option<Expression>,
}

impl StructSchema {
    /// Constructor.
    pub fn new(
        fields: BTreeMap<String, ValueSchema>,
        default: Option<Expression>,
        validator: Option<Expression>,
    ) -> Self {
        Self { fields, default, validator }
    }
}

impl Coerce for StructSchema {
    fn coerce(&self, expression: Expression) -> Result<Expression, String> {
        if let Some(_validator) = &self.validator {
            // TODO
        }

        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let mut coerced_map = BTreeMap::default();
                for (key, value) in &map.inner {
                    let Expression::Text(name) = key else {
                        return Err(format!("struct field name not a string: {}", key.type_name()));
                    };

                    match self.fields.get(name) {
                        Some(schema) => {
                            coerced_map.insert(key.clone(), schema.coerce(value.clone())?);
                        }

                        None => return Err(format!("unsupported struct field: {:?}", name)),
                    }
                }

                Ok(coerced_map.into())
            }

            _ => Err(format!("struct not a map: {}", expression.type_name())),
        }
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

impl TryFrom<Expression> for StructSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let fields = get_map(map, "fields")?;
                let default = map.into_get("default").cloned();
                let validator = map.into_get("validator").cloned();

                Ok(Self::new(fields, default, validator))
            }

            _ => Err(format!("struct schema not a map: {}", expression.type_name())),
        }
    }
}

fn get_map(map: &Map, name: &'static str) -> Result<BTreeMap<String, ValueSchema>, String> {
    match map.into_get(name) {
        Some(Expression::Map(map_resource)) => {
            let mut map = BTreeMap::default();

            for (key, value) in &map_resource.map().inner {
                let Expression::Text(key) = key else {
                    return Err(format!("struct schema \"{}\" key has non-string map key: {}", name, key.type_name()));
                };

                let value = value
                    .clone()
                    .try_into()
                    .map_err(|error| format!("struct schema {:?} key map value: {}", name, error))?;

                map.insert(key.clone(), value);
            }

            Ok(map)
        }

        Some(value) => return Err(format!("struct schema {:?} key not a map: {}", name, value.type_name())),

        None => Ok(Default::default()),
    }
}
