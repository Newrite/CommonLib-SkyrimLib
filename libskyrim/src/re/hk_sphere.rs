#![allow(non_camel_case_types)]

// TODO: SOURCE - replace this opaque stand-in with the real `hkSphere` layout once the
// vendored Havok headers expose a source-backed definition; source: `hkpShape.h` and
// `hkpSphereRepShape.h` only forward-declare `hkSphere`, but `hkpShape` function pointers
// still need the named type in Rust signatures.
crate::core_util::abstract_type! { pub type hkSphere; }
