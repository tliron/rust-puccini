use super::expression::*;

use compris::{annotate::*, normal::*};

//
// Call
//

/// Call.
#[derive(Clone, Debug, Eq)]
pub struct Call<AnnotatedT> {
    /// Function name.
    pub name: Text<AnnotatedT>,

    /// Arguments.
    pub arguments: Vec<Expression<AnnotatedT>>,
}

impl<AnnotatedT> Call<AnnotatedT> {
    /// Constructor.
    pub fn new(name: Text<AnnotatedT>, arguments: Vec<Expression<AnnotatedT>>) -> Self {
        Self { name, arguments }
    }
}

impl<AnnotatedT> PartialEq for Call<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.name == other.name) && (self.arguments == other.arguments)
    }
}

impl<AnnotatedT> Into<floria::Call> for &Call<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Call {
        let arguments: Vec<_> = self.arguments.iter().map(|argument| argument.into()).collect();
        floria::Call::new(self.name.inner.clone(), arguments)
    }
}

impl<AnnotatedT> Into<floria::Expression> for &Call<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Expression {
        let call: floria::Call = self.into();
        call.into()
    }
}
