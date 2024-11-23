use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{cli::debug::*, std::error::*},
    std::{fmt, io},
};

//
// Range
//

/// Range.
#[derive(Clone, Debug, Default)]
pub struct Range {
    /// Lower bound.
    pub lower: u64,

    /// Upper bound.
    pub upper: RangeUpperBound,
}

impl Range {
    /// Constructor.
    pub fn new(lower: u64, upper: RangeUpperBound) -> Self {
        Self { lower, upper }
    }

    /// True if the number is in the range.
    pub fn contains(&self, number: u64) -> bool {
        match self.upper {
            RangeUpperBound::Unbounded => number >= self.lower,
            RangeUpperBound::Bounded(upper) => (number >= self.lower) && (number <= upper),
        }
    }
}

impl<AnnotatedT> Resolve<Range, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) -> ResolveResult<Range, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let give = |errors: &mut ErrorRecipientT, message: &str| {
            errors.give(MalformedError::new("range".into(), message.into()).with_annotations_from(self))
        };

        Ok(match self {
            Self::List(list) => match list.to_pair() {
                Some((lower_value, upper_value)) => match lower_value.resolve_with_errors(errors)? {
                    Some(lower) => match upper_value {
                        Self::Text(text) => {
                            if text.inner == "UNBOUNDED" {
                                return Ok(Some(Range::new(lower, RangeUpperBound::Unbounded)));
                            } else {
                                give(errors, "upper bound is not an unsigned integer or 'UNBOUNDED'")?;
                                None
                            }
                        }

                        _ => match upper_value.resolve_with_errors(errors)? {
                            Some(upper) => Some(Range::new(lower, RangeUpperBound::Bounded(upper))),
                            None => None,
                        },
                    },

                    None => None,
                },

                None => {
                    give(errors, "is not a list of length 2")?;
                    None
                }
            },

            _ => {
                errors.give(IncompatibleVariantTypeError::new(self, &["list"]))?;
                None
            }
        })
    }
}

impl Debuggable for Range {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_delimiter(writer, "[")?;
        context.theme.write_number(writer, self.lower)?;
        context.theme.write_delimiter(writer, ",")?;
        match &self.upper {
            RangeUpperBound::Unbounded => context.theme.write_symbol(writer, "Unbounded")?,
            RangeUpperBound::Bounded(upper) => context.theme.write_number(writer, upper)?,
        }
        context.theme.write_delimiter(writer, "]")
    }
}

impl fmt::Display for Range {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}, {}]", self.lower, self.upper)
    }
}

//
// RangeUpperBound
//

/// Range upper bound.
#[derive(Clone, Debug, Default)]
pub enum RangeUpperBound {
    /// Unbounded.
    #[default]
    Unbounded,

    /// Bounded.
    Bounded(u64),
}

impl fmt::Display for RangeUpperBound {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unbounded => fmt::Display::fmt("UNBOUNDED", formatter),
            Self::Bounded(bounded) => fmt::Display::fmt(bounded, formatter),
        }
    }
}
