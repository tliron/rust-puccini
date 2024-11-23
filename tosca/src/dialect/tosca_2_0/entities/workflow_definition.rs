use {
    compris::resolve::*,
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// WorkflowDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// TODO
#[derive(Clone, Debug, Default, Depict, Resolve)]
pub struct WorkflowDefinition {}

//
// WorkflowDefinitions
//

/// Map of [WorkflowDefinition].
pub type WorkflowDefinitions = BTreeMap<ByteString, WorkflowDefinition>;
