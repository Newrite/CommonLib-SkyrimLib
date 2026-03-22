---
trigger: always_on
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
