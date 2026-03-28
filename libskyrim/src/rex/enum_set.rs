//! Source-backed `REX::EnumSet` support surface.
//!
//! CommonLibVR uses `REX::EnumSet` widely across `RE/**` headers. libskyrim's
//! shared implementation lives in `core_util`; this module re-exports that
//! implementation through the `crate::rex` namespace so translated RE files can
//! refer to the matching `REX` support type instead of introducing local stand-ins.

pub use core_util::{EnumSet, EnumSetInteger, EnumSetType};
