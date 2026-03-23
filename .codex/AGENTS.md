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

description: C++ to Rust inheritance translation rules. Always apply when working with structs.
---

---
trigger: always_on
description: C++ to Rust inheritance translation rules. Always apply when working with structs.
---

## GENERATION CHECKLIST (run for every file)

Before finishing any RE class translation, verify ALL items:

- [ ] Every struct has `#[repr(C)]`
- [ ] Every struct has a size assertion
- [ ] Every mixin field has an offset assertion
- [ ] Every field has a C++ offset comment
- [ ] **If this class has virtual methods AND is used as mixin → Extension Trait MUST be written**
- [ ] Extension Trait includes ALL virtual methods — not just some
- [ ] No mixin methods re-declared on child impl
- [ ] `derive(Debug)` added only where it makes sense (see Derive Policy below)

---

## Derive Policy

`#[derive(Debug)]` is **allowed and encouraged** for Rust-native types:
- Config structs, state enums, plugin-side utility types
- Types like `VariantID`, `VariantOffset` — add `derive(Debug)` freely

`#[derive(Debug)]` is **forbidden** for RE structs (`src/re/`):
- RE structs are raw memory mirrors of C++ objects — most fields are raw pointers
- `derive(Debug)` on a generic wrapper (e.g. `NiPointer<T>`) adds `T: Debug` bound
  that cascades through the entire codebase, forcing `derive(Debug)` everywhere
- Raw pointer fields print only addresses — useless for debugging
- If debug printing is genuinely needed for a specific RE struct — implement
  `fmt::Debug` manually for that struct only, do not use `derive`

---

## Layout Rules

- ALL structs: `#[repr(C)]`
- Single-field wrappers (smart pointers, handles): `#[repr(transparent)]` instead of `#[repr(C)]`
- Primary base (offset 0x00): first field named `base`, then `inherit!(Child : Parent)`
- Mixin (non-zero offset): field at exact C++ offset, then `inherit!(Child => Mixin, field_name)`
- **Mixin with virtual methods: MUST have Extension Trait in its own file — no exceptions**

### How to tell mixin from primary base

| Situation | Rule |
|---|---|
| Base occupies offset `0x00` | Primary base → `inherit!(Child : Parent)` |
| Base occupies non-zero offset | Mixin → `inherit!(Child => Mixin, field)` + **Extension Trait required** |

---

## Extension Trait (MANDATORY — triggers automatically)

**TRIGGER:** You are writing a class that:
1. Has `virtual_method!` blocks, AND
2. Is (or will be) used as a mixin field at non-zero offset in ANY child class

→ **Immediately write the Extension Trait in the same file.** Do not wait.
→ If unsure whether it will be used as mixin — write the trait anyway. Cost is zero, missing it breaks child types.

### Full pattern (substitute `MyMixin` / `MyChild` with actual names):

    // Step 1 — virtual methods on the mixin's own impl (always present)
    impl MyMixin {
        virtual_method! {
            pub const DO_THING: usize = 0x04;
            pub fn do_thing(arg: *const ArgType) -> bool
        }
        virtual_method! {
            pub const GET_ITEM: usize = 0x05;
            pub fn get_item() -> *mut ItemType
        }
    }

    // Step 2 — Extension Trait: exposes ALL virtual methods ergonomically
    pub trait MyMixinExt {
        fn do_thing(&self, arg: *const ArgType) -> bool;
        fn get_item(&self) -> *mut ItemType;
    }

    // Step 3 — Blanket impl for any type that AsRef's this mixin
    impl<T: AsRef<MyMixin>> MyMixinExt for T {
        fn do_thing(&self, arg: *const ArgType) -> bool {
            self.as_ref().do_thing(arg)
        }
        fn get_item(&self) -> *mut ItemType {
            self.as_ref().get_item()
        }
    }

    // Step 4 — In child file: inherit! generates AsRef<MyMixin> automatically
    inherit!(MyChild => MyMixin, mixin_field);
    // MyChild now has .do_thing(...) and .get_item() via the trait

### Rules

- Trait name = `<MixinName>Ext` — always, no exceptions
- Defined in the **mixin's** `.rs` file, never in the child's file
- Include **ALL** virtual methods from `impl MyMixin` — missing any breaks usability
- Non-virtual helpers go on `impl MyMixin` only — NOT in the trait
- Child NEVER re-declares mixin methods in its own `impl` block
- `pub use mixin_module::*` in `mod.rs` re-exports both struct and trait automatically

---

## Field Offset Comments (MANDATORY)

Every field in a `#[repr(C)]` struct MUST have its C++ offset as a comment:

    pub struct SomeClass {
        pub base: BaseClass,     // 00
        pub some_field: u32,     // 08
        pub other_field: u64,    // 0C
    }

- Copy offset comments directly from the C++ header
- Never omit them — they are the only way to verify ABI correctness at a glance
- Exception: vtable fields are NOT commented with offset — `virtual_method!` handles indexing

---

## Mandatory Assertions (every struct, no exceptions)

