use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiExtraData;
use crate::offsets::offsets_rtti::RTTI_NiExtraData;
use crate::offsets::offsets_vtable::VTABLE_NiExtraData;
use crate::re::NiRTTI;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::ni_object::NiObject;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiExtraData`
#[repr(C)]
pub struct NiExtraData {
    pub base: NiObject,      // 00
    pub name: BSFixedString, // 10
}

const _: () = assert!(core::mem::size_of::<NiExtraData>() == 0x18);

impl RttiType for NiExtraData {
    const RTTI: VariantID = RTTI_NiExtraData;
}

impl crate::re::ni_ref_object::NiRef for NiExtraData {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiExtraData : NiObject);

impl NiExtraData {
    pub const RTTI: VariantID = RTTI_NiExtraData;
    pub const NI_RTTI: VariantID = NiRTTI_NiExtraData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiExtraData;

    // override (NiObject)
    // ~NiExtraData() override;                                       // 00
    // const NiRTTI* GetRTTI() const override;                        // 02
    // void          LoadBinary(NiStream& a_stream) override;         // 18
    // void          LinkObject(NiStream& a_stream) override;         // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;  // 1A
    // void          SaveBinary(NiStream& a_stream) override;         // 1B
    // bool          IsEqual(NiObject* a_object) override;            // 1C

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    virtual_method! {
        pub const VFUNC_LOAD_BINARY: usize = 0x18;
        pub fn load_binary(a_stream: *mut crate::re::NiStream)
    }

    virtual_method! {
        pub const VFUNC_LINK_OBJECT: usize = 0x19;
        pub fn link_object(a_stream: *mut crate::re::NiStream)
    }

    virtual_method! {
        pub const VFUNC_REGISTER_STREAMABLES: usize = 0x1A;
        pub fn register_streamables(a_stream: *mut crate::re::NiStream) -> bool
    }

    virtual_method! {
        pub const VFUNC_SAVE_BINARY: usize = 0x1B;
        pub fn save_binary(a_stream: *mut crate::re::NiStream)
    }

    virtual_method! {
        pub const VFUNC_IS_EQUAL: usize = 0x1C;
        pub fn is_equal(a_object: *mut NiObject) -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_STREAMABLE: usize = 0x25;
        pub fn is_streamable() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_CLONEABLE: usize = 0x26;
        pub fn is_cloneable() -> bool
    }

    #[inline(always)]
    pub fn get_name(&self) -> &BSFixedString {
        &self.name
    }

    #[inline(always)]
    pub fn set_name(&mut self, name: &BSFixedString) {
        self.name = name.clone();
    }

    #[inline(always)]
    pub fn create(size: usize, vtable: usize) -> *mut NiExtraData {
        unsafe {
            let memory = crate::ffi::commonlib_malloc(size).cast::<u8>();
            assert!(!memory.is_null(), "NiExtraData::Create allocation failed");
            core::ptr::write_bytes(memory, 0, size);
            *(memory as *mut usize) = vtable;
            memory.cast()
        }
    }

    #[inline(always)]
    pub fn create_typed<T>(vtable: usize) -> *mut T {
        Self::create(core::mem::size_of::<T>(), vtable).cast()
    }
}
