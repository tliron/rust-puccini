use {compris::normal::*, kutil::std::zerocopy::*};

//
// DialectID
//

/// Dialect ID.
pub type DialectID = ByteString;

/// Gets the [DialectID] from "tosca_definitions_version".
pub fn get_dialect_id<AnnotatedT>(variant: &Variant<AnnotatedT>) -> Option<&DialectID>
where
    AnnotatedT: Default,
{
    variant
        .into_get("tosca_definitions_version")
        .and_then(|version| if let Variant::Text(version) = version { Some(&version.inner) } else { None })
}
