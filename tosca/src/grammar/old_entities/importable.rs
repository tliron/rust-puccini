use super::super::name::*;

use kutil::std::zerocopy::*;

//
// Importable
//

/// Importable.
#[derive(Clone, Debug)]
pub struct Importable {
    /// URL.
    pub url: ByteString,

    /// Scope.
    pub scope: Option<Scope>,
}

impl Importable {
    /// Constructor.
    pub fn new(url: ByteString, scope: Option<Scope>) -> Self {
        Self { url, scope }
    }
}
