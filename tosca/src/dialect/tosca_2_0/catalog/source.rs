#![allow(dead_code)]

use super::super::{super::super::grammar, entities::*};

use compris::annotate::*;

//
// Source
//

/// TOSCA 2.0 source.
#[derive(Clone, Debug)]
pub struct Source<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    ///
    pub source_id: grammar::SourceID,

    /// Scope relative to root
    pub scope: grammar::Scope,

    ///
    pub file: File<AnnotatedT>,
}
