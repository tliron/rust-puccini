#![allow(dead_code)]

use {
    kutil::std::*,
    std::{cmp::*, hash::*},
};

//
// EntityKind
//

/// TOSCA 2.0 entity kind.
///
/// Only covers entities that have names.
#[derive(Clone, Debug, Display, Copy, Eq, FromStr, Hash, PartialEq)]
pub enum EntityKind {
    ///
    ArtifactType,

    ///
    CapabilityType,

    ///
    DataType,

    ///
    GroupType,

    ///
    InterfaceType,

    ///
    NodeType,

    ///
    NodeTemplate,

    ///
    PolicyType,

    ///
    RelationshipType,

    ///
    RelationshipTemplate,

    ///
    Repository,
}
