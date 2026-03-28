use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiProperty;
use crate::offsets::offsets_rtti::RTTI_NiProperty;
use crate::offsets::offsets_vtable::VTABLE_NiProperty;
use crate::re::{NiObjectNET, NiRTTI};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiProperty::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiPropertyType {
    Alpha = 0,
    Shade = 1,
}

/// C++ `RE::NiProperty`
#[repr(C)]
pub struct NiProperty {
    pub base: NiObjectNET, // 00
}

const _: () = assert!(core::mem::size_of::<NiProperty>() == 0x30);
const _: () = assert!(core::mem::offset_of!(NiProperty, base) == 0x00);

impl RttiType for NiProperty {
    const RTTI: VariantID = RTTI_NiProperty;
}

impl crate::re::ni_ref_object::NiRef for NiProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiProperty : NiObjectNET, base);

impl NiProperty {
    pub const RTTI: VariantID = RTTI_NiProperty;
    pub const NI_RTTI: VariantID = NiRTTI_NiProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiProperty;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x25;
        pub fn get_type() -> NiPropertyType
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x26;
        pub fn update(&mut self, time: f32)
    }
}

impl AsRef<NiProperty> for NiProperty {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiProperty> for NiProperty {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait NiPropertyExt {
    fn get_rtti(&self) -> *const NiRTTI;
    fn get_type(&self) -> NiPropertyType;
    fn update(&mut self, time: f32);
}

impl<T: AsRef<NiProperty> + AsMut<NiProperty>> NiPropertyExt for T {
    #[inline(always)]
    fn get_rtti(&self) -> *const NiRTTI {
        NiProperty::get_rtti(self.as_ref())
    }

    #[inline(always)]
    fn get_type(&self) -> NiPropertyType {
        NiProperty::get_type(self.as_ref())
    }

    #[inline(always)]
    fn update(&mut self, time: f32) {
        NiProperty::update(self.as_mut(), time)
    }
}
