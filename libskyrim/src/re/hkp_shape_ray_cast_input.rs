#![allow(non_camel_case_types)]

// TODO: SOURCE - replace this opaque stand-in with the real `hkpShapeRayCastInput`
// translation once the vendored Havok headers expose a source-backed definition; source:
// `hkpShape.h` only forward-declares the type for `CastRayImpl`.
crate::core_util::abstract_type! { pub type hkpShapeRayCastInput; }
