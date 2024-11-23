use super::super::{errors::*, name::*, source::*};

//
// CallStack
//

/// Call stack.
#[derive(Clone, Debug, Default)]
pub struct CallStack(pub Vec<CallStackFrame>);

impl CallStack {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self(vec![CallStackFrame::new(source_id, name)])
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

        let frame = CallStackFrame::new(source_id, name.clone());
        if !self.0.contains(&frame) {
            self.0.push(frame);
            Ok(())
        } else {
            Err(CyclicalDerivationError::new(name.to_string()))
        }
    }
}

//
// CallStackFrame
//

/// [CallStack] frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CallStackFrame {
    /// Source ID.
    pub source_id: SourceID,

    /// Name.
    pub name: Name,
}

impl CallStackFrame {
    /// Constructor.
    pub fn new(source_id: SourceID, name: Name) -> Self {
        Self { source_id, name }
    }
}
