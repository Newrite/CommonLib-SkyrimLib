use crate::offsets::offsets_rtti::RTTI_BGSOpenCloseForm;
use crate::offsets::offsets_vtable::VTABLE_BGSOpenCloseForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpenState {
    None = 0,
    Open = 1,
    Opening = 2,
    Closed = 3,
    Closing = 4,
}

/// C++ `RE::BGSOpenCloseForm`
#[repr(C)]
pub struct BGSOpenCloseForm {
    pub vtable: *const usize,
}

const _: () = assert!(core::mem::size_of::<BGSOpenCloseForm>() == 0x8);

impl RttiType for BGSOpenCloseForm {
    const RTTI: VariantID = RTTI_BGSOpenCloseForm;
}

impl BGSOpenCloseForm {
    pub const RTTI: VariantID = RTTI_BGSOpenCloseForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSOpenCloseForm;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_HANDLE_OPEN: usize = 0x01;
        pub fn handle_open(
            &self,
            target: *mut TESObjectREFR,
            activator: *mut TESObjectREFR
        )
    }

    virtual_method! {
        pub const VFUNC_HANDLE_CLOSE: usize = 0x02;
        pub fn handle_close(
            &self,
            target: *mut TESObjectREFR,
            activator: *mut TESObjectREFR
        )
    }

    virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x03;
        pub fn unk_03(&self)
    }

    crate::relocation_func! {
        pub fn get_open_state(reference: *const TESObjectREFR) -> OpenState => RelocationID::new(14180, 14288)
    }

    crate::relocation_func! {
        pub fn set_open_state(reference: *mut TESObjectREFR, open: bool, snap: bool) => RelocationID::new(14179, 14287)
    }
}

pub trait BGSOpenCloseFormExt {
    fn dtor(&mut self);
    fn handle_open(&self, target: *mut TESObjectREFR, activator: *mut TESObjectREFR);
    fn handle_close(&self, target: *mut TESObjectREFR, activator: *mut TESObjectREFR);
    fn unk_03(&self);
}

impl<T: AsRef<BGSOpenCloseForm> + AsMut<BGSOpenCloseForm>> BGSOpenCloseFormExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn handle_open(&self, target: *mut TESObjectREFR, activator: *mut TESObjectREFR) {
        self.as_ref().handle_open(target, activator)
    }

    fn handle_close(&self, target: *mut TESObjectREFR, activator: *mut TESObjectREFR) {
        self.as_ref().handle_close(target, activator)
    }

    fn unk_03(&self) {
        self.as_ref().unk_03()
    }
}
