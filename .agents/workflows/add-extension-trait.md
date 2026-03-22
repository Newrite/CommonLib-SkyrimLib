---
name: add-extension-trait
description: Add Extension Trait to an existing RE mixin file if conditions are met.
---

First load the `form-extension-trait` skill — it defines the full pattern and all rules.

Add Extension Trait to the file: {{args}}

---

## Step 1 — Eligibility Check (stop if any condition is false)

Read the target file. Verify ALL of the following:

- [ ] The file contains at least one `virtual_method!` block
- [ ] The struct is used as a base or mixin somewhere in `libskyrim/src/re/`
  — search for EITHER of these patterns:
    - `=> StructName,`   ← mixin at non-zero offset
    - `: StructName)`    ← primary base at offset 0x00
  At least one match must exist.
- [ ] The file does NOT already contain `pub trait <StructName>Ext`

If any condition is false — **stop and report why**. Do not modify the file.

---

## Step 2 — Collect Methods

Read the entire `impl StructName { ... }` block and categorize every method.

### Category A — Virtual methods (from `virtual_method!`)
For each block extract:
- method name
- arguments as they appear in the macro (no `this:` — macro handles it internally)
- return type

### Category B — Safe wrapper helpers
Public methods in `impl StructName` that wrap a raw virtual for ergonomic use:
- `*const c_char` → `&str`
- nullable `*mut T` → `Option<&T>` / `Option<&mut T>`

### Category C — Useful non-virtual public helpers
Public methods that are not virtual and not wrappers, but are useful to child types:
- counters, accessors, convenience predicates

Do NOT collect: private methods, `ctor`, internal relocation helpers.

---

## Step 3 — Generate Missing Safe Wrappers (if needed)

If a virtual returns a raw/unsafe type and no ergonomic wrapper exists, add it
to `impl StructName` before writing the trait:

    #[inline]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_full_name())
    }

    #[inline]
    pub fn get_item(&self) -> Option<&ItemType> {
        unsafe {
            // SAFETY: engine returns null or valid pointer tied to `self`
            self.get_item_raw().as_ref()
        }
    }

Rules:
- Always `#[inline]`
- Add `// SAFETY:` comment on unsafe conversions
- **Do NOT remove the raw virtual — wrappers never replace raw methods**

---

## Step 4 — Write the Extension Trait

Append at the END of the file after all existing `impl` blocks.

### CRITICAL inclusion rule

**Every `virtual_method!` method MUST appear in the trait — no exceptions.**
Even if a safe wrapper exists, include the raw virtual too.
A safe wrapper is an addition, never a substitute.

### Block 1 — Trait definition

Order:
1. All Category A raw virtual methods
2. All Category B safe wrappers
3. All Category C useful helpers

    pub trait StructNameExt {
        // Category A — raw virtuals
        fn get_raw_name(&self) -> *const core::ffi::c_char;
        fn do_thing(&self, arg: *const ArgType) -> bool;
        // Category B — safe wrappers
        fn get_name_as_str(&self) -> &str;
        // Category C — non-virtual helpers
        fn get_count(&self) -> u32;
        // mutable methods
        fn set_name(&mut self, name: *const core::ffi::c_char);
    }

### Block 2 — Blanket impl

Use `AsRef` for `&self` methods, `AsMut` for `&mut self` methods.
If both are present, use both bounds:

    impl<T: AsRef<StructName> + AsMut<StructName>> StructNameExt for T {
        fn get_raw_name(&self) -> *const core::ffi::c_char {
            self.as_ref().get_raw_name()
        }
        fn do_thing(&self, arg: *const ArgType) -> bool {
            self.as_ref().do_thing(arg)
        }
        fn get_name_as_str(&self) -> &str {
            self.as_ref().get_name_as_str()
        }
        fn get_count(&self) -> u32 {
            self.as_ref().get_count()
        }
        fn set_name(&mut self, name: *const core::ffi::c_char) {
            self.as_mut().set_name(name)
        }
    }

**Unified call rule:**
- `&self` methods: `self.as_ref().method(args)`
- `&mut self` methods: `self.as_mut().method(args)`
- Never pass `self.as_ref()` as an explicit `this` argument — macro handles it

---

## Step 5 — Verify mod.rs re-export

Open `libskyrim/src/re/mod.rs`. Find the module entry:

    pub mod struct_name;
    pub use struct_name::*;

If `pub use struct_name::*;` is already present — no change needed.
If only `pub mod struct_name;` exists — add `pub use struct_name::*;` immediately after.

---

## Step 6 — Final Checklist

- [ ] Every `virtual_method!` method is in the trait
- [ ] No raw virtual was omitted because a wrapper exists — both are present
- [ ] Useful non-virtual helpers included where appropriate
- [ ] `&mut self` methods use `AsMut`, `&self` methods use `AsRef`
- [ ] No explicit `this:` argument in `virtual_method!` declarations
- [ ] Blanket impl uses unified `self.as_ref()` / `self.as_mut()` pattern
- [ ] Child types do not re-declare mixin methods

---

## Step 7 — Report

- File modified: `<path>`
- Trait added: `<StructName>Ext`
- Virtual methods (Category A): list
- Safe wrappers (Category B): list or "none added"
- Non-virtual helpers (Category C): list or "none"
- mod.rs changed: yes / no
- `cargo check --lib` result: run and report
