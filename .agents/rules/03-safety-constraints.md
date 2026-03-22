---
trigger: always_on
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
