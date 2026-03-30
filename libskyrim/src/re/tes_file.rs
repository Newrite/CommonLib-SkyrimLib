use crate::relocation::RelocationID;
use core::ffi::{c_char, c_void};

use core_util::EnumSet;

use crate::re::bs_core_types::FormID;
use crate::re::bs_file::BSFile;
use crate::re::bs_string::BSString;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::form::Form;
use crate::re::form_type::FormType;
use crate::re::ni_t_pointer_map::NiTPointerMap;
use crate::re::tes_bit_array_file::TESBitArrayFile;
use crate::re::tes_form::TESForm;
use crate::re::tes_object_cell::TESObjectCELL;
use crate::re::tes_world_space::TESWorldSpace;
use crate::rex::W32::{FILETIME, WIN32_FIND_DATAA};

/// C++ `RE::TESFile::Error`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    None = 0,
    NotFound = 1,
    NoFile = 2,
    NoForm = 3,
    NoChunk = 4,
    NoID = 5,
    BadFile = 6,
    BadID = 7,
    FormOpen = 8,
    FileOpen = 9,
    WriteFailure = 10,
    InvalidFile = 11,
    FileInUse = 12,
    CreateFailure = 13,
}

/// C++ `RE::TESFile::RecordFlag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TESFileRecordFlag {
    None = 0,
    Master = 1 << 0,
    Altered = 1 << 1,
    Checked = 1 << 2,
    Active = 1 << 3,
    OptimizedFile = 1 << 4,
    TempIDOwner = 1 << 5,
    Delocalized = 1 << 7,
    PrecalcDataOnly = 1 << 8,
    SmallFile = 1 << 9,
}

core_util::impl_enumset_type!(Error => u32);
core_util::impl_enumset_type!(TESFileRecordFlag => u32);

/// C++ `RE::NiFile::OpenMode`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NiFileOpenMode {
    ReadOnly = 0,
    WriteOnly = 1,
    AppendOnly = 2,
}

/// C++ `RE::TESFile`
#[repr(C)]
pub struct TESFile {
    pub last_error: EnumSet<Error, u32>,                // 000
    pub pad004: u32,                                    // 004
    pub thread_safe_parent: *mut TESFile,               // 008
    pub thread_safe_file_map: *mut NiTPointerMap,       // 010
    pub unk018: u64,                                    // 018
    pub unk020: u64,                                    // 020
    pub unk028: u8,                                     // 028
    pub unk029: bool,                                   // 029
    pub pad02a: u16,                                    // 02A
    pub pad02c: u32,                                    // 02C
    pub locked_file: *mut BSFile,                       // 030
    pub file: *mut BSFile,                              // 038
    pub form_user_data_bit_array: *mut TESBitArrayFile, // 040
    pub form_version_bit_array: *mut TESBitArrayFile,   // 048
    pub form_id_bit_array: *mut TESBitArrayFile,        // 050
    pub file_name: [c_char; 260],                       // 058
    pub path: [c_char; 260],                            // 15C
    pub buffer: *mut c_char,                            // 260
    pub buffer_alloc_size: u32,                         // 268
    pub first_cell_offset: u32,                         // 26C
    pub curr_cell_offset: u32,                          // 270
    pub unk274: u32,                                    // 274
    pub curr_cell: *mut TESObjectCELL,                  // 278
    pub curr_ref_offset: u32,                           // 280
    pub current_form: Form,                             // 284
    pub current_chunk_id: u32,                          // 29C
    pub actual_chunk_size: u32,                         // 2A0
    pub filesize: u32,                                  // 2A4
    pub file_offset: u32,                               // 2A8
    pub form_offset: u32,                               // 2AC
    pub chunk_offset: u32,                              // 2B0
    pub save_form: Form,                                // 2B4
    pub save_form_offset: u32,                          // 2CC
    pub save_chunk_offset: u64,                         // 2D0
    pub unk2d8: u64,                                    // 2D8
    pub unk2e0: u64,                                    // 2E0
    pub unk2e8: u8,                                     // 2E8
    pub is_big_endian: bool,                            // 2E9
    pub unk2ea: u8,                                     // 2EA
    pub pad2eb: u8,                                     // 2EB
    pub file_data: WIN32_FIND_DATAA,                    // 2EC
    pub version: f32,                                   // 42C
    pub form_count: u32,                                // 430
    pub next_form_id: u32,                              // 434
    pub record_flags: EnumSet<TESFileRecordFlag, u32>,  // 438
    pub pad43c: u32,                                    // 43C
    pub masters: BSSimpleList<*const c_char>,           // 440
    pub masters_data: BSSimpleList<*mut u64>,           // 450
    pub master_count: u32,                              // 460
    pub pad464: u32,                                    // 464
    pub master_ptrs: *mut *mut TESFile,                 // 468
    pub deleted_form_time: FILETIME,                    // 470
    pub compile_index: u8,                              // 478
    pub pad479: u8,                                     // 479
    pub small_file_compile_index: u16,                  // 47A
    pub pad47c: u32,                                    // 47C
    pub created_by: BSString,                           // 480
    pub summary: BSString,                              // 490
    pub decompressed_form_buffer: *mut c_char,          // 4A0
    pub decompressed_form_buffer_size: u32,             // 4A8
    pub pad4ac: u32,                                    // 4AC
    pub reserved_decompression_buffer: *mut c_void,     // 4B0
    pub reserved_decompression_buffer_size: u32,        // 4B8
    pub pad4bc: u32,                                    // 4BC
    pub interior_cell_offset_data: *mut c_void,         // 4C0
}

