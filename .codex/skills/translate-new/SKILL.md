---
name: translate-new
description: >
  Explicit command: /translate-new <TypeName>.
  Full C++ to Rust translation of a new RE header with no existing Rust equivalent.
  Handles vtable, RELOCATION_IDs, inheritance chain, dependencies, and mod.rs.
---

Perform a COMPLETE translation of the C++ header: $ARGUMENTS

## Pre-flight Checklist (do not skip any step)

1. Confirm `libskyrim/src/re/` does NOT contain this type yet
2. Read `CommonLibVR/include/RE/` header fully
3. Read `CommonLibVR/src/RE/` matching .cpp fully. Extract ALL of the following:
   - All `RELOCATION_ID(SE, AE)` pairs -> `relocation_func!` with comment
   - Static methods (no `this`) -> `pub fn` without `&self`
   - Private helpers not in .h -> `fn` (not `pub fn`)
   - `RelocateVirtual(SE_idx, AE_idx, ...)` -> record both vtable indices as comment
   - Global accessors / singleton patterns -> `relocation_variable!`
4. Resolve every `#include` dependency (see ## Dependency Rules below)
5. Print inheritance chain summary (see ## Inheritance Rules below) BEFORE writing any code


## Pre-Field Resolution (MANDATORY before writing any struct fields)

Before writing a single struct field, resolve EVERY type in the C++ inheritance list.
Do this after Pre-flight steps 1-3 and BEFORE the Output Structure section.

For each class in `: public Parent1, public Parent2, ...`:

1. Does `libskyrim/src/re/<parent_snake_case>.rs` exist?
   - YES -> type is ready, use it as a named field
   - NO -> go to step 2

2. Is this parent used as a struct field (layout needed, not pointer/ref)?
   - YES -> PAUSE entire translation.
     Run /translate-new for this parent first.
     Only RESUME current type after parent file exists in re/ and mod.rs.
   - NO (pointer or reference only) -> create abstract_type! stub:

         // AUTO-STUB: Full translation pending
         // TODO: VERIFY — replace with full translation when layout is needed
         core_util::abstract_type! { pub type ParentName; }

     Register in mod.rs immediately. Then RESUME.

3. After ALL parents are resolved -> proceed to write struct fields.
   Every inheritance list entry MUST be a named field with its resolved Rust type.
   NEVER substitute with `[u8; N]` — even temporarily.

This step runs ONCE per translation, in order, before any code is written.

## Nested Struct Translation Rule

When the C++ header contains nested structs or inner classes, translate them
as separate `#[repr(C)]` structs in the SAME file, BEFORE the main struct.

For each nested struct member type:
1. Check if the member's type already exists as a Rust type in `libskyrim/src/re/`
   - YES -> use it directly
   - NO -> check if it is a simple enum/primitive wrapper -> translate inline
2. Use `REX::EnumSet<T, U>` as the underlying storage type U (e.g. `u8`, `u32`)
   The enum T must already exist in Rust — use it as documentation comment if not repr-compatible

Example — C++:

    struct SkillBoost {
        REX::EnumSet<ActorValue, std::uint8_t> skill;  // 0
        std::uint8_t                           bonus;  // 1
    };
    static_assert(sizeof(SkillBoost) == 0x2);

Rust translation (ActorValue already exists in actor_values.rs):

    /// C++: `REX::EnumSet<ActorValue, u8>` — use ActorValue variants as values
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct SkillBoost {
        pub skill: u8,  // 00 (ActorValue as u8)
        pub bonus: u8,  // 01
    }
    const _: () = assert!(core::mem::size_of::<SkillBoost>() == 0x2);

Rules:
- NEVER skip nested structs — they are part of the layout
- Add `size_of` assertion for every nested struct
- If the enum type exists in Rust, use `core_util::EnumSet<E, U>` or
  `core_util::Enum<E, U>` — see ## REX Type Mapping Rule below.
  NEVER store as raw `u8` when the C++ type is REX::Enum or REX::EnumSet.
- Nested structs go BEFORE the parent struct in the file, not after
   Place immediately before `pub struct $TYPE_NAME {` — use that line as anchor.


## Output Structure (follow this order exactly)

    // 1. use imports (offsets_rtti, offsets_vtable, parent types)
    // 2. #[repr(C)] struct with ALL fields and // 0xOFFSET comments
    // 3. size_of assertion (mandatory)
    // 4. offset_of assertions for every mixin field (mandatory)
    // 5. impl RttiType (if class has RTTI)
    // 6. inherit! macro(s) — primary base, then each mixin
    // 7. impl ClassName {
    //      pub const RTTI / VTABLE
    //      // override (ParentName) for every overridden virtual
    //      virtual_method! blocks
    //      relocation_func! blocks with // RELOCATION_ID SE: X, AE: Y
    //      private fn helpers (no pub)
    // }
    // 8. impl FormCastable (if inherits TESForm)
    // 9. impl BSTHash / PartialEq (if C++ has BSCRC32_ / operator==)
    // 10. Extension Traits for mixin methods (if any mixins)


## STRICT PROHIBITION: No Blob Gap-Fill

NEVER collapse mixin structs into `unk` byte arrays. This is always wrong:

    // WRONG — prohibited
    pub unk020: [u8; 0xC8],   // collapses TESFullName + TESDescription + TESSpellList

Every mixin in the C++ inheritance list MUST appear as a named field
with its correct Rust type at its exact C++ offset:

    // CORRECT
    pub full_name: TESFullName,        // 020
    pub description: TESDescription,  // 030
    pub spell_list: TESSpellList,      // 040

If a mixin type is not yet translated -> use Dependency Rules above (stub or full).
NEVER substitute a named mixin class with `[u8; N]`.

`unk[u8; N]` is only permitted for fields that are:
- Explicitly marked unknown/reserved in the C++ source
- NOT a named class from the inheritance list
- Confirmed padding with no named fields in that range

## Completeness Rules

- ALL virtual functions translated — pure virtuals included
- ALL fields have `// 0xOFFSET` comments copied from C++
- ALL RELOCATION_IDs become `relocation_func!` — no `todo!()`
- ALL static methods present as associated functions
- ALL private helpers from .cpp present (no `pub`)
- `// override (Parent)` required on every vtable override

## Final Step

Add to `libskyrim/src/re/mod.rs` (alphabetical order):

    pub mod type_name;
    pub use type_name::*;

---



## REX Type Mapping Rule

CommonLib uses `REX::Enum<E, U>` and `REX::EnumSet<E, U>` as typed wrappers
over primitive storage. Both have Rust equivalents in `core_util`. Use them
instead of raw primitives — they preserve layout AND give type-safe API.

### Decision table

    C++ type                        | Rust type                        | When
    --------------------------------|----------------------------------|-------------------------
    REX::Enum<MyEnum, uint8_t>      | core_util::Enum<MyEnum, u8>      | single enum value in u8
    REX::Enum<MyEnum, uint32_t>     | core_util::Enum<MyEnum, u32>     | single enum value in u32
    REX::EnumSet<MyEnum, uint8_t>   | core_util::EnumSet<MyEnum, u8>   | flags set stored in u8
    REX::EnumSet<MyEnum, uint32_t>  | core_util::EnumSet<MyEnum, u32>  | flags set stored in u32
    bitflags macro (C++ side)       | bitflags! macro                  | pure power-of-2 masks

### When to use which

- `core_util::Enum<E, U>` — field stores ONE enum value, any underlying type.
  Example: `form_type: Enum<FormType, u8>` where FormType values are 0..N, not masks.

- `core_util::EnumSet<E, U>` — field stores a SET of flags, values may be combined.
  Example: `skill: EnumSet<ActorValue, u8>` — ActorValue used as enum identifier.

- `bitflags!` — ONLY when enum values are explicitly power-of-2 masks (1, 2, 4, 8...).
  Example: `RaceDataFlags`, `EquipmentFlags`.

- raw `u8` / `u32` — ONLY for truly unknown fields with no C++ type information.
  NEVER use raw primitive when C++ type is `REX::Enum` or `REX::EnumSet`.


### bitflags! guard — verify BEFORE using

Before writing `bitflags! { ... }`, check ALL enum values in C++.
bitflags! is valid ONLY when every value is a power of 2: 1, 2, 4, 8, 16, 32...

    // VALID for bitflags! — all values are powers of 2
    kNone  = 0,
    kFlag1 = 1 << 0,   // 1
    kFlag2 = 1 << 1,   // 2
    kFlag3 = 1 << 2,   // 4

    // INVALID for bitflags! — values are arbitrary integers
    kAttackLeft  = 26,
    kAttackRight = 32,
    kAttack3     = 38,   // <- NOT a bitmask

If values are arbitrary integers, NOT powers of 2:
- Use `core_util::Enum<E, U>` — NOT bitflags!
- Example: `AttackAnimation` values 26, 32, 38... are enum identifiers, not masks

Wrong (causes semantic corruption — allows combining non-combinable values):

    bitflags! {
        pub struct AttackAnimation: u8 {
            const ATTACK_LEFT  = 26;   // WRONG — 26 is not a mask bit
            const ATTACK_RIGHT = 32;   // WRONG
        }
    }

Correct:

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AttackAnimation(pub u8);   // or: core_util::Enum<AttackAnimationType, u8>

    impl_enum_type!(AttackAnimationType => u8);

### unsafe transmute in getters = wrong storage type

If you see `unsafe { core::mem::transmute(...) }` in a getter method,
it means the underlying storage field has the wrong type.

    // RED FLAG — transmute means storage type is wrong
    pub fn get_weapon_type(&self) -> WeaponType {
        unsafe { core::mem::transmute(self.weapon_type) }
    }

    // RED FLAG — transmute on form_type
    pub fn get_form_type(&self) -> FormType {
        unsafe { core::mem::transmute(self.form_type) }
    }

Fix: change the storage field to `core_util::Enum<WeaponType, u8>`,
then the getter becomes a safe `.get()` call with no transmute needed.

When auditing or translating: flag any `transmute` in a getter as a
storage-type bug to fix, not just a style issue.

### bytemuck::Zeroable on bitflags — prohibited

    // WRONG — bitflags types must not implement Zeroable
    unsafe impl bytemuck::Zeroable for AttackAnimation {}

Remove `bytemuck::Zeroable` impls from any `bitflags!` type.
Zeroable is for POD/layout types, not flag types.

### Registration requirement

After adding a new enum type to `core_util::Enum` or `core_util::EnumSet`,
register it with the appropriate macro in the enum's own file:

    // For Enum<E, U>:
    core_util::impl_enum_type!(MyEnum => u8);
    core_util::impl_enum_type!(MyEnum => u32);  // if multiple storage widths needed

    // For EnumSet<E, U>:
    core_util::impl_enumset_type!(MyEnum => u8);

### SkillBoost example (correct translation)

C++:
    struct SkillBoost {
        REX::EnumSet<ActorValue, std::uint8_t> skill;  // 0
        std::uint8_t                           bonus;  // 1
    };
    static_assert(sizeof(SkillBoost) == 0x2);

Rust (CORRECT — uses core_util::EnumSet):
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct SkillBoost {
        pub skill: core_util::EnumSet<ActorValue, u8>,  // 00
        pub bonus: u8,                                   // 01
    }
    const _: () = assert!(core::mem::size_of::<SkillBoost>() == 0x2);

    // In actor_values.rs, register:
    core_util::impl_enumset_type!(ActorValue => u8);

Rust (WRONG — loses type information):
    pub struct SkillBoost {
        pub skill: u8,  // 00 (ActorValue as u8)  ← prohibited if EnumSet exists
        pub bonus: u8,  // 01
    }

## Override Virtual Rule

When a child class overrides a virtual method from a parent, do NOT add a
`virtual_method!` block for it. The parent's Extension Trait already exposes
it via blanket impl — adding it again is duplication and causes conflicts.

Instead, add a single comment block listing all overrides grouped by parent:

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void InitItemImpl() override;                      // 13
    // bool GetPlayable() const override;                 // 19
    // const char* GetFormEditorID() const override;      // 32
    // bool SetFormEditorID(const char* a_str) override;  // 33

Format rules:
- One comment block per parent class
- Each line: `// ReturnType MethodName(Args) override; // vtable_index`
- C++ signature, not Rust — this is documentation only
- Do NOT generate `virtual_method!`, `relocation_func!`, or any Rust code for these

The only virtuals that get `virtual_method!` blocks are:
- Methods introduced FIRST in THIS class (not inherited)
- Pure virtual stubs that this class declares fresh (not overrides)

## VTable Rules

For every `virtual RetType Name(Args)` in C++:

    virtual_method! {
        pub const VFUNC_NAME: usize = 0xINDEX;  // 0-based from root class
        pub fn method_name(&self, arg: Type) -> RetType
    }

- Use `&mut self` if C++ method is non-const, `&self` if const
- NEVER skip a virtual — translate all including pure virtuals
- Pure virtuals: no `relocation_func!` needed

`RelocateVirtual(SE_idx, AE_idx)` means index differs between versions:

    // vtbl SE: 0xA6, AE: 0xA8
    virtual_method! {
        pub const VFUNC_NAME: usize = 0xA6;  // SE index is canonical
        pub fn method_name(&mut self, arg: bool)
    }

`RELOCATION_ID(se, ae)` in .cpp always becomes:

    // RELOCATION_ID SE: 12345, AE: 67890
    relocation_func! {
        pub fn method_name(&mut self, arg: Type) -> RetType
            => VariantID::new(12345, 67890, 0)
    }

VR ID unknown -> `VariantID::new(se, ae, 0)` + `// TODO: VERIFY — VR ID unknown`

Downcast rules:
- Inherits `TESForm` -> implement `FormCastable`, use form cast — NOT `skyrim_cast`
- Other engine class -> implement `RttiType`, then:
  `let ptr: *mut Target = unsafe { skyrim_cast::<Source, Target>(raw_ptr) };`
- Both Source and Target must implement `RttiType` for `skyrim_cast` to work

---

## Inheritance Rules

Print this block BEFORE writing any Rust code — do not skip even if chain seems obvious:

    Inheritance chain: ChildClass
      -> Parent1 (primary, offset 0x00, size 0xSIZE)
        -> GrandParent (primary, offset 0x00)
      -> Parent2/Mixin (mixin, offset 0xOFFSET, size 0xSIZE)
    Overrides from Parent1: virtual[01] InitFoo, virtual[03] DoBar
    Overrides from Parent2: virtual[00] GetName

Steps to build the map:
1. Parse `: public Parent1, public Parent2` from the header
2. Locate `CommonLibVR/include/RE/<Letter>/Parent.h` for each
3. Check `src/RE/<Letter>/Parent.cpp` — parents may have RELOCATION_IDs too
4. Recurse until root (`NiRefObject` / `TESForm` / `BaseFormComponent`)

Field order in the Rust struct:
1. `base: PrimaryParent` — always first, offset 0x00
2. Local fields in offset order — with `// 0xOFFSET` comments
3. Mixin fields — at their exact C++ offsets
4. `_pad: [u8; N]` — if padding gaps exist

Mixin rules:
- `offset_of!` assertion MANDATORY for every mixin field
- Use `inherit!(Child => Mixin, field)` — never call mixin methods directly on child impl
- Mixin methods exposed ONLY via Extension Trait

---

## Dependency Rules

For each `#include "RE/.../Type.h"` found in the header:

1. Does `libskyrim/src/re/type_name.rs` exist?
   - YES -> proceed, type is available
   - NO -> go to step 2

2. Is the type used as parent class OR critical struct (layout needed)?
   - YES -> PAUSE current translation, run /translate-new for it first, then RESUME
   - NO (pointer/reference only) -> create stub:

         // AUTO-STUB: Full translation pending
         // TODO: VERIFY — replace with full translation when layout is needed
         core_util::abstract_type! { pub type TypeName; }

3. Before creating stub: check if the type's .cpp contains `GetSingleton()`:
   - YES -> do NOT create abstract_type! stub
   - Instead translate at minimum: struct declaration + singleton accessor:

         relocation_variable! {
             pub fn get_singleton() -> *mut TypeName => VariantID::new(se, ae, vr), is_ptr
         }

4. ALWAYS register every new .rs file in mod.rs immediately (alphabetical order):

       pub mod type_name;
       pub use type_name::*;

   NEVER reference a type that has no corresponding entry in mod.rs.
