---
trigger: always_on
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
            pub fn do_thing(this: &MyMixin, arg: *const ArgType) -> bool
        }
        virtual_method! {
            pub const GET_ITEM: usize = 0x05;
            pub fn get_item(this: &MyMixin) -> *mut ItemType
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
            self.as_ref().do_thing(self.as_ref(), arg)
        }
        fn get_item(&self) -> *mut ItemType {
            self.as_ref().get_item(self.as_ref())
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
