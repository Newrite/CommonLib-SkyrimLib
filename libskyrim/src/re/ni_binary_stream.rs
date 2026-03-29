// Pointer-only partial translation of C++ `RE::NiBinaryStream`.
core_util::abstract_type! { pub type NiBinaryStream; }

// TODO: `NiBinaryStream.h` also exposes RTTI/VTABLE, nested `BufferInfo`, and
// the virtual I/O surface. End state: extend this matching file with the full
// source-backed stream translation if a consumer needs more than a raw pointer.
