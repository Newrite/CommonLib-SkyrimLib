use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSXFlags;
use crate::offsets::offsets_rtti::RTTI_BSXFlags;
use crate::offsets::offsets_vtable::VTABLE_BSXFlags;
use crate::re::{NiIntegerExtraData, NiRTTI};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSXFlags::Flag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSXFlagsFlag {
    None = 0,
    Animated = 1 << 0,
    Havok = 1 << 1,
    Ragdoll = 1 << 2,
    Complex = 1 << 3,
    Addon = 1 << 4,
    EditorMarker = 1 << 5,
    Dynamic = 1 << 6,
    Articulated = 1 << 7,
    NeedsTransformUpdate = 1 << 8,
    ExternalEmit = 1 << 9,
    MagicShaderParticles = 1 << 10,
    Lights = 1 << 11,
    Breakable = 1 << 12,
    SearchedBreakable = 1 << 13,
}

core_util::impl_enumset_type!(BSXFlagsFlag => i32);

pub type BSXFlagsFlags = EnumSet<BSXFlagsFlag, i32>;

/// C++ `RE::BSXFlags`
#[repr(C)]
pub struct BSXFlags {
    pub base: NiIntegerExtraData, // 00
}

const _: () = assert!(core::mem::size_of::<BSXFlags>() == 0x20);

impl RttiType for BSXFlags {
    const RTTI: VariantID = RTTI_BSXFlags;
}

impl crate::re::ni_ref_object::NiRef for BSXFlags {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSXFlags : NiIntegerExtraData, base);

impl BSXFlags {
    pub const RTTI: VariantID = RTTI_BSXFlags;
    pub const NI_RTTI: VariantID = NiRTTI_BSXFlags;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSXFlags;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    #[inline(always)]
    pub fn get_flags(&self) -> BSXFlagsFlags {
        BSXFlagsFlags::from_underlying(self.base.value)
    }

    #[inline(always)]
    pub fn set_flags(&mut self, flags: BSXFlagsFlags) {
        self.base.value = flags.underlying();
    }
}
