use crate::offsets::offsets_rtti::RTTI_TESPackageData;
use crate::offsets::offsets_vtable::VTABLE_TESPackageData;
use crate::re::BGSLoadFormBuffer;
use crate::re::BGSSaveFormBuffer;
use crate::re::TESForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::TESPackageData`
#[repr(C)]
pub struct TESPackageData {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<TESPackageData>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESPackageData, vtable) == 0x00);

impl RttiType for TESPackageData {
    const RTTI: VariantID = RTTI_TESPackageData;
}

impl AsRef<TESPackageData> for TESPackageData {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESPackageData> for TESPackageData {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl TESPackageData {
    pub const RTTI: VariantID = RTTI_TESPackageData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESPackageData;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_COPY: usize = 0x01;
        pub fn copy(&mut self, package: *mut TESPackageData, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_COMPARE: usize = 0x02;
        pub fn compare(package: *mut TESPackageData) -> bool
    }

    virtual_method! {
        pub const VFUNC_INIT_ITEM: usize = 0x03;
        pub fn init_item(&mut self, form: *mut TESForm)
    }

    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x04;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x05;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }
}

pub trait TESPackageDataExt {
    fn dtor(&mut self);
    fn copy(&mut self, package: *mut TESPackageData, form: *mut TESForm);
    fn compare(&self, package: *mut TESPackageData) -> bool;
    fn init_item(&mut self, form: *mut TESForm);
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
}

impl<T: AsRef<TESPackageData> + AsMut<TESPackageData>> TESPackageDataExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        TESPackageData::dtor(self.as_mut())
    }

    #[inline(always)]
    fn copy(&mut self, package: *mut TESPackageData, form: *mut TESForm) {
        TESPackageData::copy(self.as_mut(), package, form)
    }

    #[inline(always)]
    fn compare(&self, package: *mut TESPackageData) -> bool {
        self.as_ref().compare(package)
    }

    #[inline(always)]
    fn init_item(&mut self, form: *mut TESForm) {
        TESPackageData::init_item(self.as_mut(), form)
    }

    #[inline(always)]
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESPackageData::save_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESPackageData::load_game(self.as_mut(), buf)
    }
}
