use super::{full_name::*, scope::*};

use {
    compris::impl_resolve_from_str,
    kutil::{
        cli::depict::*,
        std::{immutable::*, string::*},
    },
    std::{fmt, io, str::*},
};

//
// Name
//

/// Name.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(pub ByteString);

impl Name {
    /// Constructor.
    ///
    /// Note that it does not validate the name!
    pub const fn from_static(name: &'static str) -> Self {
        Self(ByteString::from_static(name))
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Convert to a [FullName].
    pub fn to_full_name(self, scope: Scope) -> FullName {
        FullName::new(scope, self)
    }
}

impl Depict for Name {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        context.theme.write_name(writer, &self.0)
    }
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.contains(":") {
            return Err("contains invalid characters".into());
        }
        Ok(Name(ByteString::from(string)))
    }
}

impl_resolve_from_str!(Name);

// Delegation

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

// Conversions

impl Into<ByteString> for Name {
    fn into(self) -> ByteString {
        self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for Name {
    fn from(string: &'static str) -> Self {
        Name::from_static(string)
    }
}

impl From<FullName> for Name {
    fn from(full_name: FullName) -> Self {
        full_name.name
    }
}

impl From<&FullName> for Name {
    fn from(full_name: &FullName) -> Self {
        full_name.name.clone()
    }
}
