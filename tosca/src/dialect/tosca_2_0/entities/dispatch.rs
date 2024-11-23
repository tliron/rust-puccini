use kutil::std::zerocopy::*;

/// Dispatch prefix.
pub const DISPATCH_PREFIX: &str = "tosca:2.0:";

/// Dispatch name.
pub fn get_dispatch_name(name: &str) -> ByteString {
    (DISPATCH_PREFIX.to_string() + name).into()
}
