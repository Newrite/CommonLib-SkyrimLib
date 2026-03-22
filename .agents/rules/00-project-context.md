---
trigger: always_on
description: Core project context for libskyrim — Skyrim RE C++ to Rust translation.
---

## Project Identity
This is `libskyrim`: a #![no_std] Rust library translating reverse-engineered
Skyrim C++ headers (CommonLibVR/CommonLibSSE format) to Rust.

CRITICAL: 100% binary compatibility with Skyrim MSVC ABI is non-negotiable.
Any memory layout error = CTD (Crash To Desktop).

## Source Locations
- C++ Headers:  CommonLibVR/include/RE/<Letter>/<Name>.h
- C++ Sources:  CommonLibVR/src/RE/<Letter>/<Name>.cpp
- Rust Output:  libskyrim/src/re/<snake_case>.rs
- Module Index: libskyrim/src/re/mod.rs
- Offsets RTTI: libskyrim/src/offsets/offsets_rtti.rs
- Offsets VT:   libskyrim/src/offsets/offsets_vtable.rs

## MANDATORY: .cpp is NOT optional
The .cpp file is the PRIMARY source for:
- All method implementations with RELOCATION_ID(SE, AE)
- Static methods (no `this`) that exist ONLY in .cpp
- Private helper methods declared nowhere in .h
- RelocateVirtual calls with two vtable indices (SE idx, AE idx)
Rule: if .cpp exists → it MUST be read before ANY code is written.

## Mandatory Macro Toolkit
Never reimplement these patterns manually — always use the provided macros:

### From `libskyrim::relocation`
- `virtual_method!`           — every virtual function in a class
- `relocation_func!`          — every RELOCATION_ID(...) method from .cpp
- `relocation_variable!`      — global singletons and game variables by Address Library ID
- `define_vtable_hook!`       — vtable function interception
- `define_call_hook!`         — CALL/JMP instruction interception

### From `libskyrim::runtime`
- `runtime_data_accessor!`    — versioned field access (SE/AE/VR offset differences)
- `runtime_pointer_accessor!` — versioned pointer field access

### From `core_util`
- `abstract_type!`   — forward-declared / opaque types (unknown layout)
- `inherit!`         — C++ inheritance (primary base and mixin variants)
- `cstr!`            — C string literals for FFI
- `wcstr!`           — wide string literals for WinAPI FFI
- `disarray!`        — static arrays with auto-captured size
- `attempt!`         — scoped `?` operator
- `Later<T>`         — once-init global (replaces `static mut`, global singletons)
- `RacyCell<T>`      — unsafe-sync cell (replaces `static mut` for mutable globals)
