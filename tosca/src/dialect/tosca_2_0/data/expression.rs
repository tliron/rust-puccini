use super::{call::*, dispatch::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, iter::*},
    },
    std::{fmt, io},
};

/// Function prefix.
pub const FUNCTION_PREFIX: &str = "$";

//
// Expression
//

/// Expression.
#[derive(Clone, Debug, Eq)]
pub enum Expression<AnnotatedT> {
    /// Literal.
    Literal(Variant<AnnotatedT>),

    /// Call.
    Call(Call<AnnotatedT>),
}

impl<AnnotatedT> Expression<AnnotatedT> {
    /// True if undefined.
    pub fn is_undefined(&self) -> bool {
        match self {
            Self::Literal(literal) => literal.is_undefined(),
            Self::Call(_) => false,
        }
    }

    /// To Floria: variant and updater.
    pub fn to_floria(&self) -> (Option<Variant<WithoutAnnotations>>, Option<floria::Call>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match self {
            Self::Literal(literal) => (Some(literal.clone().into_annotated()), None),
            Self::Call(call) => (None, Some(call.into())),
        }
    }
}

impl<AnnotatedT> Resolve<Expression<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        &self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Expression<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        if let Some((key, value)) = self.to_key_value_pair() {
            if let Variant::Text(key_text) = key {
                if key_text.inner.starts_with(FUNCTION_PREFIX) {
                    let key_string = &key_text.inner[1..];

                    // Escaped?
                    if key_string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let unescaped =
                            Variant::from([(Variant::from(key_string).with_annotations_from(key_text), value.clone())]);
                        return Ok(Some(unescaped.into()));
                    }

                    let mut arguments = Vec::default();
                    for argument in value.iterator() {
                        let argument: Option<Expression<_>> = argument.resolve_with_errors(errors)?;
                        if let Some(argument) = argument {
                            arguments.push(argument);
                        }
                    }

                    return Ok(Some(
                        Call::new(Text::from(get_dispatch_name(key_string)).with_annotations_from(key_text), arguments)
                            .into(),
                    ));
                }
            }
        } else if let Variant::Text(text) = self {
            if text.inner.starts_with(FUNCTION_PREFIX) {
                let string = &text.inner[1..];

                // Escaped?
                if string.starts_with(FUNCTION_PREFIX) {
                    // Unescape
                    let unescaped = Variant::from(string).with_annotations_from(text);
                    return Ok(Some(unescaped.into()));
                }

                return Ok(Some(
                    Call::new(Text::from(get_dispatch_name(string)).with_annotations_from(text), Default::default())
                        .into(),
                ));
            }
        }

        Ok(Some(self.clone().into()))
    }
}

impl<AnnotatedT> Depict for Expression<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(literal) => literal.depict(writer, context),

            Self::Call(call) => {
                context.separate(writer)?;
                context.theme.write_name(writer, &call.name)?;
                context.theme.write_delimiter(writer, '(')?;

                let child_context = &context.child().with_format(DepictionFormat::Compact).with_separator(false);
                for (argument, last) in IterateWithLast::new(&call.arguments) {
                    argument.depict(writer, child_context)?;
                    if !last {
                        context.theme.write_delimiter(writer, ',')?;
                    }
                }

                context.theme.write_delimiter(writer, ')')
            }
        }
    }
}

impl<AnnotatedT> Annotated for Expression<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn get_annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations(),
            Self::Call(call) => call.name.annotated.get_annotations(),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations_mut(),
            Self::Call(call) => call.name.annotated.get_annotations_mut(),
        }
    }
}

impl<AnnotatedT> Default for Expression<AnnotatedT> {
    fn default() -> Self {
        Self::Literal(Default::default())
    }
}

impl<AnnotatedT> PartialEq for Expression<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal == other_literal,
            (Self::Call(call), Self::Call(other_call)) => call == other_call,
            _ => false,
        }
    }
}

impl<AnnotatedT> fmt::Display for Expression<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => fmt::Display::fmt(literal, formatter),

            Self::Call(call) => {
                write!(formatter, "{}(", call.name)?;

                for (argument, last) in IterateWithLast::new(&call.arguments) {
                    fmt::Display::fmt(argument, formatter)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, ")")
            }
        }
    }
}

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(variant: Variant<AnnotatedT>) -> Self {
        Self::Literal(variant)
    }
}

impl<AnnotatedT> From<Call<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(call: Call<AnnotatedT>) -> Self {
        Self::Call(call)
    }
}

impl<AnnotatedT> Into<floria::Expression> for &Expression<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Expression {
        match self {
            Expression::Literal(literal) => floria::Expression::Literal(literal.clone().into_annotated()),
            Expression::Call(call) => call.into(),
        }
    }
}

/// Merge validation expressions.
pub fn complete_validation<AnnotatedT>(
    validation: &mut Option<Expression<AnnotatedT>>,
    parent_validation: Option<&Expression<AnnotatedT>>,
    struct_annotations: &mut StructAnnotations,
    parent_struct_annotations: &StructAnnotations,
) where
    AnnotatedT: Clone + Default,
{
    match validation {
        Some(my_validation) => {
            if let Some(parent_validation) = parent_validation {
                if my_validation != parent_validation {
                    *validation = Some(
                        Call::new(
                            get_dispatch_name("and").into(),
                            vec![parent_validation.clone(), my_validation.clone()],
                        )
                        .into(),
                    );
                }
            }
        }

        None => {
            if parent_validation.is_some() {
                *validation = parent_validation.cloned();
                if let Some(annotations) = parent_struct_annotations.get("validation") {
                    struct_annotations.insert("validation".into(), annotations.clone());
                }
            }
        }
    }
}
