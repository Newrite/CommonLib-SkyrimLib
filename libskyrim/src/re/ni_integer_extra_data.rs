use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiIntegerExtraData;
use crate::offsets::offsets_rtti::RTTI_NiIntegerExtraData;
use crate::offsets::offsets_vtable::VTABLE_NiIntegerExtraData;
use crate::re::{BSFixedString, NiExtraData, NiRTTI};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NiIntegerExtraData`
#[repr(C)]
pub struct NiIntegerExtraData {
    pub base: NiExtraData, // 00
    pub value: i32,        // 18
    pub pad1c: u32,        // 1C
}

const _: () = assert!(core::mem::size_of::<NiIntegerExtraData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(NiIntegerExtraData, value) == 0x18);

impl RttiType for NiIntegerExtraData {
    const RTTI: VariantID = RTTI_NiIntegerExtraData;
}

impl crate::re::ni_ref_object::NiRef for NiIntegerExtraData {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiIntegerExtraData : NiExtraData, base);

impl NiIntegerExtraData {
    pub const RTTI: VariantID = RTTI_NiIntegerExtraData;
    pub const NI_RTTI: VariantID = NiRTTI_NiIntegerExtraData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiIntegerExtraData;

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    #[inline(always)]
    pub fn create(name: &BSFixedString, value: i32) -> *mut NiIntegerExtraData {
        let data = NiExtraData::create_typed::<NiIntegerExtraData>(Self::VTABLE[0].address());
        if !data.is_null() {
            unsafe {
                (*data).base.name = name.clone();
                (*data).value = value;
            }
        }
        data
    }
}
