// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
CSAR.
*/

mod meta;
mod version;

#[allow(unused_imports)]
pub use {meta::*, version::*};
