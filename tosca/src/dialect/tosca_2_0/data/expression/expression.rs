use super::super::call::*;

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::{utils::*, *},
        std::iter::*,
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

    /// Call.
    Call(Call<AnnotatedT>),

    /// List.
    List(Vec<Expression<AnnotatedT>>),

    /// Map.
    Map(BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>),
}

impl<AnnotatedT> Expression<AnnotatedT> {
    /// True if literal.
    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,

            Self::Call(_) => false,

            Self::List(list) => {
                for item in list {
                    if !item.is_literal() {
                        return false;
                    }
                }
                true
            }

            Self::Map(map) => {
                for (key, value) in map {
                    if !key.is_literal() {
                        return false;
                    }
                    if !value.is_literal() {
                        return false;
                    }
                }
                true
            }
        }
    }

    /// To literal variant.
    ///
    /// Returns [None] if not literal.
    pub fn to_literal_variant(&self) -> Option<Variant<WithoutAnnotations>>
    where
        AnnotatedT: Annotated + Clone,
    {
        match self {
            Self::Literal(literal) => Some(literal.clone().into_annotated()),

            Self::List(list) => {
                let mut literal_list = List::new_with_capacity(list.len());

                for item in list {
                    let Some(item) = item.to_literal_variant() else {
                        return None;
                    };

                    literal_list.inner.push(item);
                }

                Some(literal_list.into())
            }

            Self::Map(map) => {
                let mut literal_map = Map::default();

                for (key, value) in map {
                    let Some(key) = key.to_literal_variant() else {
                        return None;
                    };

                    let Some(value) = value.to_literal_variant() else {
                        return None;
                    };

                    literal_map.inner.insert(key, value);
                }

                Some(literal_map.into())
            }

            _ => None,
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

            Self::List(list) => depict_list(list.iter(), None, writer, context),

            Self::Map(map) => depict_map(map.iter(), None, writer, context),
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
            Self::List(list) => list.iter().next().and_then(|item| item.get_annotations()),
            Self::Map(map) => map.iter().next().and_then(|(_key, value)| value.get_annotations()),
        }
    }

    fn get_annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::Literal(literal) => literal.get_annotations_mut(),
            Self::Call(call) => call.name.annotated.get_annotations_mut(),
            Self::List(list) => list.iter_mut().next().and_then(|item| item.get_annotations_mut()),
            Self::Map(map) => map.iter_mut().next().and_then(|(_key, value)| value.get_annotations_mut()),
        }
    }
}

impl<AnnotatedT> Default for Expression<AnnotatedT> {
    fn default() -> Self {
        Self::Literal(Default::default())
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

            Self::Call(call) => {
                state.write_u8(2);
                call.hash(state);
            }

            Self::List(list) => {
                state.write_u8(3);
                list.hash(state);
            }

            Self::Map(map) => {
                state.write_u8(4);
                map.hash(state);
            }
        }
    }
}

impl<AnnotatedT> PartialEq for Expression<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal == other_literal,
            (Self::Call(call), Self::Call(other_call)) => call == other_call,
            (Self::List(list), Self::List(other_list)) => list == other_list,
            (Self::Map(map), Self::Map(other_map)) => map == other_map,
            _ => false,
        }
    }
}

impl<AnnotatedT> Eq for Expression<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Expression<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal.partial_cmp(other_literal),
            (Self::Call(call), Self::Call(other_call)) => call.partial_cmp(other_call),
            (Self::List(list), Self::List(other_list)) => list.partial_cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.partial_cmp(other_map),
            _ => None,
        }
    }
}

impl<AnnotatedT> Ord for Expression<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Literal(literal), Self::Literal(other_literal)) => literal.cmp(other_literal),
            (Self::Call(call), Self::Call(other_call)) => call.cmp(other_call),
            (Self::List(list), Self::List(other_list)) => list.cmp(other_list),
            (Self::Map(map), Self::Map(other_map)) => map.cmp(other_map),

            (Self::Literal(_), Self::Call(_)) => Ordering::Less,
            (Self::Literal(_), Self::List(_)) => Ordering::Less,
            (Self::Literal(_), Self::Map(_)) => Ordering::Less,

            (Self::Call(_), Self::Literal(_)) => Ordering::Greater,
            (Self::Call(_), Self::List(_)) => Ordering::Less,
            (Self::Call(_), Self::Map(_)) => Ordering::Less,

            (Self::List(_), Self::Literal(_)) => Ordering::Greater,
            (Self::List(_), Self::Call(_)) => Ordering::Greater,
            (Self::List(_), Self::Map(_)) => Ordering::Less,

            (Self::Map(_), Self::Literal(_)) => Ordering::Greater,
            (Self::Map(_), Self::Call(_)) => Ordering::Greater,
            (Self::Map(_), Self::List(_)) => Ordering::Greater,
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
