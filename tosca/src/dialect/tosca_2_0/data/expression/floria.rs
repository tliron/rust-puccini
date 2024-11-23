use super::{super::super::dialect::*, expression::*};

use {kutil::std::immutable::*, std::collections::*};

impl<AnnotatedT> Expression<AnnotatedT> {
    /// Into Floria property value and updater.
    pub fn into_floria_property_fields(self) -> (Option<floria::Expression>, Option<floria::Expression>) {
        let floria_expression: floria::Expression = self.into();
        if floria_expression.is_literal() {
            (Some(floria_expression), None)
        } else {
            (None, Some(with_evaluate(floria_expression)))
        }
    }
}

impl<AnnotatedT> Into<floria::Expression> for Expression<AnnotatedT> {
    fn into(self) -> floria::Expression {
        match self {
            Expression::Literal(literal) => literal.into(),

            Expression::List(list) => {
                let list: Vec<floria::Expression> = list.into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Expression::Map(map) => {
                let map: BTreeMap<floria::Expression, floria::Expression> =
                    map.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Expression::Call(call) => call.into(),
        }
    }
}

fn with_evaluate(floria_expression: floria::Expression) -> floria::Expression {
    floria::Call::new(DIALECT_ID, ByteString::from_static("_evaluate"), vec![floria_expression.into()], false).into()
}
