use super::super::{name::*, source::*};

use {kutil::std::zerocopy::*, std::fmt};

//
// ID
//

/// Entity ID.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ID {
    /// Source.
    pub source: SourceID,

    /// Name.
    pub name: Name,
}

impl ID {
    /// Constructor.
    pub fn new(source: SourceID, name: Name) -> Self {
        Self { source, name }
    }

    /// To Floria group ID.
    pub fn to_group_id(&self, prefix: ByteString) -> floria::ID {
        let floria_scope = vec!["tosca".into(), prefix, self.source.clone().into()];
        floria::ID::new_for(floria::Kind::Group, floria_scope, self.name.clone().into())
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.source, self.name)
    }
}
