use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiShadeProperty;
use crate::offsets::offsets_rtti::RTTI_NiShadeProperty;
use crate::offsets::offsets_vtable::VTABLE_NiShadeProperty;
use crate::re::{BSGeometry, NiProperty, NiPropertyType, NiRTTI};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiShadeProperty`
#[repr(C)]
pub struct NiShadeProperty {
    pub base: NiProperty, // 00
}

const _: () = assert!(core::mem::size_of::<NiShadeProperty>() == 0x30);
const _: () = assert!(core::mem::offset_of!(NiShadeProperty, base) == 0x00);

impl RttiType for NiShadeProperty {
    const RTTI: VariantID = RTTI_NiShadeProperty;
}

impl crate::re::ni_ref_object::NiRef for NiShadeProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiShadeProperty : NiProperty, base);

impl NiShadeProperty {
    pub const RTTI: VariantID = RTTI_NiShadeProperty;
    pub const NI_RTTI: VariantID = NiRTTI_NiShadeProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiShadeProperty;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x25;
        pub fn get_type() -> NiPropertyType
    }

    crate::virtual_method! {
        pub const VFUNC_SETUP_GEOMETRY: usize = 0x27;
        pub fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_FINISH_SETUP_GEOMETRY: usize = 0x28;
        pub fn finish_setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_LIGHT_STATE: usize = 0x29;
        pub fn set_light_state(&mut self, light_index: i32)
    }
}

impl AsRef<NiShadeProperty> for NiShadeProperty {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiShadeProperty> for NiShadeProperty {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait NiShadePropertyExt {
    fn get_rtti(&self) -> *const NiRTTI;
    fn get_type(&self) -> NiPropertyType;
    fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool;
    fn finish_setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool;
    fn set_light_state(&mut self, light_index: i32);
}

impl<T: AsRef<NiShadeProperty> + AsMut<NiShadeProperty>> NiShadePropertyExt for T {
    #[inline(always)]
    fn get_rtti(&self) -> *const NiRTTI {
        NiShadeProperty::get_rtti(self.as_ref())
    }

    #[inline(always)]
    fn get_type(&self) -> NiPropertyType {
        NiShadeProperty::get_type(self.as_ref())
    }

    #[inline(always)]
    fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool {
        NiShadeProperty::setup_geometry(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn finish_setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool {
        NiShadeProperty::finish_setup_geometry(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn set_light_state(&mut self, light_index: i32) {
        NiShadeProperty::set_light_state(self.as_mut(), light_index)
    }
}
