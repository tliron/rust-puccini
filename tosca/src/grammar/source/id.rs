use {kutil::std::immutable::*, std::fmt, uuid::*};

//
// SourceID
//

/// Source ID.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceID {
    /// Universally unique identifier.
    UUID(Uuid),

    /// URL.
    URL(ByteString),

    /// Internal.
    Internal(ByteString),
}

impl SourceID {
    /// Constructor.
    pub fn url_or_default(url: Option<ByteString>) -> Self {
        match url {
            Some(url) => Self::URL(url),
            None => Default::default(),
        }
    }
}

impl Default for SourceID {
    fn default() -> Self {
        Self::UUID(Uuid::new_v4())
    }
}

impl fmt::Display for SourceID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UUID(uuid) => write!(formatter, "id:{}", uuid),
            Self::URL(url) => write!(formatter, "url:{}", url),
            Self::Internal(internal) => write!(formatter, "internal:{}", internal),
        }
    }
}

impl Into<ByteString> for SourceID {
    fn into(self) -> ByteString {
        match self {
            Self::UUID(uuid) => uuid.to_string().into(),
            Self::URL(url) => url,
            Self::Internal(internal) => internal,
        }
    }
}

impl Into<ByteString> for &SourceID {
    fn into(self) -> ByteString {
        match self {
            SourceID::UUID(uuid) => uuid.to_string().into(),
            SourceID::URL(url) => url.clone(),
            SourceID::Internal(internal) => internal.clone(),
        }
    }
}
