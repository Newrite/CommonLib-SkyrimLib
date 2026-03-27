use crate::offsets::offsets_rtti::RTTI_BSScript__ObjectBindPolicy;
use crate::offsets::offsets_vtable::VTABLE_BSScript__ObjectBindPolicy;
use crate::re::{
    BSFixedString, BSSpinLock, BSTHashMap, BSTScrapHashMap, BSTSmartPointer,
    IVMObjectBindInterface, IVirtualMachine, Object, VMHandle, Variable, bst_hash_map::UnkValue,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

#[repr(C)]
pub struct ObjectBindPolicy {
    pub vtable: *const usize,                             // 00
    pub vm: *mut IVirtualMachine,                         // 08
    pub bind_interface: *mut IVMObjectBindInterface,      // 10
    pub attached_scripts_lock: BSSpinLock,                // 18
    pub attached_scripts: BSTHashMap<VMHandle, UnkValue>, // 20
}

const _: () = assert!(core::mem::size_of::<ObjectBindPolicy>() == 0x50);

impl RttiType for ObjectBindPolicy {
    const RTTI: VariantID = RTTI_BSScript__ObjectBindPolicy;
}

impl ObjectBindPolicy {
    pub const RTTI: VariantID = RTTI_BSScript__ObjectBindPolicy;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__ObjectBindPolicy;

    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04() }
    crate::virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05() }
    crate::virtual_method! { pub const VFUNC_UNK_06: usize = 0x06; pub fn unk_06() }
    crate::virtual_method! { pub const VFUNC_UNK_07: usize = 0x07; pub fn unk_07() }
    crate::virtual_method! { pub const VFUNC_UNK_08: usize = 0x08; pub fn unk_08() }
    crate::virtual_method! { pub const VFUNC_UNK_09: usize = 0x09; pub fn unk_09() }
    crate::virtual_method! { pub const VFUNC_UNK_0A: usize = 0x0A; pub fn unk_0a() }
    crate::virtual_method! { pub const VFUNC_UNK_0B: usize = 0x0B; pub fn unk_0b() }
    crate::virtual_method! { pub const VFUNC_UNK_0C: usize = 0x0C; pub fn unk_0c() }

    crate::relocation_func! {
        pub fn bind_object(&mut self, object_ptr: &mut BSTSmartPointer<Object>, handle: VMHandle) => RelocationID::new(97379, 104184)
    }

    crate::relocation_func! {
        pub fn get_initial_property_values(
            &self,
            handle: VMHandle,
            class_name: &BSFixedString,
            property_values: &mut BSTScrapHashMap<BSFixedString, Variable>,
            non_converted_properties: &mut u32
        ) => RelocationID::new(97371, 104176)
    }
}

pub trait ObjectBindPolicyExt: AsRef<ObjectBindPolicy> + AsMut<ObjectBindPolicy> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn unk_01(&mut self, handle: VMHandle) {
        self.as_mut().unk_01(handle)
    }

    #[inline(always)]
    fn unk_02(&mut self, handle: VMHandle) {
        self.as_mut().unk_02(handle)
    }

    #[inline(always)]
    fn unk_03(&mut self, handle: VMHandle) {
        self.as_mut().unk_03(handle)
    }

    #[inline(always)]
    fn unk_04(&mut self) {
        self.as_mut().unk_04()
    }

    #[inline(always)]
    fn unk_05(&mut self) {
        self.as_mut().unk_05()
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        self.as_mut().unk_06()
    }

    #[inline(always)]
    fn unk_07(&mut self) {
        self.as_mut().unk_07()
    }

    #[inline(always)]
    fn unk_08(&mut self) {
        self.as_mut().unk_08()
    }

    #[inline(always)]
    fn unk_09(&mut self) {
        self.as_mut().unk_09()
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        self.as_mut().unk_0a()
    }

    #[inline(always)]
    fn unk_0b(&mut self) {
        self.as_mut().unk_0b()
    }

    #[inline(always)]
    fn unk_0c(&mut self) {
        self.as_mut().unk_0c()
    }

    #[inline(always)]
    fn bind_object(&mut self, object_ptr: &mut BSTSmartPointer<Object>, handle: VMHandle) {
        self.as_mut().bind_object(object_ptr, handle)
    }

    #[inline(always)]
    fn get_initial_property_values(
        &self,
        handle: VMHandle,
        class_name: &BSFixedString,
        property_values: &mut BSTScrapHashMap<BSFixedString, Variable>,
        non_converted_properties: &mut u32,
    ) {
        self.as_ref().get_initial_property_values(
            handle,
            class_name,
            property_values,
            non_converted_properties,
        )
    }
}

impl<T> ObjectBindPolicyExt for T where T: AsRef<ObjectBindPolicy> + AsMut<ObjectBindPolicy> {}
