use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_NiObjectNET;
use crate::offsets::offsets_vtable::VTABLE_NiObjectNET;
use crate::re::BSFixedString;
use crate::re::NiExtraData;
use crate::re::NiObject;
use crate::re::NiPointer;
use crate::re::NiTimeController;
// other imports
use crate::relocation_func;
use crate::virtual_method;
use crate::relocation::VariantID;

#[repr(C)]
pub struct NiObjectNET {
    pub base: NiObject,                            // 00
    pub name: BSFixedString,                       // 10
    pub controllers: NiPointer<NiTimeController>,  // 18
    pub extra: *mut *mut NiExtraData,              // 20
    pub extra_data_size: u16,                      // 28
    pub max_size: u16,                             // 2A
    pub pad2c: u32,                                // 2C
}

const _: () = assert!(core::mem::size_of::<NiObjectNET>() == 0x30);

impl crate::relocation::RttiType for NiObjectNET {
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

    // RELOCATION_ID SE: 69156, AE: 70517
    relocation_func! {
        pub fn remove_controller(this: &mut NiObjectNET, a_controller: *mut NiTimeController) => VariantID::new(69156, 70517, 0)
    }

    pub fn get_controllers(&self) -> *mut NiTimeController {
        self.controllers.get()
    }
}
