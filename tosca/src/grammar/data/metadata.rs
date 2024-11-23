use {compris::normal::*, kutil::std::zerocopy::*, std::collections::*};

//
// Metadata
//

/// Metadata.
pub type Metadata<AnnotatedT> = BTreeMap<ByteString, Variant<AnnotatedT>>;
