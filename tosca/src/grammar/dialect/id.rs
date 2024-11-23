use {compris::normal::*, kutil::std::immutable::*};

//
// DialectID
//

/// Dialect ID.
pub type DialectID = ByteString;

/// Gets the dialect ID from `tosca_definitions_version`.
pub fn get_dialect_id<AnnotatedT>(variant: &Variant<AnnotatedT>) -> Option<&DialectID>
where
    AnnotatedT: Default,
{
    variant.into_get("tosca_definitions_version").and_then(|version| match version {
        Variant::Text(version) => Some(&version.inner),
        _ => None,
    })
}
