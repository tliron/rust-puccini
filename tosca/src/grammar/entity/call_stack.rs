use super::super::{errors::*, name::*, source::*};

//
// CallStack
//

/// Call stack.
#[derive(Clone, Debug, Default)]
pub struct CallStack(pub Vec<StackFrame>);

impl CallStack {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self(vec![StackFrame::new(source_id, name)])
    }

    /// Add a frame to the top of the stack.
    pub fn add<AnnotatedT>(
        &mut self,
        source_id: SourceID,
        name: Name,
    ) -> Result<(), CyclicalDerivationError<AnnotatedT>>
    where
        AnnotatedT: Default,
    {
        tracing::trace!(source = source_id.to_string(), "add frame: {}", name);

        let entry = StackFrame::new(source_id, name.clone());
        if !self.0.contains(&entry) {
            self.0.push(entry);
            Ok(())
        } else {
            Err(CyclicalDerivationError::new(name.to_string()))
        }
    }
}

//
// StackFrame
//

/// [CallStack] frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StackFrame {
    /// Source ID.
    pub source_id: SourceID,

    /// Name.
    pub name: Name,
}

impl StackFrame {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self { source_id, name }
    }
}
