crate::core_util::abstract_type! { pub type TESBitArrayFile; }

// TODO: The vendored CommonLib tree only forward-declares `RE::TESBitArrayFile` from
// `TESFile.h` and does not provide a standalone header, RTTI, VTABLE, or layout surface.
// Replace this opaque stand-in with a real translation once the dependency becomes
// source-backed instead of a pure forward declaration.
