use super::{super::super::super::grammar::*, entity_kind::*};

use {
    compris::annotate::*,
    kutil::{cli::depict::*, std::error::*},
};

//
// MyNodeType
//

///
#[derive(Clone, Debug, Depict)]
pub struct MyNodeType {
    ///
    pub name: String,

    ///
    #[depict(option)]
    pub derived_from: Option<FullName>,

    ///
    #[depict(as(debug))]
    pub completion: Completion,
}

impl MyNodeType {
    ///
    pub fn new(name: &str, derived_from: Option<&'static str>) -> Self {
        Self {
            name: name.to_string(),
            derived_from: derived_from.map(|name| Name::from(name).into()),
            completion: Default::default(),
        }
    }
}

impl Entity for MyNodeType {
    fn completion(&self) -> Completion {
        self.completion
    }

    fn complete(
        &mut self,
        depot: &mut Depot,
        source_id: &SourceID,
        callstack: &mut CallStack,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion == Completion::Incomplete);
        self.completion = Completion::Cannot;

        if let Some(derived_from) = &self.derived_from {
            match depot.get_complete_entity_next::<MyNodeType, _, _>(
                NODE_TYPE,
                derived_from,
                source_id,
                callstack,
                &mut errors.to_error_recipient(),
            )? {
                Some(parent) => tracing::info!("inheriting {} from {}", self.name, parent.name),
                None => return Ok(()),
            }
        }

        if let Ok(neighbor) =
            depot.get_entity::<MyNodeType, WithoutAnnotations>(NODE_TYPE, &Name::from_static("child").into(), source_id)
        {
            tracing::info!("child: {}", neighbor.name);
        }

        self.completion = Completion::Complete;
        Ok(())
    }
}
