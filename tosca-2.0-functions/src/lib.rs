// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

/*!
TOSCA 2.0 built-in functions.
*/

mod data;
mod dispatcher;
mod functions;

#[allow(unused_imports)]
pub use dispatcher::*;
