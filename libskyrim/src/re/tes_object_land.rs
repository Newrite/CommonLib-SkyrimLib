use crate::offsets::offsets_rtti::RTTI_TESObjectLAND;
use crate::offsets::offsets_vtable::VTABLE_TESObjectLAND;
use crate::re::{FormCastable, FormType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESObjectLAND; }

impl RttiType for TESObjectLAND {
    const RTTI: VariantID = RTTI_TESObjectLAND;
}

impl FormCastable for TESObjectLAND {
    const TARGET_FORM_TYPE: FormType = FormType::Land;
}

impl TESObjectLAND {
    pub const RTTI: VariantID = RTTI_TESObjectLAND;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectLAND;
    pub const FORMTYPE: FormType = FormType::Land;
}

// TODO: `TESObjectCELL` only needs pointer-compatible `TESObjectLAND` support for the
// runtime tail today. Replace this opaque stand-in with a real partial/full translation
// once a caller needs `TESObjectLAND` layout, child-cell helpers, or queued-texture
// ownership semantics from `TESObjectLAND.h`.
