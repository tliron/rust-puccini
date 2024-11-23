use {compris::normal::*, kutil::std::immutable::*, std::collections::*};

//
// Metadata
//

/// Metadata.
pub type Metadata<AnnotatedT> = BTreeMap<ByteString, Variant<AnnotatedT>>;
