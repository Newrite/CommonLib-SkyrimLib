use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSAttackData;
use crate::offsets::offsets_vtable::VTABLE_BGSAttackData;
use crate::re::{BGSKeyword, BSFixedString, NiRef, NiRefObject, SpellItem};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::AttackData::AttackFlag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSAttackDataAttackFlag {
    None = 0,
    IgnoreWeapon = 1 << 0,
    BashAttack = 1 << 1,
    PowerAttack = 1 << 2,
    ChargeAttack = 1 << 3,
    RotatingAttack = 1 << 4,
    ContinuousAttack = 1 << 5,
    OverrideData = 0x8000_0000,
}

core_util::impl_enumset_type!(BGSAttackDataAttackFlag => u32);

/// C++ `RE::AttackData`
#[repr(C)]
pub struct AttackData {
    pub damage_mult: f32,                             // 00
    pub attack_chance: f32,                           // 04
    pub attack_spell: *mut SpellItem,                 // 08
    pub flags: EnumSet<BGSAttackDataAttackFlag, u32>, // 10
    pub attack_angle: f32,                            // 14
    pub strike_angle: f32,                            // 18
    pub stagger_offset: f32,                          // 1C
    pub attack_type: *mut BGSKeyword,                 // 20
    pub knock_down: f32,                              // 28
    pub recovery_time: f32,                           // 2C
    pub stamina_mult: f32,                            // 30
    pub pad34: u32,                                   // 34
}

const _: () = assert!(core::mem::size_of::<AttackData>() == 0x38);
const _: () = assert!(core::mem::offset_of!(AttackData, attack_spell) == 0x08);
const _: () = assert!(core::mem::offset_of!(AttackData, flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(AttackData, attack_type) == 0x20);

/// C++ `RE::BGSAttackData`
#[repr(C)]
pub struct BGSAttackData {
    pub base: NiRefObject,    // 00
    pub event: BSFixedString, // 10
    pub data: AttackData,     // 18
}

const _: () = assert!(core::mem::size_of::<BGSAttackData>() == 0x50);
const _: () = assert!(core::mem::offset_of!(BGSAttackData, event) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSAttackData, data) == 0x18);

impl RttiType for BGSAttackData {
    const RTTI: VariantID = RTTI_BGSAttackData;
}

inherit!(BGSAttackData : NiRefObject);

impl NiRef for BGSAttackData {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl BGSAttackData {
    pub const RTTI: VariantID = RTTI_BGSAttackData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSAttackData;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::relocation_func! {
        fn ctor_impl(&mut self) -> *mut BGSAttackData => RelocationID::new(26718, 27397)
    }

    #[inline(always)]
    pub fn create() -> *mut Self {
        unsafe { crate::ffi::commonlib_bgs_attack_data_create().cast() }
    }

    #[inline(always)]
    pub unsafe fn ctor_in_place(&mut self) -> *mut Self {
        self.ctor_impl()
    }

    #[inline(always)]
    pub fn is_left_attack(&self) -> bool {
        self.data.flags.all(BGSAttackDataAttackFlag::ChargeAttack)
    }
}
