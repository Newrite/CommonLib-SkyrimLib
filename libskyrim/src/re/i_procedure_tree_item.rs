use crate::offsets::offsets_rtti::RTTI_IProcedureTreeItem;
use crate::offsets::offsets_vtable::VTABLE_IProcedureTreeItem;
use crate::re::TESFile;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IProcedureTreeItem`
#[repr(C)]
pub struct IProcedureTreeItem {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IProcedureTreeItem>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IProcedureTreeItem, vtable) == 0x00);

impl RttiType for IProcedureTreeItem {
    const RTTI: VariantID = RTTI_IProcedureTreeItem;
}

impl AsRef<IProcedureTreeItem> for IProcedureTreeItem {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IProcedureTreeItem> for IProcedureTreeItem {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IProcedureTreeItem {
    pub const RTTI: VariantID = RTTI_IProcedureTreeItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IProcedureTreeItem;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01()
    }

    virtual_method! {
        pub const VFUNC_UNK_02: usize = 0x02;
        pub fn unk_02()
    }

    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x03;
        pub fn load(mod_file: *mut TESFile)
    }

    virtual_method! {
        pub const VFUNC_UNK_04: usize = 0x04;
        pub fn unk_04()
    }

    virtual_method! {
        pub const VFUNC_UNK_05: usize = 0x05;
        pub fn unk_05()
    }

    virtual_method! {
        pub const VFUNC_UNK_06: usize = 0x06;
        pub fn unk_06()
    }

    virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07()
    }

    virtual_method! {
        pub const VFUNC_UNK_08: usize = 0x08;
        pub fn unk_08()
    }

    virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09()
    }

    virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a()
    }

    virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b()
    }

    virtual_method! {
        pub const VFUNC_UNK_0C: usize = 0x0C;
        pub fn unk_0c()
    }

    virtual_method! {
        pub const VFUNC_UNK_0D: usize = 0x0D;
        pub fn unk_0d()
    }

    virtual_method! {
        pub const VFUNC_UNK_0E: usize = 0x0E;
        pub fn unk_0e()
    }

    virtual_method! {
        pub const VFUNC_UNK_0F: usize = 0x0F;
        pub fn unk_0f()
    }

    virtual_method! {
        pub const VFUNC_UNK_10: usize = 0x10;
        pub fn unk_10()
    }
}

pub trait IProcedureTreeItemExt {
    fn dtor(&mut self);
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn load(&mut self, mod_file: *mut TESFile);
    fn unk_04(&mut self);
    fn unk_05(&mut self);
    fn unk_06(&mut self);
    fn unk_07(&mut self);
    fn unk_08(&mut self);
    fn unk_09(&mut self);
    fn unk_0a(&mut self);
    fn unk_0b(&mut self);
    fn unk_0c(&mut self);
    fn unk_0d(&mut self);
    fn unk_0e(&mut self);
    fn unk_0f(&mut self);
    fn unk_10(&mut self);
}

impl<T: AsRef<IProcedureTreeItem> + AsMut<IProcedureTreeItem>> IProcedureTreeItemExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        IProcedureTreeItem::dtor(self.as_mut())
    }

    #[inline(always)]
    fn unk_01(&mut self) {
        IProcedureTreeItem::unk_01(self.as_mut())
    }

    #[inline(always)]
    fn unk_02(&mut self) {
        IProcedureTreeItem::unk_02(self.as_mut())
    }

    #[inline(always)]
    fn load(&mut self, mod_file: *mut TESFile) {
        IProcedureTreeItem::load(self.as_mut(), mod_file)
    }

    #[inline(always)]
    fn unk_04(&mut self) {
        IProcedureTreeItem::unk_04(self.as_mut())
    }

    #[inline(always)]
    fn unk_05(&mut self) {
        IProcedureTreeItem::unk_05(self.as_mut())
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        IProcedureTreeItem::unk_06(self.as_mut())
    }

    #[inline(always)]
    fn unk_07(&mut self) {
        IProcedureTreeItem::unk_07(self.as_mut())
    }

    #[inline(always)]
    fn unk_08(&mut self) {
        IProcedureTreeItem::unk_08(self.as_mut())
    }

    #[inline(always)]
    fn unk_09(&mut self) {
        IProcedureTreeItem::unk_09(self.as_mut())
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        IProcedureTreeItem::unk_0a(self.as_mut())
    }

    #[inline(always)]
    fn unk_0b(&mut self) {
        IProcedureTreeItem::unk_0b(self.as_mut())
    }

    #[inline(always)]
    fn unk_0c(&mut self) {
        IProcedureTreeItem::unk_0c(self.as_mut())
    }

    #[inline(always)]
    fn unk_0d(&mut self) {
        IProcedureTreeItem::unk_0d(self.as_mut())
    }

    #[inline(always)]
    fn unk_0e(&mut self) {
        IProcedureTreeItem::unk_0e(self.as_mut())
    }

    #[inline(always)]
    fn unk_0f(&mut self) {
        IProcedureTreeItem::unk_0f(self.as_mut())
    }

    #[inline(always)]
    fn unk_10(&mut self) {
        IProcedureTreeItem::unk_10(self.as_mut())
    }
}
