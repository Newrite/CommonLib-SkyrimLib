---
trigger: always_on
description: C++ to Rust inheritance translation rules. Always apply when working with structs.
---

## Layout Rules
- ALL structs: `#[repr(C)]`
- Primary base: first field named `base`, then `inherit!(Child : Parent)`
- Mixin: field at exact C++ offset, then `inherit!(Child => Mixin, field_name)`
- Mixin methods: ONLY via Extension Traits (e.g. `FullNameExt`), never directly on child

## Field Offset Comments (MANDATORY)
Every field in a `#[repr(C)]` struct MUST have its C++ offset as a comment:

    pub struct TESDescription {
        pub base: BaseFormComponent, // 00
        pub file_offset: u32,        // 08
        pub description_text: BGSLocalizedStringDL, // 0C
    }

- Copy offset comments directly from the C++ header
- Never omit them — they are the only way to verify ABI correctness at a glance
- Exception: vtable fields are NOT commented with offset — `virtual_method!` handles indexing

## Mandatory Assertions (every struct, no exceptions)

Size assertion — for every struct:

    const _: () = assert!(core::mem::size_of::<StructName>() == 0xSIZE);

Offset assertion — for every mixin field:

    const _: () = assert!(core::mem::offset_of!(StructName, mixin_field) == 0xOFFSET);

## Virtual Functions
- NEVER calculate vtable pointers manually
- ALWAYS use `virtual_method!` macro
- Import: `use crate::offsets::offsets_vtable::VTABLE_ClassName;`
- `RelocateVirtual(0xSE, 0xAE, this, args)` in .cpp — record BOTH indices as comment:
  `// vtbl SE: 0xA6, AE: 0xA8` directly above the `virtual_method!` block

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

## Const / Non-Const Overload Pairs
C++ frequently declares both `T* GetFoo()` and `const T* GetFoo() const`.
Translate BOTH — never drop the const variant:

| C++ | Rust |
|---|---|
| `T* GetFoo()` | `fn get_foo(&mut self) -> *mut T` |
| `const T* GetFoo() const` | `fn get_foo_ref(&self) -> *const T` |

If both overloads delegate to a private helper (e.g. `GetFooImpl()`):
translate the helper as `fn get_foo_impl(&self) -> *const T` (private, no `pub`).

## Static Methods
C++ `static RetType ClassName::Method(Args)` has no `this`.
Translate as associated function — no `&self` or `&mut self`:

    impl ClassName {
        pub fn method(args) -> RetType { ... }
    }

NEVER omit static methods — they are invisible in .h if defined only in .cpp.
Always check .cpp for them explicitly.


## Extension Trait Pattern (Mixin Method Access)

When a mixin class (e.g. `BGSKeywordForm`) has virtual methods that child classes
inherit, those methods are NOT put directly on the child. Instead, create an
Extension Trait so any `T: AsRef<MixinType>` gets the methods automatically.

### Structure (mandatory pattern, follow exactly):

**Step 1** — virtual methods go on the mixin's own `impl` block:

    impl BGSKeywordForm {
        virtual_method! {
            pub const HAS_KEYWORD: usize = 0x04;
            pub fn has_keyword(this: &BGSKeywordForm, keyword: *const BGSKeyword) -> bool
        }
    }

**Step 2** — Extension Trait defined in the same file as the mixin:

    pub trait BGSKeywordFormExt {
        fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
        fn get_default_keyword(&self) -> *mut BGSKeyword;
    }

**Step 3** — Blanket impl for any type that `AsRef`s the mixin:

    impl<T: AsRef<BGSKeywordForm>> BGSKeywordFormExt for T {
        fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
            self.as_ref().has_keyword(self.as_ref(), keyword)
        }
        fn get_default_keyword(&self) -> *mut BGSKeyword {
            self.as_ref().get_default_keyword(self.as_ref())
        }
    }

**Step 4** — Child struct gets `inherit!` for the mixin field,
which auto-generates `AsRef<BGSKeywordForm>`:

    inherit!(TESObjectWEAP => BGSKeywordForm, keyword_form);

This gives `TESObjectWEAP` access to `.has_keyword(...)` via the trait —
no explicit cast needed.

### Rules:
- Extension Trait name = `<MixinName>Ext` (e.g. `BGSKeywordFormExt`)
- Trait is defined in the MIXIN's .rs file, not the child's
- Include ALL virtual methods of the mixin in the trait — no exceptions
- Non-virtual helper methods (e.g. `get_num_keywords`) go directly on
  `impl MixinType` only — they do NOT need to be in the Extension Trait
- Child NEVER re-declares mixin methods in its own impl block

### File Placement Rules for Extension Traits

- Extension Trait is defined in the MIXIN's file, not the child's:
  `libskyrim/src/re/bgs_keyword_form.rs` contains both `BGSKeywordForm` and `BGSKeywordFormExt`

- Child files do NOT need explicit imports for Extension Traits:
  `pub use bgs_keyword_form::*` in `mod.rs` already re-exports both the struct and the trait

- For explicit method calls in child scope, import the trait directly:
  `use crate::re::BGSKeywordFormExt;`

- If a child inherits multiple mixins, import each Extension Trait separately:

      use crate::re::BGSKeywordFormExt;
      use crate::re::TESFullNameExt;
      use crate::re::TESModelExt;