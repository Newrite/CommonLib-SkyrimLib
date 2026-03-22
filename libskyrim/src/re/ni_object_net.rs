use crate::core_util::inherit;
use crate::relocation_func;

use crate::offsets::offsets_rtti::RTTI_NiObjectNET;
use crate::offsets::offsets_vtable::VTABLE_NiObjectNET;
use crate::relocation::{VariantID, RttiType};

use crate::re::ni_object::NiObject;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::ni_time_controller::NiTimeController;

#[repr(C)]
pub struct NiObjectNET {
    pub base: NiObject,                      // 00
    pub name: BSFixedString,                  // 10
    pub controllers: NiPointer<NiTimeController>, // 18
    pub extra: *mut *mut core::ffi::c_void,   // 20 - NiExtraData
    pub extra_data_size: u16,                // 28
    pub max_size: u16,                       // 2A
    pub pad2c: u32,                          // 2C
}

const _: () = assert!(core::mem::size_of::<NiObjectNET>() == 0x30);

impl RttiType for NiObjectNET {
    const RTTI: VariantID = RTTI_NiObjectNET;
}

impl crate::re::ni_ref_object::NiRef for NiObjectNET {
    #[inline(always)]
    fn inc_ref(&self) { self.base.inc_ref(); }
    #[inline(always)]
    fn dec_ref(&self) { self.base.dec_ref(); }
}

inherit!(NiObjectNET : NiObject);

impl NiObjectNET {
    pub const RTTI: VariantID = RTTI_NiObjectNET;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiObjectNET;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                            // 02
    // void          LoadBinary(NiStream& a_stream) override;             // 18
    // void          LinkObject(NiStream& a_stream) override;             // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void          SaveBinary(NiStream& a_stream) override;             // 1B
    // bool          IsEqual(NiObject* a_object) override;                // 1C
    // void          ProcessClone(NiCloningProcess& a_cloning) override;  // 1D
    // void          PostLinkObject(NiStream& a_stream) override;         // 1E

    // RELOCATION_ID SE: 68856, AE: 70208
    relocation_func! {
        pub fn remove_controller(&mut self, a_controller: *mut NiTimeController) -> bool => VariantID::new(68856, 70208, 0)
    }

    // RELOCATION_ID SE: 68855, AE: 70207
    relocation_func! {
        pub fn get_controllers(&self) -> *mut NiTimeController => VariantID::new(68855, 70207, 0)
    }
}

impl AsRef<NiObjectNET> for NiObjectNET {
    #[inline(always)]
    fn as_ref(&self) -> &Self { self }
}

impl AsMut<NiObjectNET> for NiObjectNET {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self { self }
}

pub trait NiObjectNETExt {
    fn remove_controller(&mut self, a_controller: *mut NiTimeController) -> bool;
    fn get_controllers(&self) -> *mut NiTimeController;
}

impl<T: AsRef<NiObjectNET> + AsMut<NiObjectNET>> NiObjectNETExt for T {
    fn remove_controller(&mut self, a_controller: *mut NiTimeController) -> bool {
        NiObjectNET::remove_controller(self.as_mut(), a_controller)
    }

    fn get_controllers(&self) -> *mut NiTimeController {
        NiObjectNET::get_controllers(self.as_ref())
    }
}
