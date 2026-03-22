---
description: Load this skill when working with C++ class hierarchies, multiple
  inheritance, vtable layouts, mixin offsets, or tracing parent class chains.
---

## Procedural Steps

1. **Identify Parents**: Parse `: public Parent1, public Parent2`
2. **Locate Headers**: `CommonLibVR/include/RE/<Letter>/Parent.h` for each
3. **Check .cpp**: For each parent, also check `src/RE/<Letter>/Parent.cpp`
   — parents may have RELOCATION_IDs and static methods too
4. **Recurse**: Repeat for every parent until root (`NiRefObject` / `TESForm` / `BaseFormComponent`)
5. **Build Memory Map** (internal):
   - Primary base (Parent1) starts at 0x00, size = sizeof(Parent1)
   - Mixin (Parent2) starts at sizeof(Parent1) + padding to alignment
   - Record every vtable index from every parent in chain
6. **Output**: Print inheritance chain summary BEFORE any code block

## Required Summary Format
Print this before writing any Rust code:

    Inheritance chain: ChildClass
      → Parent1 (primary, offset 0x00, size 0xSIZE)
        → GrandParent (primary, offset 0x00)
      → Parent2/Mixin (mixin, offset 0xOFFSET, size 0xSIZE)
    Overrides from Parent1: virtual[01] InitFoo, virtual[03] DoBar
    Overrides from Parent2: virtual[00] GetName

## Override Tracking
For each parent in the chain, list which virtual functions the child overrides.
These become `// override (ParentName)` comments in the Rust impl block.
Source: compare C++ header virtual declarations with child's .h and .cpp.

## Mixin Rules
- Mixin field offset MUST match C++ exactly → add `offset_of!` assertion
- Virtual calls on mixin need pointer shift → use `inherit!(Child => Mixin, field)`
- Mixin methods exposed via Extension Trait ONLY — never directly on child impl

## Rust Output Structure
After building the memory map, the Rust struct fields must follow this order:
1. `base: PrimaryParent`        — always first, offset 0x00
2. Local fields in offset order — with `// 0xOFFSET` comments
3. Mixin fields                 — at their exact C++ offsets
4. Padding fields if needed     — `_pad: [u8; N]` with `// padding` comment
