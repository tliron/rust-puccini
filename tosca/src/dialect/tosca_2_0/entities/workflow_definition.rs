use {
    compris::resolve::*,
    kutil::{cli::debug::*, std::zerocopy::*},
    std::collections::*,
};

//
// WorkflowDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// TODO
#[derive(Clone, Debug, Debuggable, Default, Resolve)]
pub struct WorkflowDefinition {}

//
// WorkflowDefinitions
//

/// Map of [WorkflowDefinition].
pub type WorkflowDefinitions = BTreeMap<ByteString, WorkflowDefinition>;
