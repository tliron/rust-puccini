use super::tosca::*;

use {compris::annotate::*, kutil::std::error::*};

//
// ToscaErrorRecipientRef
//

/// Common reference type for [ErrorRecipient] of [ToscaError].
pub type ToscaErrorRecipientRef<'own> = ErrorRecipientRef<'own, ToscaError<WithAnnotations>>;
