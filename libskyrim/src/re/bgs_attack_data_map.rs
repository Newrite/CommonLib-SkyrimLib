use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSAttackDataMap;
use crate::offsets::offsets_vtable::VTABLE_BGSAttackDataMap;
use crate::re::{BGSAttackData, BSFixedString, BSTHashMap, NiPointer, NiRef, NiRefObject, TESRace};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BGSAttackDataMap`
#[repr(C)]
pub struct BGSAttackDataMap {
    pub base: NiRefObject,                                                    // 00
    pub attack_data_map: BSTHashMap<BSFixedString, NiPointer<BGSAttackData>>, // 10
    pub default_data_race: *mut TESRace,                                      // 40
}

const _: () = assert!(core::mem::size_of::<BGSAttackDataMap>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSAttackDataMap, attack_data_map) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSAttackDataMap, default_data_race) == 0x40);

impl RttiType for BGSAttackDataMap {
    const RTTI: VariantID = RTTI_BGSAttackDataMap;
}

inherit!(BGSAttackDataMap : NiRefObject);

impl NiRef for BGSAttackDataMap {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl BGSAttackDataMap {
    pub const RTTI: VariantID = RTTI_BGSAttackDataMap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSAttackDataMap;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }
}
