use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiControllerManager;
use crate::offsets::offsets_rtti::RTTI_NiControllerManager;
use crate::offsets::offsets_vtable::VTABLE_NiControllerManager;
use crate::re::BSAnimNoteListener;
use crate::re::BSFixedString;
use crate::re::BSTHashMap;
use crate::re::NiAVObjectPalette;
use crate::re::NiControllerSequence;
use crate::re::NiPointer;
use crate::re::NiRef;
use crate::re::NiTObjectArray;
use crate::re::NiTObjectSet;
use crate::re::NiTPrimitiveSet;
use crate::re::NiTimeController;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core::ffi::c_char;
use core::ptr;

/// C++ `RE::NiControllerManager`
#[repr(C)]
pub struct NiControllerManager {
    pub base: NiTimeController,                                           // 00
    pub sequence_array: NiTObjectArray<NiPointer<NiControllerSequence>>,  // 48
    pub active_sequences: NiTPrimitiveSet<*mut NiControllerSequence>,     // 60
    pub string_map: BSTHashMap<BSFixedString, *mut NiControllerSequence>, // 70
    pub listener: *mut BSAnimNoteListener,                                // A0
    pub cumulative: bool,                                                 // A8
    pub pada9: u8,                                                        // A9
    pub padaa: u16,                                                       // AA
    pub padac: u32,                                                       // AC
    pub temp_blend_seqs: NiTObjectSet<NiPointer<NiControllerSequence>>,   // B0
    pub object_palette: NiPointer<NiAVObjectPalette>,                     // C0
}

const _: () = assert!(core::mem::size_of::<NiControllerManager>() == 0xC8);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, sequence_array) == 0x48);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, active_sequences) == 0x60);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, string_map) == 0x70);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, listener) == 0xA0);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, cumulative) == 0xA8);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, temp_blend_seqs) == 0xB0);
const _: () = assert!(core::mem::offset_of!(NiControllerManager, object_palette) == 0xC0);

impl RttiType for NiControllerManager {
    const RTTI: VariantID = RTTI_NiControllerManager;
}

impl NiRef for NiControllerManager {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiControllerManager : NiTimeController, base);

impl NiControllerManager {
    pub const RTTI: VariantID = RTTI_NiControllerManager;
    pub const NI_RTTI: VariantID = NiRTTI_NiControllerManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiControllerManager;

    // override (NiTimeController)
    // const NiRTTI*        GetRTTI() const override;                            // 02
    // NiObject*            CreateClone(NiCloningProcess& a_cloning) override;   // 17
    // void                 LoadBinary(NiStream& a_stream) override;             // 18
    // void                 LinkObject(NiStream& a_stream) override;             // 19
    // bool                 RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void                 SaveBinary(NiStream& a_stream) override;             // 1B
    // bool                 IsEqual(NiObject* a_object) override;                // 1C
    // void                 ProcessClone(NiCloningProcess& a_cloning) override;  // 1D
    // void                 PostLinkObject(NiStream& a_stream) override;         // 1E
    // NiControllerManager* AsNiControllerManager() override;                    // 24
    // void                 Start(float a_time) override;                        // 25
    // void                 Stop() override;                                     // 26
    // void                 Update(float a_time) override;                       // 27
    // void                 SetTarget(NiObjectNET* a_target) override;           // 28
    // bool                 TargetIsRequiredType() const override;               // 2E

    virtual_method! {
        pub const VFUNC_START_NO_ARGS: usize = 0x2F;
        pub fn start_no_args()
    }

    #[inline]
    pub fn get_sequence_by_name(&self, a_name: &str) -> *mut NiControllerSequence {
        let key = BSFixedString::from_str(a_name);
        let it = self.string_map.find(&key);
        if it.is_null() {
            return ptr::null_mut();
        }
        unsafe { (*it).second }
    }

    #[inline]
    pub fn get_sequence_by_name_cstr(&self, a_name: *const c_char) -> *mut NiControllerSequence {
        if a_name.is_null() {
            return ptr::null_mut();
        }
        let name = unsafe { core::ffi::CStr::from_ptr(a_name) };
        let name = name.to_string_lossy();
        let it = self.string_map.find(&BSFixedString::from_str(&name));
        if it.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*it).second }
        }
    }
}
