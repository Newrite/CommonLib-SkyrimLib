---
name: verify-translation
description: >
  Explicit command: /verify-translation <TypeName>.
  Deep verification of an existing Rust translation against the full C++ inheritance
  chain. Checks layout, vtable indices, RELOCATION_IDs, traits, statics, and privates.
  Automatically fixes all failures found.
---

Perform deep verification of the Rust translation for: $ARGUMENTS

## Verification Protocol

1. Read `libskyrim/src/re/<type>.rs`
2. Read `CommonLibVR/include/RE/` header for the same type
3. Read `CommonLibVR/src/RE/` .cpp for the same type — MANDATORY
4. Recursively trace the FULL C++ inheritance chain to root.
   For each class in the chain: read its .h AND .cpp before verifying.
5. For each class in the chain, run all checks below.

---


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


## Pre-Field / Pre-Verification Resolution Check

Before running any layout checks, verify that every type in the C++ inheritance
list has a corresponding Rust file. This catches blob substitutions early.

For each class in `: public Parent1, public Parent2, ...`:

1. Does `libskyrim/src/re/<parent_snake_case>.rs` exist?
   - YES -> OK, continue
   - NO -> flag immediately:

         FAIL resolution — TESFullName has no Rust file, struct field is invalid

2. Is the Rust struct field for this parent a named type (not `[u8; N]`)?
   - YES -> OK
   - NO -> flag as FAIL regardless of correct offset value:

         FAIL layout — offset 0x20: `unk020: [u8; N]` substitutes TESFullName (prohibited)

These FAIL items must be fixed before proceeding to Layout Checks.
Fix = run /translate-new or create abstract_type! stub, then replace blob with named field.


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

## Layout Checks

- Field order matches exactly (C++ offset comments vs Rust field positions)
- Every Rust field has `// 0xOFFSET` comment matching C++ source
- `size_of` assertion value matches C++ `sizeof` comment
- `offset_of` assertions exist for every mixin field and match C++ offsets
- No padding gaps — if present, `_pad: [u8; N]` field exists



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

If values are arbitrary integers (not powers of 2):
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

## VTable Checks

- Virtual function count matches between C++ and Rust
- Every vtable index is correct (0-based, continuous, no gaps)
- `RelocateVirtual(SE, AE)` calls have both indices recorded as comments
- `// override (ParentName)` comments present for all overridden virtuals
- No `virtual_method!` blocks for overrides — overridden parent virtuals must appear
  ONLY as comment documentation, not as `virtual_method!` declarations:

      // CORRECT
      // override (TESForm)
      // bool Load(TESFile* a_mod) override;  // 06

      // WRONG — flag as FAIL
      // override (TESForm)
      virtual_method! { pub const VFUNC_LOAD: usize = 0x06; ... }

  Flag as: `FAIL vtable — override virtual_method! found for Load (should be comment only)`

- No `bytemuck::Zeroable` on any polymorphic type

Rules for `virtual_method!`:
- Index = position in vtable (0-based), counting from the root class
- Use `&mut self` if C++ is non-const, `&self` if const
- NEVER write explicit `this:` parameter — macro injects it internally
- `RelocateVirtual(SE_idx, AE_idx)` -> record as `// vtbl SE: 0xA6, AE: 0xA8`

## RELOCATION_ID Checks

- Every `RELOCATION_ID(se, ae)` in .cpp has a corresponding `relocation_func!`
- Every `relocation_func!` has `// RELOCATION_ID SE: X, AE: Y` comment above it
- No hardcoded raw IDs — all use `VariantID::new(se, ae, vr)`
- No `todo!()` standing in for a real `relocation_func!`
- VR ID = 0 entries have `// TODO: VERIFY — VR ID unknown` comment

## Static and Private Method Checks

- All static methods from .cpp present as associated functions (no `&self`)
- All private helpers from .cpp present as non-pub `fn`

## Trait and Macro Checks

- `RttiType` implemented if class has RTTI
- `FormCastable` implemented if class inherits `TESForm`
- `BSTHash` / `PartialEq` implemented if C++ has `BSCRC32_` / `operator==`
- `inherit!` macro present for every base class (primary and each mixin)
- All offset/RTTI/VTable constants imported from offsets modules, not hardcoded

---


### REX Type and bitflags Checks

- No `bitflags!` used for enum where values are NOT powers of 2:

      FAIL rex  — AttackAnimation uses bitflags! but values 26,32,38 are not masks
                  Fix: replace with core_util::Enum<AttackAnimation, u8>

- No `unsafe { transmute(...) }` in getter methods:

      FAIL rex  — get_weapon_type() uses transmute — storage field type is wrong
                  Fix: change field to core_util::Enum<WeaponType, u8>

      FAIL rex  — get_form_type() uses transmute — form_type field should be
                  core_util::EnumSet<FormType, u8> per C++ REX::EnumSet<FormType, uint8_t>

- No `unsafe impl bytemuck::Zeroable` on bitflags types:

      FAIL rex  — AttackAnimation has Zeroable impl — remove it

- Every REX::EnumSet<E, U> in C++ maps to core_util::EnumSet<E, U> in Rust:

      FAIL rex  — form_type: u8 should be EnumSet<FormType, u8>
                  (C++: REX::EnumSet<FormType, uint8_t>)

      FAIL rex  — casting_type: u16 should be EnumSet<CastingType, u16>
                  (C++: REX::EnumSet<MagicSystem::CastingType, uint16_t>)

## Report Format

For each check output one line:

    OK  layout   — all fields match with correct offset comments
    OK  static   — GetSingleton present as associated fn
    OK  reloc    — SE: 37787 AE: 38736 matches relocation_func! with comment
    FAIL layout  — offset 0x10: missing `flags` field (C++ has `ObjRefFlags`)
    FAIL vtable  — virtual[42] missing, gap in index sequence
    FAIL reloc   — AddSpell has todo!() instead of relocation_func!
    FAIL override — InitializeDataComponent missing // override (BaseFormComponent)
    FAIL static  — LookupByHandle in .cpp not translated
    WARN vtable  — index uncertain, no .cpp RELOCATION_ID found
    WARN VR ID   — VariantID::new(X, Y, 0) — VR unknown, TODO comment missing

---

## After Report

- Apply fixes for ALL FAIL items automatically
- For WARN items: add `// TODO: VERIFY — <reason>` comment and continue
- Re-run all checks after fixes to confirm all FAIL resolved

---

## Inheritance Chain Reference

When tracing the chain, print this summary BEFORE reporting results:

    Inheritance chain: ChildClass
      -> Parent1 (primary, offset 0x00, size 0xSIZE)
        -> GrandParent (primary, offset 0x00)
      -> Parent2/Mixin (mixin, offset 0xOFFSET, size 0xSIZE)
    Overrides from Parent1: virtual[01] InitFoo, virtual[03] DoBar
    Overrides from Parent2: virtual[00] GetName

Recurse until root: `NiRefObject` / `TESForm` / `BaseFormComponent`.
For each parent: read .h AND .cpp — parents may have their own RELOCATION_IDs.

Downcast rules:
- Inherits `TESForm` -> `FormCastable`, NOT `skyrim_cast`
- Other engine class -> `RttiType` + `skyrim_cast::<Source, Target>(raw_ptr)`
- Both Source and Target must implement `RttiType` for `skyrim_cast` to work
