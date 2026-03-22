---
name: vtable-analyzer
description: Load when working with virtual functions, vtable indices, RTTI,
  skyrim_cast, or RELOCATION_IDs for methods.
---

## Virtual Method Translation

For every `virtual RetType Name(Args)` in C++:

    virtual_method! {
        pub const VFUNC_NAME: usize = 0xINDEX;  // 0-based vtable position
        pub fn method_name(&self, arg: Type) -> RetType
    }

Rules:
- Index = position in vtable (0-based), counting from the root class
- Use `&mut self` if C++ method is non-const, `&self` if const
- Adjustor thunks for mixins shift the base — index is relative to mixin's own vtable
- NEVER skip a virtual function — translate all, including pure virtuals

Pure virtual (no body in C++):

    virtual_method! {
        pub const VFUNC_NAME: usize = 0xINDEX;
        pub fn method_name(&self) -> RetType
    }
    // No relocation_func! needed — pure virtual has no implementation ID

## RelocateVirtual Pattern
`RelocateVirtual<...>(0xSE_IDX, 0xAE_IDX, this, args)` in .cpp means the
vtable index differs between SE and AE. Record BOTH as comment:

    // vtbl SE: 0xA6, AE: 0xA8
    virtual_method! {
        pub const VFUNC_DRAW_WEAPON: usize = 0xA6; // use SE index as canonical
        pub fn draw_weapon_magic_hands(&mut self, draw: bool)
    }

## RELOCATION_ID Extraction and Translation

From `.cpp`, find:

    static REL::Relocation<func_t> func{ RELOCATION_ID(12345, 67890) };

Translate as:

    // RELOCATION_ID SE: 12345, AE: 67890
    relocation_func! {
        pub fn method_name(&mut self, arg: Type) -> RetType => VariantID::new(12345, 67890, 0)
    }

From `.cpp`, find `SKSE::stl::write_vfunc<0x42, MyClass>(...)`:
→ This is a vtable hook registration, not a method ID
→ Record index 0x42 for the corresponding `virtual_method!`

VR ID rules:
- If VR ID is known → `VariantID::new(se, ae, vr)`
- If VR ID is unknown → `VariantID::new(se, ae, 0)` + comment:
  `// TODO: VERIFY — VR ID unknown`

## Static Method Translation
`RetType ClassName::Method(Args)` in .cpp with no `this`:

    // RELOCATION_ID SE: 12345, AE: 67890
    relocation_func! {
        pub fn lookup_by_handle(handle: RefHandle) -> NiPointer<Actor> => VariantID::new(12345, 67890, 0)
    }

No `&self` or `&mut self` — these are associated functions.

## Polymorphic Downcast
- Inherits `TESForm`? → implement `FormCastable`, use form cast — NOT `skyrim_cast`
- Other engine class? → implement `RttiType`, use:

      let ptr: *mut Target = unsafe { skyrim_cast::<Source, Target>(raw_ptr) };

- Both `Source` and `Target` must implement `RttiType` for `skyrim_cast` to work
