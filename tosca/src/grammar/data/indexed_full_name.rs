use super::super::name::*;

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{cli::depict::*, std::error::*},
    std::{fmt, io},
};

//
// IndexedFullName
//

/// [FullName] with optional index.
#[derive(Clone, Debug)]
pub struct IndexedFullName {
    /// Full name.
    pub full_name: FullName,

    /// Optional index.
    pub index: Option<usize>,
}

impl IndexedFullName {
    /// Constructor.
    pub fn new(full_name: FullName, index: Option<usize>) -> Self {
        Self { full_name, index }
    }
}

impl<AnnotatedT> Resolve<IndexedFullName, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<IndexedFullName, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut give =
            |message| errors.give(MalformedError::new("indexed name".into(), message).with_annotations_from(&self));

        Ok(match &self {
            Self::Text(text) => match text.inner.parse() {
                Ok(full_name) => Some(IndexedFullName::new(full_name, None)),

                Err(error) => {
                    give(error.to_string())?;
                    None
                }
            },

            Self::List(list) => match list.inner.len() {
                2 => {
                    let full_name = match list.inner.get(0).expect("first item") {
                        Variant::Text(text) => &text.inner,

                        _ => {
                            give("first item is not text".into())?;
                            return Ok(None);
                        }
                    };

                    let index = match list.inner.get(1).expect("second item") {
                        Self::Integer(integer) => integer.inner as usize,

                        Self::UnsignedInteger(unsigned_integer) => unsigned_integer.inner as usize,

                        _ => {
                            give("second item is not an integer".into())?;
                            return Ok(None);
                        }
                    };

                    match full_name.parse() {
                        Ok(full_name) => Some(IndexedFullName::new(full_name, Some(index))),

                        Err(error) => {
                            give(error.to_string())?;
                            None
                        }
                    }
                }

                _ => {
                    give("list length is not 2".into())?;
                    None
                }
            },

            _ => {
                errors.give(IncompatibleVariantTypeError::new_from(&self, &["text", "list"]))?;
                None
            }
        })
    }
}

impl Depict for IndexedFullName {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match self.index {
            Some(index) => {
                self.full_name.depict(writer, context)?;
                context.theme.write_delimiter(writer, '[')?;
                context.theme.write_number(writer, index)?;
                context.theme.write_delimiter(writer, ']')
            }

            None => self.full_name.depict(writer, context),
        }
    }
}

impl fmt::Display for IndexedFullName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.index {
            Some(index) => write!(formatter, "{}[{}]", self.full_name, index),
            None => fmt::Display::fmt(&self.full_name, formatter),
        }
    }
}
