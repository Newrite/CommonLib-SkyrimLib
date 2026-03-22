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
