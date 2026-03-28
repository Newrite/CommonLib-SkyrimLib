#![allow(non_camel_case_types)]

// TODO: SOURCE - replace this opaque stand-in with the real `hkpCdVertex` translation once
// the vendored Havok headers expose a source-backed definition; source: `hkpShape.h` only
// forward-declares `hkpCdVertex`, but `hkpShape` function pointers still need the named type
// in Rust signatures.
crate::core_util::abstract_type! { pub type hkpCdVertex; }