const _: () = assert!(core::mem::size_of::<TESFile>() == 0x4C8);
const _: () = assert!(core::mem::offset_of!(TESFile, file_name) == 0x58);
const _: () = assert!(core::mem::offset_of!(TESFile, current_form) == 0x284);
const _: () = assert!(core::mem::offset_of!(TESFile, record_flags) == 0x438);
const _: () = assert!(core::mem::offset_of!(TESFile, compile_index) == 0x478);

impl TESFile {
    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_combined_index(&self) -> u32 {
        self.compile_index as u32 + self.small_file_compile_index as u32
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_compile_index(&self) -> u8 {
        self.compile_index
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_small_file_compile_index(&self) -> u16 {
        self.small_file_compile_index
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_current_sub_record_size(&self) -> u32 {
        self.actual_chunk_size
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_current_chunk_id(&self) -> u32 {
        self.current_chunk_id
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub fn get_filename_as_str(&self) -> &str {
        core_util::ptr_to_str(self.file_name.as_ptr())
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub fn get_partial_index(&self) -> u32 {
        if self.is_light() {
            0xFE000 | self.small_file_compile_index as u32
        } else {
            self.compile_index as u32
        }
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub fn is_light(&self) -> bool {
        self.record_flags.all(TESFileRecordFlag::SmallFile)
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub fn is_localized(&self) -> bool {
        self.record_flags.all(TESFileRecordFlag::Delocalized)
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub const fn get_form_id(&self, form_lower: FormID) -> u32 {
        (self.compile_index as u32) << 24 | (form_lower & 0x00FF_FFFF)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13857, AE: 13933
    crate::relocation_func! {
        pub fn close_tes(&mut self, force: bool) -> bool => RelocationID::new(13857, 13933)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13923, AE: 14018
    crate::relocation_func! {
        pub fn duplicate(&mut self, cache_size: u32) -> *mut TESFile => RelocationID::new(13923, 14018)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13902, AE: 13988
    crate::relocation_func! {
        pub fn get_current_sub_record_type(&self) -> u32 => RelocationID::new(13902, 13988)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13897, AE: 13982
    crate::relocation_func! {
        pub fn get_form_type(&self) -> FormType => RelocationID::new(13897, 13982)
    }

    // ADDED: gap-fill
    pub fn is_form_in_mod(&self, form_id: FormID) -> bool {
        if !self.is_light() && (form_id >> 24) as u8 == self.compile_index {
            return true;
        }

        if self.is_light()
            && (form_id >> 24) == 0xFE
            && ((form_id & 0x00FF_F000) >> 12) as u16 == self.small_file_compile_index
        {
            return true;
        }

        false
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13855, AE: 13931
    crate::relocation_func! {
        pub fn open_tes(&mut self, access_mode: NiFileOpenMode, lock: bool) -> bool => RelocationID::new(13855, 13931)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13904, AE: 13991
    crate::relocation_func! {
        pub fn read_data(&mut self, buf: *mut c_void, size: u32) -> bool => RelocationID::new(13904, 13991)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13898, AE: 13984
    crate::relocation_func! {
        pub fn seek(&mut self, offset: u32) -> bool => RelocationID::new(13898, 13984)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13894, AE: 13979
    crate::relocation_func! {
        pub fn seek_next_form(&mut self, skip_ignored: bool) -> bool => RelocationID::new(13894, 13979)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13903, AE: 13990
    crate::relocation_func! {
        pub fn seek_next_subrecord(&mut self) -> bool => RelocationID::new(13903, 13990)
    }

    // ADDED: gap-fill
    pub fn seek_next_subrecord_type(&mut self, record_type: u32) -> bool {
        let mut current_type = self.get_current_sub_record_type();
        while current_type != record_type {
            if !self.seek_next_subrecord() {
                return false;
            }
            current_type = self.get_current_sub_record_type();
        }
        true
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 13888, AE: 13972
    crate::relocation_func! {
        pub fn seek_form(&mut self, form: *mut TESForm) -> bool => RelocationID::new(13888, 13972)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 20022, AE: 20456
    crate::relocation_func! {
        fn seek_cell_impl(world_space: *mut TESWorldSpace, file: *mut TESFile, x: i32, y: i32) -> bool => RelocationID::new(20022, 20456)
    }

    // ADDED: gap-fill
    #[inline(always)]
    pub fn seek_cell(&mut self, world_space: *mut TESWorldSpace, x: i32, y: i32) -> bool {
        Self::seek_cell_impl(world_space, self, x, y)
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 18631, AE: 19103
    crate::relocation_func! {
        pub fn seek_landscape_for_current_cell(&mut self) -> bool => RelocationID::new(18631, 19103)
    }

    // ADDED: gap-fill
    pub fn get_runtime_form_id(&self, raw_form_id: FormID) -> u32 {
        if raw_form_id.wrapping_sub(1) <= 0x7FE {
            return raw_form_id;
        }

        let mut owner = self as *const TESFile;
        let master_index = (raw_form_id >> 24) as usize;

        if master_index < self.master_count as usize && !self.master_ptrs.is_null() {
            let owner_ptr = unsafe { *self.master_ptrs.add(master_index) };
            if !owner_ptr.is_null() {
                owner = owner_ptr;
            }
        }

        let owner_ref = unsafe { &*owner };
        if owner_ref.record_flags.any(TESFileRecordFlag::SmallFile) {
            (raw_form_id & 0x0000_0FFF)
                | 0xFE00_0000
                | ((owner_ref.small_file_compile_index as u32) << 12)
        } else {
            (raw_form_id & 0x00FF_FFFF) | ((owner_ref.compile_index as u32) << 24)
        }
    }
}
