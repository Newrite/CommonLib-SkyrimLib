use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiAlphaProperty;
use crate::offsets::offsets_rtti::RTTI_NiAlphaProperty;
use crate::offsets::offsets_vtable::VTABLE_NiAlphaProperty;
use crate::re::{NiProperty, NiPropertyType, NiRTTI};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiAlphaProperty::AlphaFunction`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiAlphaPropertyAlphaFunction {
    One = 0,
    Zero = 1,
    SrcColor = 2,
    InvSrcColor = 3,
    DestColor = 4,
    InvDestColor = 5,
    SrcAlpha = 6,
    InvSrcAlpha = 7,
    DestAlpha = 8,
    InvDestAlpha = 9,
    SrcAlphaSat = 10,
}

/// C++ `RE::NiAlphaProperty::TestFunction`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiAlphaPropertyTestFunction {
    Always = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Never = 7,
}

/// C++ `RE::NiAlphaProperty::AlphaFlags`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiAlphaPropertyAlphaFlags {
    IsEditorModifiable = 1 << 15,
}

/// C++ `RE::NiAlphaProperty`
#[repr(C)]
pub struct NiAlphaProperty {
    pub base: NiProperty,    // 00
    pub alpha_flags: u16,    // 30
    pub alpha_threshold: u8, // 32
    pub pad33: u8,           // 33
    pub pad34: u32,          // 34
}

const _: () = assert!(core::mem::size_of::<NiAlphaProperty>() == 0x38);
const _: () = assert!(core::mem::offset_of!(NiAlphaProperty, alpha_flags) == 0x30);

impl RttiType for NiAlphaProperty {
    const RTTI: VariantID = RTTI_NiAlphaProperty;
}

impl crate::re::ni_ref_object::NiRef for NiAlphaProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiAlphaProperty : NiProperty, base);

impl NiAlphaProperty {
    pub const RTTI: VariantID = RTTI_NiAlphaProperty;
    pub const NI_RTTI: VariantID = NiRTTI_NiAlphaProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiAlphaProperty;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x25;
        pub fn get_type() -> NiPropertyType
    }

    #[inline(always)]
    pub fn get_alpha_blending(&self) -> bool {
        (self.alpha_flags & 1) != 0
    }

    #[inline(always)]
    pub fn get_alpha_testing(&self) -> bool {
        ((self.alpha_flags >> 9) & 1) != 0
    }

    #[inline(always)]
    pub fn get_dest_blend_mode(&self) -> NiAlphaPropertyAlphaFunction {
        unsafe { core::mem::transmute(((self.alpha_flags >> 5) & 15) as i32) }
    }

    #[inline(always)]
    pub fn get_src_blend_mode(&self) -> NiAlphaPropertyAlphaFunction {
        unsafe { core::mem::transmute(((self.alpha_flags >> 1) & 15) as i32) }
    }

    #[inline(always)]
    pub fn set_alpha_blending(&mut self, enable: bool) {
        if enable {
            self.alpha_flags |= 1;
        } else {
            self.alpha_flags &= !1;
        }
    }

    #[inline(always)]
    pub fn set_alpha_testing(&mut self, enable: bool) {
        if enable {
            self.alpha_flags |= 512;
        } else {
            self.alpha_flags &= !512;
        }
    }

    #[inline(always)]
    pub fn set_dest_blend_mode(&mut self, mode: NiAlphaPropertyAlphaFunction) {
        self.alpha_flags &= !480;
        self.alpha_flags |= (mode as u16) << 5;
    }

    #[inline(always)]
    pub fn set_src_blend_mode(&mut self, mode: NiAlphaPropertyAlphaFunction) {
        self.alpha_flags &= !30;
        self.alpha_flags |= (mode as u16) << 1;
    }
}
