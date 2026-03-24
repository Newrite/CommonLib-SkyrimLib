use core::ops::{Index, IndexMut};

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraMissingLinkedRefIDs;
use crate::offsets::offsets_vtable::VTABLE_ExtraMissingLinkedRefIDs;
use crate::re::tes_form::FormID;
use crate::re::{BGSKeyword, BSExtraData, ExtraDataType, ExtraDataTyped, TESForm, TESObjectREFR};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraMissingLinkedRefIDs::Entry`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExtraMissingLinkedRefIDsEntry {
    pub keyword: *mut BGSKeyword, // 00
    pub linked_ref_id: FormID,    // 08
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<ExtraMissingLinkedRefIDsEntry>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsEntry, keyword) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsEntry, linked_ref_id) == 0x08);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsEntry, pad0c) == 0x0C);

/// C++ `RE::ExtraMissingLinkedRefIDs::Array::Data`
#[repr(C)]
#[derive(Clone, Copy)]
pub union ExtraMissingLinkedRefIDsArrayData {
    pub entry_ptr: *mut ExtraMissingLinkedRefIDsEntry,
    pub entry: [ExtraMissingLinkedRefIDsEntry; 1],
}

const _: () = assert!(core::mem::size_of::<ExtraMissingLinkedRefIDsArrayData>() == 0x10);

/// C++ `RE::ExtraMissingLinkedRefIDs::Array`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExtraMissingLinkedRefIDsArray {
    pub data: ExtraMissingLinkedRefIDsArrayData, // 00
    pub size: u32,                               // 10
    pub pad14: u32,                              // 14
}

const _: () = assert!(core::mem::size_of::<ExtraMissingLinkedRefIDsArray>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsArray, data) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsArray, size) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDsArray, pad14) == 0x14);

impl ExtraMissingLinkedRefIDsArray {
    #[inline(always)]
    pub const fn size(&self) -> u32 {
        self.size
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub fn begin(&self) -> *const ExtraMissingLinkedRefIDsEntry {
        if self.size > 1 {
            unsafe { self.data.entry_ptr }
        } else {
            unsafe { self.data.entry.as_ptr() }
        }
    }

    #[inline(always)]
    pub fn begin_mut(&mut self) -> *mut ExtraMissingLinkedRefIDsEntry {
        if self.size > 1 {
            unsafe { self.data.entry_ptr }
        } else {
            unsafe { self.data.entry.as_mut_ptr() }
        }
    }

    #[inline(always)]
    pub fn end(&self) -> *const ExtraMissingLinkedRefIDsEntry {
        unsafe { self.begin().add(self.size as usize) }
    }

    #[inline(always)]
    pub fn end_mut(&mut self) -> *mut ExtraMissingLinkedRefIDsEntry {
        unsafe { self.begin_mut().add(self.size as usize) }
    }

    #[inline(always)]
    pub fn get(&self, pos: u32) -> &ExtraMissingLinkedRefIDsEntry {
        assert!(pos < self.size());
        unsafe { &*self.begin().add(pos as usize) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: u32) -> &mut ExtraMissingLinkedRefIDsEntry {
        assert!(pos < self.size());
        unsafe { &mut *self.begin_mut().add(pos as usize) }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[ExtraMissingLinkedRefIDsEntry] {
        if self.is_empty() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.begin(), self.size as usize) }
        }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [ExtraMissingLinkedRefIDsEntry] {
        if self.is_empty() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.begin_mut(), self.size as usize) }
        }
    }
}

impl Index<u32> for ExtraMissingLinkedRefIDsArray {
    type Output = ExtraMissingLinkedRefIDsEntry;

    #[inline(always)]
    fn index(&self, index: u32) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<u32> for ExtraMissingLinkedRefIDsArray {
    #[inline(always)]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        self.get_mut(index)
    }
}

/// C++ `RE::ExtraMissingLinkedRefIDs`
#[repr(C)]
pub struct ExtraMissingLinkedRefIDs {
    pub base: BSExtraData,                      // 00
    pub entries: ExtraMissingLinkedRefIDsArray, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraMissingLinkedRefIDs>() == 0x28);
const _: () = assert!(core::mem::offset_of!(ExtraMissingLinkedRefIDs, entries) == 0x10);

impl RttiType for ExtraMissingLinkedRefIDs {
    const RTTI: VariantID = RTTI_ExtraMissingLinkedRefIDs;
}

impl ExtraDataTyped for ExtraMissingLinkedRefIDs {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::MissingLinkedRefIDs;
}

inherit!(ExtraMissingLinkedRefIDs : BSExtraData);

impl ExtraMissingLinkedRefIDs {
    pub const RTTI: VariantID = RTTI_ExtraMissingLinkedRefIDs;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraMissingLinkedRefIDs;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::MissingLinkedRefIDs;

    // override (BSExtraData)
    // ~ExtraMissingLinkedRefIDs() override;  // 00
    // ExtraDataType GetType() const override;  // 01 - { return kMissingLinkedRefIDs; }

    pub fn get_linked_ref(&mut self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR {
        for entry in self.entries.as_slice() {
            if entry.keyword == keyword {
                if let Some(form) = TESForm::lookup_by_id(entry.linked_ref_id) {
                    return unsafe { (*form).as_reference() };
                }
                return core::ptr::null_mut();
            }
        }
        core::ptr::null_mut()
    }
}
