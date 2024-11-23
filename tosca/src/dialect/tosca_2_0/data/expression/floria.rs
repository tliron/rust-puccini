use super::super::dispatch::*;

use compris::{annotate::*, normal::*};

impl<AnnotatedT> super::Expression<AnnotatedT> {
    /// To Floria property variant and updater.
    pub fn to_floria_property_fields(&self) -> (Option<Variant<WithoutAnnotations>>, Option<floria::Call>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let flora_expression: floria::Expression = self.into();
        match flora_expression {
            floria::Expression::Literal(literal) => (Some(literal), None),
            floria::Expression::Call(call) => (None, Some(call)),
        }
    }

    /// To Floria property validator.
    pub fn to_floria_property_validator(&self) -> Option<floria::Call>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match self {
            Self::Literal(literal) => match literal {
                Variant::Boolean(_) => {
                    Some(floria::Call::new(get_dispatch_name("_literal"), vec![literal.clone().into()]))
                }

                _ => None,
            },

            Self::Call(call) => Some(call.into()),

            _ => None,
        }
    }
}

impl<AnnotatedT> Into<floria::Expression> for &super::Expression<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Expression {
        if let Some(literal) = self.to_literal_variant() {
            return floria::Expression::Literal(literal);
        }

        match self {
            super::Expression::Call(call) => call.into(),

            super::Expression::List(list) => {
                let arguments: Vec<_> = list.into_iter().map(|item| item.into()).collect();
                floria::Call::new(get_dispatch_name("_list"), arguments).into()
            }

            super::Expression::Map(map) => {
                let arguments: Vec<_> =
                    map.into_iter().map(|(key, value)| vec![key.into(), value.into()]).flatten().collect();
                floria::Call::new(get_dispatch_name("_map"), arguments).into()
            }

            _ => panic!("`into_literal_variant` should have handled this case"),
        }
    }
}
