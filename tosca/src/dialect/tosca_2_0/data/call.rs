use super::{super::dialect::*, expression::*};

use {
    compris::{annotate::*, normal::*},
    kutil::{
        cli::depict::*,
        std::{immutable::*, iter::*},
    },
    std::{cmp::*, fmt, hash::*, io},
};

//
// Call
//

/// Call.
#[derive(Clone, Debug)]
pub struct Call<AnnotatedT> {
    /// Plugin name.
    pub plugin: ByteString,

    /// Function name.
    pub function: Text<AnnotatedT>,

    /// Arguments.
    pub arguments: Vec<Expression<AnnotatedT>>,

    /// True if eagerly evaluated.
    ///
    /// Note that TOSCA 2.0 does not have a way to represent eager calls.
    pub eager: bool,
}

impl<AnnotatedT> Call<AnnotatedT> {
    /// Constructor.
    pub fn new(
        plugin: ByteString,
        function: Text<AnnotatedT>,
        arguments: Vec<Expression<AnnotatedT>>,
        eager: bool,
    ) -> Self {
        Self { eager, plugin, function, arguments }
    }

    /// True if native.
    pub fn is_native(&self) -> bool {
        self.plugin == DIALECT_ID
    }

    /// Constructor.
    ///
    /// Note that TOSCA 2.0 does not have a way to represent eager calls.
    pub fn new_native(function: Text<AnnotatedT>, arguments: Vec<Expression<AnnotatedT>>) -> Self {
        Self::new(DIALECT_ID, function, arguments, false)
    }

    /// Constructor.
    pub fn new_native_static(function: &'static str) -> Self
    where
        AnnotatedT: Default,
    {
        Self::new(DIALECT_ID, ByteString::from_static(function).into(), Default::default(), false)
    }
}

impl<AnnotatedT> Annotated for Call<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        self.function.annotated.annotations()
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        self.function.annotated.annotations_mut()
    }
}

impl<AnnotatedT> Depict for Call<AnnotatedT> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        if self.eager {
            context.theme.write_delimiter(writer, '*')?;
        };
        context.theme.write_name(writer, &self.plugin)?;
        context.theme.write_delimiter(writer, ':')?;
        context.theme.write_name(writer, &self.function)?;
        context.theme.write_delimiter(writer, '(')?;

        let child_context = &context.child().with_format(DepictionFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.depict(writer, child_context)?;
            if !last {
                context.theme.write_delimiter(writer, ',')?;
            }
        }

        context.theme.write_delimiter(writer, ')')
    }
}

impl<AnnotatedT> fmt::Display for Call<AnnotatedT> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.eager {
            write!(formatter, "*")?;
        }

        write!(formatter, "{}:{}(", self.plugin, self.function)?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            fmt::Display::fmt(argument, formatter)?;
            if !last {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}

impl<AnnotatedT> PartialEq for Call<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.plugin == other.plugin) && (self.function == other.function) && (self.arguments == other.arguments)
    }
}

impl<AnnotatedT> Eq for Call<AnnotatedT> {}

impl<AnnotatedT> PartialOrd for Call<AnnotatedT> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.plugin.partial_cmp(&other.plugin) {
            Some(Ordering::Equal) => match self.function.partial_cmp(&other.function) {
                Some(Ordering::Equal) => self.arguments.partial_cmp(&other.arguments),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Ord for Call<AnnotatedT> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.plugin.cmp(&other.plugin) {
            Ordering::Equal => match self.function.cmp(&other.function) {
                Ordering::Equal => self.arguments.cmp(&other.arguments),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl<AnnotatedT> Hash for Call<AnnotatedT> {
    fn hash<HasherT: Hasher>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.plugin.hash(state);
        self.function.hash(state);
        self.arguments.hash(state);
    }
}

impl<AnnotatedT> Into<floria::Call> for Call<AnnotatedT> {
    fn into(self) -> floria::Call {
        let arguments: Vec<_> = self.arguments.into_iter().map(|argument| argument.into()).collect();
        floria::Call::new(self.plugin, self.function.inner, arguments, self.eager)
    }
}

impl<AnnotatedT> Into<floria::Expression> for Call<AnnotatedT> {
    fn into(self) -> floria::Expression {
        floria::Expression::Call(self.into())
    }
}
