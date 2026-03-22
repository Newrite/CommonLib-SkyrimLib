---
trigger: always_on
description: Rust primitive types and crate rules for libskyrim.
---

## Forbidden
- `std::` anything — project is #![no_std]
- `static mut` — use Later<T> or RacyCell<T>
- Raw libc file pointers — use cstd::io::File
- Hardcoded VariantIDs inline — always import from offsets module
- `bytemuck::Zeroable` on ANY polymorphic type (vtable pointer = CTD)

## Required Substitutions
| C++ Pattern                           | Rust Equivalent                              |
|---------------------------------------|----------------------------------------------|
| Forward declaration                   | `core_util::abstract_type!` in new .rs file  |
| String literal for FFI                | `core_util::cstr!("str")`                    |
| Wide string                           | `core_util::wcstr!("str")`                   |
| Global singleton                      | `Later<T>` or `RacyCell<T>`                  |
| File I/O                              | `cstd::io::File::open()`                     |
| EnumSet / bitmask                     | `bitflags::bitflags!` (v2.11.0)              |
| WinAPI                                | `windows-sys` (v0.61.2) only                 |
| `static RetType Foo::Bar(args)`       | `pub fn bar(args) -> RetType` (no &self)     |
| `std::int8_t` / `std::int32_t`        | `i8` / `i32` (not `c_int`)                   |
| `const char*` return                  | `*const u8` or `BSFixedString`               |
| C++ inheritance (primary base)        | `inherit!(Child : Parent)`                   |
| C++ inheritance (mixin)               | `inherit!(Child => Mixin, field_name)`        |

## Mandatory Macro Patterns

### Virtual functions
Every `virtual RetType Name(Args)` in a C++ class:
```rust
virtual_method! {
    pub const VFUNC_NAME: usize = INDEX; // 0-based vtable position
    pub fn method_name(&self, arg: Type) -> RetType
}
```

### RELOCATION_ID methods from .cpp
Every `static REL::Relocation<func_t> func{ RELOCATION_ID(se, ae) }`:
```rust
// SE: 37787  AE: 38736
relocation_func! {
    pub fn method_name(&mut self, arg: Type) -> RetType => VariantID::new(se, ae, 0)
}
```
Static C++ methods (no `this`): same macro but no `&self`/`&mut self`.

### RelocateVirtual from .cpp
`RelocateVirtual<...>(0xSE_IDX, 0xAE_IDX, this, args)` — record BOTH indices:
```rust
// vtbl SE: 0xA6, AE: 0xA8
virtual_method! {
    pub const VFUNC_DRAW_WEAPON: usize = INDEX;
    pub fn draw_weapon_magic_hands(&mut self, draw: bool)
}
```

### Global game variables / singletons
```rust
// Reference variant
relocation_variable! {
    pub fn get_singleton() -> &'static MyType => VariantID::new(se, ae, vr)
}
// Pointer variant (double-deref)
relocation_variable! {
    pub fn get_singleton() -> *mut MyType => VariantID::new(se, ae, vr), is_ptr
}
```

### Versioned field access (RUNTIME_DATA_ACCESSOR)
```rust
runtime_data_accessor! {
    pub fn get_runtime_data() -> RuntimeData {
        se_ae: 0x1B8,
        vr: 0x230
    }
}
```

## Engine Utility Traits

| C++ Pattern | Rust Translation |
|---|---|
| `BSCRC32_<Type>` specialization | `impl BSTHash for Type` — requires `T: Copy + Sized + bytemuck::NoUninit` to avoid hashing padding bytes |
| `operator==` / `std::equal_to<T>` | `impl PartialEq for Type` |
| `operator!=` | covered by `PartialEq` automatically via `#[derive]` or manual impl |

If a C++ header includes `RE/C/CRC.h`, always check the bottom of the file
for `BSCRC32_` specializations and translate them.
