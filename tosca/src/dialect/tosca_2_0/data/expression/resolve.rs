use super::super::{call::*, dispatch::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::std::error::*,
    std::collections::*,
};

/// Function prefix.
pub const FUNCTION_PREFIX: &str = "$";

impl<AnnotatedT> super::Expression<AnnotatedT> {
    fn resolve_list<ErrorRecipientT>(
        list: &List<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut resolved_list = Vec::with_capacity(list.inner.len());

        for item in list {
            let item: Option<Self> = item.resolve_with_errors(errors)?;
            if let Some(item) = item {
                resolved_list.push(item);
            }
        }

        Ok(Some(Self::List(resolved_list)))
    }

    fn resolve_map<ErrorRecipientT>(
        map: &Map<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut new_map = BTreeMap::default();

        for (key, value) in map {
            let key: Option<Self> = key.resolve_with_errors(errors)?;
            if let Some(key) = key {
                let value: Option<Self> = value.resolve_with_errors(errors)?;
                if let Some(value) = value {
                    new_map.insert(key, value);
                }
            }
        }

        Ok(Some(Self::Map(new_map)))
    }
}

impl<AnnotatedT> Resolve<super::Expression<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<super::Expression<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        match self {
            Variant::Text(text) => {
                if text.inner.starts_with(FUNCTION_PREFIX) {
                    let string = &text.inner[1..];

                    // Escaped?
                    if string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let unescaped = Variant::from(string).with_annotations_from(text);
                        return Ok(Some(unescaped.into()));
                    }

                    // Call with no arguments
                    return Ok(Some(
                        Call::new(
                            Text::from(get_dispatch_name(string)).with_annotations_from(text),
                            Default::default(),
                        )
                        .into(),
                    ));
                }

                Ok(Some(self.clone().into()))
            }

            Variant::List(list) => super::Expression::resolve_list(list, errors),

            Variant::Map(map) => {
                if let Some((key, value)) = map.to_key_value_pair()
                    && let Variant::Text(key_text) = key
                    && key_text.inner.starts_with(FUNCTION_PREFIX)
                {
                    let key_string = &key_text.inner[1..];

                    // Escaped?
                    if key_string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let unescaped_key = Variant::from(key_string).with_annotations_from(key_text);
                        return super::Expression::resolve_map(&Map::from([(unescaped_key, value.clone())]), errors);
                    }

                    let mut arguments = Vec::default();
                    for argument in value.iterator() {
                        let argument: Option<super::Expression<_>> = argument.resolve_with_errors(errors)?;
                        if let Some(argument) = argument {
                            arguments.push(argument);
                        }
                    }

                    return Ok(Some(
                        Call::new(Text::from(get_dispatch_name(key_string)).with_annotations_from(key_text), arguments)
                            .into(),
                    ));
                }

                super::Expression::resolve_map(map, errors)
            }

            _ => Ok(Some(self.clone().into())),
        }
    }
}