Size assertion — for every struct:

    const _: () = assert!(core::mem::size_of::<StructName>() == 0xSIZE);

Offset assertion — for every mixin field:

    const _: () = assert!(core::mem::offset_of!(StructName, mixin_field) == 0xOFFSET);

---

## Virtual Functions

- NEVER calculate vtable pointers manually
- ALWAYS use `virtual_method!` macro
- Import: `use crate::offsets::offsets_vtable::VTABLE_ClassName;`
- `RelocateVirtual(0xSE, 0xAE, this, args)` in .cpp — record BOTH indices as comment:
  `// vtbl SE: 0xA6, AE: 0xA8` directly above the `virtual_method!` block

---

## Override Comments (MANDATORY)

When a class overrides virtual methods from a parent, document them as comments
inside the `impl` block — even if the implementation goes through `relocation_func!`
or is inherited automatically. Pattern:

    impl MyClass {
        // override (ParentClassName)
        // void InitializeDataComponent() override;                // vtable index 01
        // void ClearDataComponent() override;                     // vtable index 02
        // void CopyComponent(BaseFormComponent* a_rhs) override;  // vtable index 03

        relocation_func! { ... }
    }

Rules:
- Use the exact C++ method signature in the comment
- Include the vtable index from the parent's vtable
- Group all overrides together at the top of the impl block
- If override is fully virtual (no Rust body needed), the comment IS the translation

---

## Const / Non-Const Overload Pairs

C++ frequently declares both `T* GetFoo()` and `const T* GetFoo() const`.
Translate BOTH — never drop the const variant:

| C++ | Rust |
|---|---|
| `T* GetFoo()` | `fn get_foo(&mut self) -> *mut T` |
| `const T* GetFoo() const` | `fn get_foo_ref(&self) -> *const T` |

If both overloads delegate to a private helper (e.g. `GetFooImpl()`):
translate the helper as `fn get_foo_impl(&self) -> *const T` (private, no `pub`).

---

## Static Methods

C++ `static RetType ClassName::Method(Args)` has no `this`.
Translate as associated function — no `&self` or `&mut self`:

    impl ClassName {
        pub fn method(args) -> RetType { ... }
    }

NEVER omit static methods — they are invisible in .h if defined only in .cpp.
Always check .cpp for them explicitly.

---

## File Placement Rules for Extension Traits

- Extension Trait is defined in the **mixin's** file, not the child's
- Child files do NOT need explicit imports — `pub use mixin_module::*` in `mod.rs` covers it
- For explicit method calls in child scope, import the trait directly:
  `use crate::re::MyMixinExt;`
- If a child inherits multiple mixins, import each Extension Trait separately

description: Memory safety, bytemuck, and unsafe discipline.
---

## bytemuck::Zeroable Rules

ALLOWED: structs with ONLY primitives, bools, raw pointers (`*const T`, `*mut T`)

FORBIDDEN: ANY struct with virtual functions, vtable pointer, or inheriting from
`TESForm` / `BaseFormComponent` / `NiRefObject`

## Every `unsafe` Block

Must have a comment explaining the invariant it relies on:

    // SAFETY: ptr is guaranteed non-null by engine contract at this call site
    unsafe { ... }

## Macro Safety Boundary
`relocation_func!`, `virtual_method!`, `relocation_variable!`, and
`runtime_data_accessor!` already encapsulate their unsafe internals.
Do NOT wrap their call sites in an extra `unsafe { }` block unless
you are doing something additionally unsafe at the call site itself.

## Uncertainty Protocol

If ABI, offset, or behavior is uncertain — NEVER guess silently.
You MUST emit: `// TODO: VERIFY — <reason for uncertainty>`
Skipping is forbidden. Partial output with TODO is always better than wrong output.

---

## Engine Memory Allocation (CRITICAL)

**NEVER use Rust's allocator (`Box::new`, `Vec::new`, `alloc::alloc`, etc.)
for RE objects that will be passed to or owned by the engine.**

Skyrim uses its own heap (`RE::malloc` / `MemoryManager`). Mixing allocators
causes heap corruption and crashes — the engine will call `RE::free` on a
pointer that was allocated by Rust's allocator.

### Correct pattern for allocating an RE object:

    pub fn create(arg1: *mut TypeA, arg2: *mut TypeB) -> *mut Self {
        unsafe {
            let ptr = crate::ffi::commonlib_malloc(core::mem::size_of::<Self>()) as *mut Self;
            if !ptr.is_null() {
                (*ptr).ctor();              // engine constructor via relocation_func!
                (*ptr).populate(arg1, arg2); // fill fields
            }
            ptr
        }
    }

### Available FFI allocators (all in `crate::ffi`):

| Function | C++ equivalent | Use for |
|---|---|---|
| `commonlib_malloc(size)` | `RE::malloc(size)` | General RE object allocation |
| `commonlib_free(ptr)` | `RE::free(ptr)` | Free RE object |
| `commonlib_aligned_alloc(align, size)` | `RE::aligned_alloc` | Aligned allocation |
| `commonlib_aligned_free(ptr)` | `RE::aligned_free` | Free aligned allocation |
| `commonlib_calloc(count, size)` | `RE::calloc` | Zero-initialized allocation |
| `commonlib_realloc(ptr, size)` | `RE::realloc` | Reallocate |

