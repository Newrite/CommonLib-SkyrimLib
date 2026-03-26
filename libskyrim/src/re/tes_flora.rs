use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESFlora;
use crate::offsets::offsets_vtable::VTABLE_TESFlora;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_object_acti::TESObjectACTI;
use crate::re::tes_produce_form::TESProduceForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESFloraRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESFlora {
    pub base: TESObjectACTI,          // 00
    pub produce_form: TESProduceForm, // C8
}

const _: () = assert!(core::mem::size_of::<TESFlora>() == 0xE8);
const _: () = assert!(core::mem::offset_of!(TESFlora, produce_form) == 0xC8);

impl RttiType for TESFlora {
    const RTTI: VariantID = RTTI_TESFlora;
}

impl FormCastable for TESFlora {
    const TARGET_FORM_TYPE: FormType = FormType::Flora;
}

inherit!(TESFlora : TESObjectACTI);
inherit!(TESFlora => TESProduceForm, produce_form);

impl TESFlora {
    pub const RTTI: VariantID = RTTI_TESFlora;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESFlora;
    pub const FORMTYPE: FormType = FormType::Flora;

    // override (TESObjectACTI)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // override (TESObjectACTI)
    // bool Load(TESFile* a_mod) override;                                                                    // 06
    // void InitItemImpl() override;                                                                          // 13
    // bool Activate(...) override;                                                                           // 37
    // bool GetActivateText(TESObjectREFR* a_activator, BSString& a_dst) override;                           // 4C
    // bool CalculateDoFavor(Actor* a_activator, bool a_arg2, TESObjectREFR* a_toActivate, float) override; // 4D
}
