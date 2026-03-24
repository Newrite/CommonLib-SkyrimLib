use core::ffi::{c_char, c_void};

use crate::offsets::offsets_rtti::RTTI_SettingCollection_Setting_;
use crate::offsets::offsets_vtable::VTABLE_SettingCollection_Setting_;
use crate::re::Setting;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::SettingCollection<T>` translated for generic `T`.
#[repr(C)]
pub struct SettingCollection<T> {
    pub vtable: *const usize,     // 00
    pub sub_key: [c_char; 0x104], // 08
    pub handle: *mut c_void,      // 110
    _marker: core::marker::PhantomData<T>,
}

const _: () = assert!(core::mem::size_of::<SettingCollection<Setting>>() == 0x118);
const _: () = assert!(core::mem::offset_of!(SettingCollection<Setting>, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(SettingCollection<Setting>, sub_key) == 0x08);
const _: () = assert!(core::mem::offset_of!(SettingCollection<Setting>, handle) == 0x110);

pub type SettingCollectionSetting = SettingCollection<Setting>;

impl RttiType for SettingCollection<Setting> {
    const RTTI: VariantID = RTTI_SettingCollection_Setting_;
}

impl<T> AsRef<SettingCollection<T>> for SettingCollection<T> {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<T> AsMut<SettingCollection<T>> for SettingCollection<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl SettingCollection<Setting> {
    pub const RTTI: VariantID = RTTI_SettingCollection_Setting_;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SettingCollection_Setting_;
}

impl<T> SettingCollection<T> {
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_INSERT_SETTING: usize = 0x01;
        pub fn insert_setting(setting: *mut T)
    }

    virtual_method! {
        pub const VFUNC_REMOVE_SETTING: usize = 0x02;
        pub fn remove_setting(setting: *mut T)
    }

    virtual_method! {
        pub const VFUNC_WRITE_SETTING: usize = 0x03;
        pub fn write_setting(setting: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_READ_SETTING: usize = 0x04;
        pub fn read_setting(setting: *mut T) -> bool
    }

    virtual_method! {
        pub const VFUNC_OPEN_HANDLE: usize = 0x05;
        pub fn open_handle(create: bool) -> bool
    }

    virtual_method! {
        pub const VFUNC_CLOSE_HANDLE: usize = 0x06;
        pub fn close_handle() -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07()
    }

    virtual_method! {
        pub const VFUNC_WRITE_ALL_SETTINGS: usize = 0x08;
        pub fn write_all_settings()
    }

    virtual_method! {
        pub const VFUNC_READ_ALL_SETTINGS: usize = 0x09;
        pub fn read_all_settings()
    }

    #[inline(always)]
    pub fn sub_key_as_str(&self) -> &str {
        core_util::ptr_to_str(self.sub_key.as_ptr())
    }
}

pub trait SettingCollectionExt<T> {
    fn dtor(&mut self);
    fn insert_setting(&mut self, setting: *mut T);
    fn remove_setting(&mut self, setting: *mut T);
    fn write_setting(&mut self, setting: *mut T) -> bool;
    fn read_setting(&mut self, setting: *mut T) -> bool;
    fn open_handle(&mut self, create: bool) -> bool;
    fn close_handle(&mut self) -> bool;
    fn unk_07(&mut self);
    fn write_all_settings(&mut self);
    fn read_all_settings(&mut self);
    fn sub_key_as_str<'a>(&'a self) -> &'a str
    where
        T: 'a;
}

impl<T, U> SettingCollectionExt<T> for U
where
    U: AsRef<SettingCollection<T>> + AsMut<SettingCollection<T>>,
{
    fn dtor(&mut self) {
        SettingCollection::dtor(self.as_mut())
    }

    fn insert_setting(&mut self, setting: *mut T) {
        SettingCollection::insert_setting(self.as_mut(), setting)
    }

    fn remove_setting(&mut self, setting: *mut T) {
        SettingCollection::remove_setting(self.as_mut(), setting)
    }

    fn write_setting(&mut self, setting: *mut T) -> bool {
        SettingCollection::write_setting(self.as_mut(), setting)
    }

    fn read_setting(&mut self, setting: *mut T) -> bool {
        SettingCollection::read_setting(self.as_mut(), setting)
    }

    fn open_handle(&mut self, create: bool) -> bool {
        SettingCollection::open_handle(self.as_mut(), create)
    }

    fn close_handle(&mut self) -> bool {
        SettingCollection::close_handle(self.as_mut())
    }

    fn unk_07(&mut self) {
        SettingCollection::unk_07(self.as_mut())
    }

    fn write_all_settings(&mut self) {
        SettingCollection::write_all_settings(self.as_mut())
    }

    fn read_all_settings(&mut self) {
        SettingCollection::read_all_settings(self.as_mut())
    }

    fn sub_key_as_str<'a>(&'a self) -> &'a str
    where
        T: 'a,
    {
        SettingCollection::sub_key_as_str(self.as_ref())
    }
}