### Rules:
- NEVER invent helper functions like `crate::re::malloc::<T>()` — they do not exist
- ALWAYS use `crate::ffi::commonlib_malloc(core::mem::size_of::<Self>()) as *mut Self`
- ALWAYS check the returned pointer for null before dereferencing
- ALWAYS call the engine constructor (`ctor()`) after allocation if the C++ type has one
- Deallocation must use `commonlib_free` / `commonlib_aligned_free` — never Rust's `drop`
  or `dealloc` for engine-owned objects

description: RTTI, VTable and Address Library ID management rules.
---

## Import Pattern (mandatory for every class with RTTI/VTable)

    use crate::offsets::offsets_rtti::RTTI_ClassName;
    use crate::offsets::offsets_vtable::VTABLE_ClassName;

    impl RttiType for ClassName { const RTTI: VariantID = RTTI_ClassName; }

    impl ClassName {
        pub const RTTI: VariantID = RTTI_ClassName;
        pub const VTABLE: &'static [VariantID] = &VTABLE_ClassName;
    }

## RELOCATION_ID Translation Pattern

When .cpp contains:
    static REL::Relocation<func_t> func{ RELOCATION_ID(37787, 38736) };

Translate as `relocation_func!` with BOTH IDs preserved in VariantID:

    // RELOCATION_ID SE: 37787, AE: 38736
    relocation_func! {
        pub fn add_cast_power(&mut self, power: *mut SpellItem) => VariantID::new(37787, 38736, 0)
    }

For static C++ methods (no `this`):

    // RELOCATION_ID SE: 12345, AE: 67890
    relocation_func! {
        pub fn lookup_by_handle(handle: RefHandle) -> NiPointer<Actor> => VariantID::new(12345, 67890, 0)
    }

Rules:
- BOTH IDs (SE and AE) are mandatory — omitting either is forbidden
- VR id = 0 when unknown — write `// TODO: VERIFY — VR ID unknown`
- Comment `// RELOCATION_ID SE: X, AE: Y` is REQUIRED above every relocation_func!
- Never hardcode raw IDs inline — always use VariantID::new(se, ae, vr)

## TESForm Fast Cast (replaces skyrim_cast)

If class inherits from TESForm, implement `FormCastable`:

    impl FormCastable for MyForm {
        const TARGET_FORM_TYPE: FormType = FormType::MyType;
    }

## skyrim_cast (for non-TESForm classes)

For classes inheriting from engine types other than TESForm:

    // Both T and U must impl RttiType
    let ptr: *mut TargetType = unsafe { skyrim_cast::<SourceType, TargetType>(raw_ptr) };

description: Completeness enforcement. Zero tolerance for omissions.
---

## ABSOLUTE PROHIBITIONS
- Comments like `// ... (many more)` → FORBIDDEN
- Skipping virtual functions → FORBIDDEN
- Truncating enum constants → FORBIDDEN
- Assuming .cpp doesn't exist without checking → FORBIDDEN
- Skipping static methods because they have no `this` → FORBIDDEN
- Translating only one overload of a const/non-const pair → FORBIDDEN

## The .cpp Rule (CRITICAL)
For EVERY header `include/RE/.../Name.h`:
1. IMMEDIATELY check `src/RE/.../Name.cpp`
2. Read it fully if it exists
3. Extract ALL of the following — missing any one is a bug:
   - All `RELOCATION_ID(SE, AE)` method implementations
   - Static methods (`RetType ClassName::Method(Args)` with no `this`)
   - Private helper methods present in .cpp but NOT declared in .h
   - `RelocateVirtual(SE_idx, AE_idx, ...)` calls — record both indices
   - Global accessors / singleton patterns
4. Do NOT finalize translation before this step

## Static Methods (CRITICAL)
`RetType ClassName::Method(Args)` in .cpp with no `this` = C++ static method.
Translate as associated function with no `&self`:

    impl ClassName {
        pub fn method(args) -> RetType { ... }
    }

These exist ONLY in .cpp — they will never appear in .h declarations.

## Const / Non-Const Overload Pairs
If .h or .cpp declares BOTH `T* GetFoo()` and `const T* GetFoo() const`:
- Translate BOTH — never drop either variant
- Non-const: `fn get_foo(&mut self) -> *mut T`
- Const:     `fn get_foo_ref(&self) -> *const T`
- Private helper (e.g. `GetFooImpl`): `fn get_foo_impl(&self) -> *const T` (no `pub`)

## TODO Policy
Uncertain about an offset, ABI, or behavior? Write:
`// TODO: VERIFY — [what needs verification and why]`
This is REQUIRED. Silent omission is a bug, not a shortcut.

## Large Classes
Use as many agent turns as needed. Output EVERY method.
If class has 100+ virtual functions — translate all 100+.
Never summarize, never truncate, never defer to "remaining methods follow the same pattern".
