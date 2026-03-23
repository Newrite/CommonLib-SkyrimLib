---
name: translate-partial
description: >
  Explicit command: /translate-partial <TypeName>.
  Audit and complete a partially translated Rust file against its C++ source.
  Performs full gap analysis: fields, vtable, RELOCATION_IDs, traits, statics.
---

Audit and complete the partial translation for: $ARGUMENTS

## Steps

1. Read the existing Rust file in `libskyrim/src/re/`
2. Read the corresponding C++ header in `CommonLibVR/include/RE/`
3. Read the corresponding `.cpp` file — MANDATORY even if it seems complete. Verify:
   - All `RELOCATION_ID(SE, AE)` pairs — are they all in `relocation_func!`?
   - Static methods (no `this`) — are they all present as associated functions?
   - Private helpers not in .h — are they present as non-pub `fn`?
   - `RelocateVirtual(SE, AE)` calls — are both vtable indices recorded?
   - Singleton / global accessor patterns — translated via `relocation_variable!`?


## Pre-Field Resolution (MANDATORY before gap-fill of any struct fields)

Before adding or patching ANY struct field, resolve every type in the C++
inheritance list. This runs once, before Step 4 gap analysis writes anything.

Read the C++ header inheritance declaration:
`: public TESForm, public TESFullName, public TESDescription, ...`

For each parent type in that list:

1. Does `libskyrim/src/re/<parent_snake_case>.rs` exist?
   - YES -> type is ready, use it as a named field
   - NO -> go to step 2

2. Is this parent used as a struct field (layout needed, not pointer/ref)?
   - YES -> PAUSE. Run /translate-new for this parent first.
     Only RESUME after parent file exists in re/ and mod.rs.
   - NO (pointer or reference only) -> create abstract_type! stub:

         // AUTO-STUB: Full translation pending
         // TODO: VERIFY — replace with full translation when layout is needed
         core_util::abstract_type! { pub type ParentName; }

     Register in mod.rs immediately. Then RESUME.

3. After ALL parents from the inheritance list are resolved:
   - Each must appear as a named field in the struct at its correct C++ offset
   - NEVER use `[u8; N]` to represent a named parent type
   - `// ADDED: gap-fill` is only valid for concrete named fields from C++ source


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


4. Produce a full gap analysis — list EVERYTHING missing:
   - Virtual functions present in C++ but absent in Rust
   - Member fields missing or with wrong types
   - Fields missing `// 0xOFFSET` comments
   - Missing `size_of` assertion
   - Missing `offset_of` assertions for mixin fields
   - Missing RTTI / VTable constants
   - Missing `inherit!` macro calls (primary or mixin)
   - Missing trait implementations (`FormCastable`, `RttiType`, `BSTHash`, `PartialEq`)
   - Missing `// override (ParentName)` comments for vtable overrides
   - `RELOCATION_ID` methods in .cpp missing `relocation_func!` in Rust
   - Static methods in .cpp absent in Rust
   - Private helper methods in .cpp absent in Rust
   - `relocation_func!` entries missing `// RELOCATION_ID SE: X, AE: Y` comment
   - Methods using `todo!()` instead of real `relocation_func!`

5. For each gap: add the missing piece with a comment `// ADDED: gap-fill`

6. Do NOT rewrite existing correct code — only append/patch

## STRICT PROHIBITION: No Blob Gap-Fill

NEVER collapse mixin structs into `unk` byte arrays. This is always wrong:

    // WRONG — prohibited
    pub unk020: [u8; 0xC8],   // collapses TESFullName + TESDescription + TESSpellList

Every mixin listed in the C++ inheritance declaration MUST appear as a named field
with its correct Rust type, at its exact C++ offset:

    // CORRECT
    pub full_name: TESFullName,        // 020
    pub description: TESDescription,  // 030
    pub spell_list: TESSpellList,      // 040
    pub skin_form: BGSSkinForm,        // 050
    pub biped_form: BGSBipedObjectForm,// 060
    pub keyword_form: BGSKeywordForm,  // 070
    pub attack_data: BGSAttackDataForm,// 088

If a mixin type is not yet translated:
- PAUSE and run /translate-new for it first (if layout is needed)
- OR create an `abstract_type!` stub (if pointer/ref only)
- NEVER substitute with `[u8; N]`

A `unk[u8; N]` blob is only permitted for fields that are:
- Explicitly marked as unknown/reserved in the C++ source
- Not a named class that appears in the inheritance list
- Confirmed padding with no named fields in that range

The `// ADDED: gap-fill` comment does NOT grant permission to use blobs.
It only marks genuinely missing named fields being added.



7. Run the audit checks below as final verification. Fix any remaining issues.

---

## Audit Checks

### Field-by-Field
For each C++ field comment `// 0x08 SomeType name`:
- Find corresponding Rust field
- Verify type mapping is correct
- Verify `// 0x08` comment is present on the Rust field
- Verify it lands at correct offset given preceding fields

### VTable Index Check
List all `virtual` functions in C++ in order.
Compare with `virtual_method!` indices in Rust.
Any gap in numbering = missing override or wrong index.
Verify `// override (ParentName)` present for all overrides.

### Size Check
C++ `// sizeof == 0x1B8` -> Rust `size_of` assertion must equal `0x1B8`.
Missing assertion -> flag as gap. Mixin field present -> `offset_of!` assertion required.

### RELOCATION_ID Check
For every `RELOCATION_ID(se, ae)` in .cpp:
- `relocation_func!` must exist in Rust
- `// RELOCATION_ID SE: X, AE: Y` comment must be above it
- Both SE and AE values must match exactly

### Report Format

    OK  offset 0x08  — `formID: u32` matches TESForm::formID
    OK  static       — `LookupByHandle` present as associated fn
    GAP offset 0x10  — missing `flags` field (C++ has `ObjRefFlags`)
    GAP reloc        — `AddSpell` has todo!() instead of relocation_func!
    GAP override     — `InitializeDataComponent` missing // override (Base)
    WARN vtable[42]  — index uncertain, no .cpp RELOCATION_ID found

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

## VTable Rules (Reference)

`virtual_method!` macro injects `&self` internally — NEVER write explicit `this:` parameter:

    virtual_method! {
        pub const VFUNC_NAME: usize = 0xINDEX;
        pub fn method_name(&self, arg: Type) -> RetType
    }

`RELOCATION_ID(se, ae)` in .cpp always becomes:

    // RELOCATION_ID SE: 12345, AE: 67890
    relocation_func! {
        pub fn method_name(&mut self, arg: Type) -> RetType
            => VariantID::new(12345, 67890, 0)
    }

VR ID unknown -> `VariantID::new(se, ae, 0)` + `// TODO: VERIFY — VR ID unknown`
