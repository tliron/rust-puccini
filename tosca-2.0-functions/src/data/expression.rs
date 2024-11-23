use super::{super::dispatcher::*, comparator::*, scalar::*, timestamp::*, version::*};

use {
    floria_plugin_sdk::{data::*, dispatch_bindings::*},
    std::collections::*,
};

//
// ExpressionUtilities
//

/// Expression utilities.
pub trait ExpressionUtilities: Sized {
    /// Evaluate an expression.
    fn evaluate(self, call_site: &CallSite) -> Result<Option<Expression>, String>;

    /// Evaluate an expression.
    fn must_evaluate(self, call_site: &CallSite) -> Result<Expression, String> {
        self.evaluate(call_site)?.ok_or_else(|| "expression has no value".into())
    }

    /// Coerce into a data kind.
    ///
    /// Returns true if the expression was modified.
    fn coerce(self, data_kind: &str) -> Result<(Expression, bool), String>;

    /// If the other expression is custom, coerce into its data kind.
    ///
    /// This is often called before [comparator](ExpressionUtilities::comparator) in order to
    /// ensure compatibility between two expressions.
    fn coerce_if_custom(self, other: &Expression) -> Result<Expression, String>;

    /// Comparator.
    fn comparator(self) -> Result<Expression, String>;

    /// Coerce into a data kind.
    fn must_coerce(self, data_kind: &str) -> Result<Expression, String> {
        self.coerce(data_kind).map(|(expression, _modified)| expression)
    }

    /// Coerce into a data kind.
    ///
    /// Returns [None] if the expression was not modified.
    fn coerce_option(self, data_kind: &str) -> Result<Option<Expression>, String> {
        let (expression, modified) = self.coerce(data_kind)?;
        Ok(if modified { Some(expression) } else { None })
    }
}

impl ExpressionUtilities for Expression {
    fn evaluate(self, call_site: &CallSite) -> Result<Option<Expression>, String> {
        match self {
            Expression::List(list_resource) => {
                let list = list_resource.list();

                let mut evaluated_list = Vec::with_capacity(list.inner.len());
                for item in &list.inner {
                    if let Some(item) = item.clone().evaluate(call_site)? {
                        evaluated_list.push(item);
                    } else {
                        return Err("could not evaluate list item".into());
                    }
                }

                Ok(Some(evaluated_list.into()))
            }

            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let mut evaluated_map = BTreeMap::default();
                for (key, value) in &map.inner {
                    if let Some(key) = key.clone().evaluate(call_site)?
                        && let Some(value) = value.clone().evaluate(call_site)?
                    {
                        evaluated_map.insert(key, value);
                    } else {
                        return Err("could not evaluate map key-value pair".into());
                    }
                }

                Ok(Some(evaluated_map.into()))
            }

            Expression::Custom(custom_resource) => {
                let custom = custom_resource.custom();
                let inner = match custom.inner.clone().evaluate(call_site)? {
                    Some(inner) => inner,
                    None => return Err("could not evaluate custom inner".into()),
                };
                Ok(Some(Custom::new(custom.kind.clone(), inner).into()))
            }

            Expression::Call(call_resource) => {
                let call = call_resource.call();
                if call.plugin == "tosca_2_0" {
                    Dispatcher::dispatch(call.function.clone(), call.arguments.clone(), call_site.clone())
                } else {
                    Err("other plugin".into())
                }
            }

            _ => Ok(Some(self)),
        }
    }

    fn coerce(self, data_kind: &str) -> Result<(Expression, bool), String> {
        match data_kind {
            "string" => {
                if matches!(self, Expression::Text(_)) {
                    return Ok((self, false));
                }
            }

            "integer" => {
                if matches!(self, Expression::Integer(_) | Expression::UnsignedInteger(_)) {
                    return Ok((self, false));
                }
            }

            "float" => {
                if matches!(self, Expression::Float(_)) {
                    return Ok((self, false));
                }
            }

            "boolean" => {
                if matches!(self, Expression::Boolean(_)) {
                    return Ok((self, false));
                }
            }

            "bytes" => {
                if matches!(self, Expression::Blob(_)) {
                    return Ok((self, false));
                }
            }

            "nil" => {
                if matches!(self, Expression::Null) {
                    return Ok((self, false));
                }
            }

            "list" => {
                if matches!(self, Expression::List(_)) {
                    return Ok((self, false));
                }
            }

            "map" => {
                if matches!(self, Expression::Map(_)) {
                    return Ok((self, false));
                }
            }

            "timestamp" => {
                let timestamp: Timestamp = self.try_into()?;
                return Ok((timestamp.into(), true));
            }

            "scalar" => {
                // Note: Only supports Expression::Custom
                // You need coerce_custom for Expression::Text
                let scalar: Scalar = self.try_into()?;
                return Ok((scalar.into(), true));
            }

            "version" => {
                let version: Version = self.try_into()?;
                return Ok((version.into(), true));
            }

            _ => return Err(format!("unsupported data kind: {}", data_kind)),
        }

        let article = match data_kind {
            "integer" => "an",
            _ => "a",
        };

        Err(format!("value not {} {}: {}", article, data_kind, self.type_name()))
    }

    fn coerce_if_custom(self, other: &Expression) -> Result<Expression, String> {
        Ok(
            if let Expression::Custom(other) = other
                && !matches!(self, Expression::Custom(_))
            {
                let custom = other.custom();
                match custom.kind.as_str() {
                    "scalar" => {
                        let scalar: Scalar = custom.try_into()?;
                        let scalar = Scalar::new_from_expression(self, &scalar.schema)?;
                        scalar.into()
                    }

                    kind => {
                        let (expression, _modified) = self.coerce(kind)?;
                        expression
                    }
                }
            } else {
                self
            },
        )
    }

    fn comparator(self) -> Result<Expression, String> {
        match self {
            Expression::Integer(_) | Expression::UnsignedInteger(_) | Expression::Float(_) | Expression::Text(_) => {
                Ok(self)
            }

            Expression::Custom(custom) => {
                let custom = custom.custom();
                match custom.kind.as_str() {
                    "scalar" => {
                        let scalar: Scalar = custom.try_into()?;
                        Ok(scalar.comparator())
                    }

                    "timestamp" => {
                        let timestamp: Timestamp = custom.try_into()?;
                        Ok(timestamp.comparator())
                    }

                    "version" => {
                        let version: Version = custom.try_into()?;
                        Ok(version.comparator())
                    }

                    kind => Err(format!("unsupported custom kind: {}", kind)),
                }
            }

            _ => Err(format!("cannot compare {}", self.type_name())),
        }
    }
}
