use super::super::call::*;

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::{utils::*, *},
        std::{immutable::*, iter::*},
    },
    std::{cmp::*, collections::*, fmt, hash::*, io},
};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug)]
pub enum Expression<AnnotatedT> {
    /// Literal.
    Literal(Variant<AnnotatedT>),

    /// List.
    List(Vec<Expression<AnnotatedT>>),

    /// Map.
    Map(BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>),

    /// Call.
    Call(Call<AnnotatedT>),
}

impl<AnnotatedT> Expression<AnnotatedT> {
    /// True if `$_apply` call.
    pub fn is_apply(&self) -> bool {
        match self {
            Expression::Call(call) => call.is_native() && (call.function.inner == "_apply"),
            _ => false,
        }
    }

    /// True if must wrap in `$_assert` call.
    pub fn must_assert(&self) -> bool {
        match self {
            Expression::Call(call) => {
                call.is_native() && !matches!(call.function.as_str(), "_evaluate" | "_assert" | "_apply" | "_schema")
            }
            _ => true,
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

    fn annotations(&self) -> Option<&Annotations> {
        match self {
            Self::Literal(literal) => literal.annotations(),
            Self::List(list) => list.iter().next().and_then(|item| item.annotations()),
            Self::Map(map) => map.iter().next().and_then(|(_key, value)| value.annotations()),
            Self::Call(call) => call.annotations(),
        }
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Literal(literal) => literal.annotations_mut(),
            Self::List(list) => list.iter_mut().next().and_then(|item| item.annotations_mut()),
            Self::Map(map) => map.iter_mut().next().and_then(|(_key, value)| value.annotations_mut()),
            Self::Call(call) => call.annotations_mut(),
        }
    }
}

impl<AnnotatedT> Depict for Expression<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self {
            Self::Literal(literal) => literal.depict(writer, context),
            Self::List(list) => depict_list(list.iter(), None, writer, context),
            Self::Map(map) => depict_map(map.iter(), None, writer, context),
            Self::Call(call) => call.depict(writer, context),
        }
    }
}

impl<AnnotatedT> Default for Expression<AnnotatedT> {
    fn default() -> Self {
        Self::Literal(Default::default())
    }
}

impl<AnnotatedT> fmt::Display for Expression<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => fmt::Display::fmt(literal, formatter),

            Self::List(list) => {
                write!(formatter, "[")?;

                for (item, last) in IterateWithLast::new(list) {
                    fmt::Display::fmt(item, formatter)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, "]")
            }

            Self::Map(map) => {
                write!(formatter, "{{")?;

                for ((key, value), last) in IterateWithLast::new(map) {
                    write!(formatter, "{}:{}", key, value)?;
                    if !last {
                        write!(formatter, ",")?;
                    }
                }

                write!(formatter, "}}")
            }

            Self::Call(call) => fmt::Display::fmt(call, formatter),
        }
    }
}

impl<AnnotatedT> PartialEq for Expression<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal == other_literal,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            (Self::Call(call), Self::Call(other_call)) => call == other_call,
            _ => false,
        }
    }
}

impl<AnnotatedT> Eq for Expression<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Expression<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal.partial_cmp(other_literal),
            (Self::List(list), Self::List(other_list)) => list.partial_cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.partial_cmp(other_map),
            (Self::Call(call), Self::Call(other_call)) => call.partial_cmp(other_call),
            _ => None,
        }
    }
}

impl<AnnotatedT> Ord for Expression<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal.cmp(other_literal),
            (Self::List(list), Self::List(other_list)) => list.cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.cmp(other_map),
            (Self::Call(call), Self::Call(other_call)) => call.cmp(other_call),

            (Self::Literal(_), _) => Ordering::Less,

            (Self::List(_), Self::Literal(_)) => Ordering::Greater,
            (Self::List(_), _) => Ordering::Less,

            (Self::Map(_), Self::Literal(_) | Self::List(_)) => Ordering::Greater,
            (Self::Map(_), _) => Ordering::Less,

            (Self::Call(_), _) => Ordering::Greater,
        }
    }
}

impl<AnnotatedT> Hash for Expression<AnnotatedT> {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Literal(literal) => {
                state.write_u8(1);
                literal.hash(state);
            }

            Self::List(list) => {
                state.write_u8(2);
                list.hash(state);
            }

            Self::Map(map) => {
                state.write_u8(3);
                map.hash(state);
            }

            Self::Call(call) => {
                state.write_u8(4);
                call.hash(state);
            }
        }
    }
}

// Conversions

impl<AnnotatedT> From<ByteString> for Expression<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: ByteString) -> Self {
        Self::Literal(string.into())
    }
}

impl<AnnotatedT> From<&'static str> for Expression<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: &'static str) -> Self {
        ByteString::from_static(string).into()
    }
}

impl<AnnotatedT> From<u64> for Expression<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(unsigned_integer: u64) -> Self {
        Self::Literal(unsigned_integer.into())
    }
}

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(variant: Variant<AnnotatedT>) -> Self {
        Self::Literal(variant)
    }
}

impl<AnnotatedT> From<Vec<Expression<AnnotatedT>>> for Expression<AnnotatedT> {
    fn from(list: Vec<Expression<AnnotatedT>>) -> Self {
        Self::List(list)
    }
}

impl<AnnotatedT> From<BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>> for Expression<AnnotatedT> {
    fn from(map: BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>) -> Self {
        Self::Map(map)
    }
}

impl<AnnotatedT> From<Call<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(call: Call<AnnotatedT>) -> Self {
        Self::Call(call)
    }
}
