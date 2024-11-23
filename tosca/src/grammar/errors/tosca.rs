use super::{
    cyclical_derivation::*, missing_required::*, name_reused::*, number_overflow::*, source_not_loaded::*,
    undeclared::*, unknown_type::*, unsupported_dialect::*, unsupported_source::*, wrong_type::*,
};

use {
    compris::{annotate::*, normal::*, parse::*, resolve::*},
    kutil::cli::depict::*,
    read_url::*,
    std::fmt,
    thiserror::*,
};

//
// ToscaError
//

/// TOSCA error.
#[derive(Debug, Depict, Error)]
#[depict(variant = false)]
pub enum ToscaError<AnnotatedT> {
    /// Source not loaded.
    #[error("source not loaded: {0}")]
    #[depict(as(depict))]
    SourceNotLoaded(#[from] SourceNotLoadedError<AnnotatedT>),

    /// Unsupported source.
    #[error("unsupported source: {0}")]
    #[depict(as(depict))]
    UnsupportedSource(#[from] UnsupportedSourceError),

    /// Unsupported dialect.
    #[error("unsupported dialect: {0}")]
    #[depict(as(depict))]
    UnsupportedDialect(#[from] UnsupportedDialectError<AnnotatedT>),

    /// URL.
    #[error("URL: {0}")]
    URL(#[from] UrlError),

    /// Parse.
    #[error("parse: {0}")]
    #[depict(as(depict))]
    Parse(#[from] ParseError),

    /// Resolve.
    #[error("resolve: {0}")]
    #[depict(as(depict))]
    Resolve(#[from] ResolveError<AnnotatedT>),

    /// Malformed.
    #[error("malformed: {0}")]
    #[depict(as(depict))]
    Malformed(#[from] MalformedError<AnnotatedT>),

    /// Invalid key.
    #[error("invalid key: {0}")]
    #[depict(as(depict))]
    InvalidKey(#[from] InvalidKeyError<AnnotatedT>),

    /// Malformed.
    #[error("malformed: {0}")]
    #[depict(as(depict))]
    IncompatibleVariantType(#[from] IncompatibleVariantTypeError<AnnotatedT>),

    /// Name reused.
    #[error("named reused: {0}")]
    #[depict(as(depict))]
    NameReused(#[from] NameReusedError<AnnotatedT>),

    /// Cyclical derivation.
    #[error("cyclical derivation: {0}")]
    #[depict(as(depict))]
    CyclicalDerivation(#[from] CyclicalDerivationError<AnnotatedT>),

    /// Unknown type.
    #[error("unknown type: {0}")]
    #[depict(as(depict))]
    UnknownType(#[from] UnknownTypeError<AnnotatedT>),

    /// Wrong type.
    #[error("wrong type: {0}")]
    #[depict(as(depict))]
    WrongType(#[from] WrongTypeError<AnnotatedT>),

    /// Undeclared.
    #[error("undeclared: {0}")]
    #[depict(as(depict))]
    Undeclared(#[from] UndeclaredError<AnnotatedT>),

    /// Missing required.
    #[error("missing required: {0}")]
    #[depict(as(depict))]
    MissingRequired(#[from] MissingRequiredError<AnnotatedT>),

    /// Number overflow.
    #[error("number overflow")]
    #[depict(as(depict))]
    NumberOverflow(#[from] NumberOverflowError<AnnotatedT>),

    /// Store.
    #[error("store: {0}")]
    #[depict(as(depict))]
    Store(#[from] floria::StoreError),
}

impl<AnnotatedT> ToscaError<AnnotatedT> {
    /// Captured.
    pub fn captured(self) -> CapturedAnnotatedError
    where
        AnnotatedT: 'static + Annotated + fmt::Debug + Send + Sync,
    {
        Box::new(self)
    }

    /// Into different [Annotated] implementation.
    pub fn into_annotated<NewAnnotationsT>(self) -> ToscaError<NewAnnotationsT>
    where
        AnnotatedT: Annotated + Default,
        NewAnnotationsT: Annotated + Default,
    {
        match self {
            Self::SourceNotLoaded(source_not_loaded) => source_not_loaded.into_annotated().into(),
            Self::UnsupportedSource(unsupported_source) => unsupported_source.into(),
            Self::UnsupportedDialect(unsupported_dialect) => unsupported_dialect.into_annotated().into(),
            Self::URL(url) => url.into(),
            Self::Parse(parse) => parse.into(),
            Self::Resolve(resolve) => resolve.into_annotated().into(),
            Self::Malformed(malformed) => malformed.into_annotated().into(),
            Self::InvalidKey(invalid_key) => invalid_key.into_annotated().into(),
            Self::IncompatibleVariantType(incompatible_variant_type) => {
                incompatible_variant_type.into_annotated().into()
            }
            Self::NameReused(name_reused) => name_reused.into_annotated().into(),
            Self::CyclicalDerivation(cyclical_derivation) => cyclical_derivation.into_annotated().into(),
            Self::UnknownType(unknown_type) => unknown_type.into_annotated().into(),
            Self::WrongType(wrong_type) => wrong_type.into_annotated().into(),
            Self::Undeclared(undeclared) => undeclared.into_annotated().into(),
            Self::MissingRequired(missing_required) => missing_required.into_annotated().into(),
            Self::NumberOverflow(number_overflow) => number_overflow.into_annotated().into(),
            Self::Store(store) => store.into(),
        }
    }
}

// Delegated

impl<AnnotatedT> Annotated for ToscaError<AnnotatedT>
where
    AnnotatedT: Annotated,
{
    fn can_have_annotations() -> bool {
        AnnotatedT::can_have_annotations()
    }

    fn annotations(&self) -> Option<&Annotations> {
        match self {
            Self::SourceNotLoaded(source_not_loaded) => source_not_loaded.annotations(),
            Self::UnsupportedSource(_) => None,
            Self::UnsupportedDialect(unsupported_dialect) => unsupported_dialect.annotations(),
            Self::URL(_) => None,
            Self::Parse(_) => None,
            Self::Resolve(resolve) => resolve.annotations(),
            Self::Malformed(malformed) => malformed.annotations(),
            Self::InvalidKey(invalid_key) => invalid_key.annotations(),
            Self::IncompatibleVariantType(incompatible_variant_type) => incompatible_variant_type.annotations(),
            Self::NameReused(name_reused) => name_reused.annotations(),
            Self::CyclicalDerivation(cyclical_derivation) => cyclical_derivation.annotations(),
            Self::UnknownType(unknown_type) => unknown_type.annotations(),
            Self::WrongType(wrong_type) => wrong_type.annotations(),
            Self::Undeclared(undeclared) => undeclared.annotations(),
            Self::MissingRequired(missing_required) => missing_required.annotations(),
            Self::NumberOverflow(number_overflow) => number_overflow.annotations(),
            Self::Store(_) => None,
        }
    }

    fn annotations_mut(&mut self) -> Option<&mut Annotations> {
        match self {
            Self::SourceNotLoaded(source_not_loaded) => source_not_loaded.annotations_mut(),
            Self::UnsupportedSource(_) => None,
            Self::UnsupportedDialect(unsupported_dialect) => unsupported_dialect.annotations_mut(),
            Self::URL(_) => None,
            Self::Parse(_) => None,
            Self::Resolve(resolve) => resolve.annotations_mut(),
            Self::Malformed(malformed) => malformed.annotations_mut(),
            Self::InvalidKey(invalid_key) => invalid_key.annotations_mut(),
            Self::IncompatibleVariantType(incompatible_variant_type) => incompatible_variant_type.annotations_mut(),
            Self::NameReused(name_reused) => name_reused.annotations_mut(),
            Self::CyclicalDerivation(cyclical_derivation) => cyclical_derivation.annotations_mut(),
            Self::UnknownType(unknown_type) => unknown_type.annotations_mut(),
            Self::WrongType(wrong_type) => wrong_type.annotations_mut(),
            Self::Undeclared(undeclared) => undeclared.annotations_mut(),
            Self::MissingRequired(missing_required) => missing_required.annotations_mut(),
            Self::NumberOverflow(number_overflow) => number_overflow.annotations_mut(),
            Self::Store(_) => None,
        }
    }
}

impl<AnnotatedT> DynAnnotatedError for ToscaError<AnnotatedT> where
    AnnotatedT: 'static + Annotated + fmt::Debug + Send + Sync
{
}

// Conversions

impl<AnnotatedT> Into<CapturedAnnotatedError> for ToscaError<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + fmt::Debug + Send + Sync,
{
    fn into(self) -> CapturedAnnotatedError {
        self.captured()
    }
}
