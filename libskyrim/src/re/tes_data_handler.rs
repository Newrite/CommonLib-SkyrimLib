use crate::relocation::{RelocationID, VariantOffset};
use core::ffi::c_char;
use core_util::inherit;

use crate::re::bgs_addon_node::BGSAddonNode;
use crate::re::bgs_primitive::BGSPrimitive;
use crate::re::bs_pointer_handle::ObjectRefHandle;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::bst_array::BSTArray;
use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::inventory_changes::InventoryChanges;
use crate::re::ni_point3::NiPoint3;
use crate::re::ni_t_array::NiTPrimitiveArray;
use crate::re::ni_t_list::NiTList;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_file::TESFile;
use crate::re::tes_form::{FormID, TESForm};
use crate::re::tes_object_cell::TESObjectCELL;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_region_data_manager::TESRegionDataManager;
use crate::re::tes_region_list::TESRegionList;
use crate::re::tes_world_space::TESWorldSpace;
use crate::rex::W32::{GetModuleHandleW, GetProcAddress};
use crate::{
    runtime_data_accessor, runtime_data_mut_accessor, runtime_data_ptr_accessor,
    runtime_pointer_accessor,
};

/// C++ `RE::TESObjectList`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESObjectList {
    pub pad0: u8, // 00
}

const _: () = assert!(core::mem::size_of::<TESObjectList>() == 0x1);

/// C++ `RE::TESFileCollection`
#[repr(C)]
pub struct TESFileCollection {
    pub files: BSTArray<*mut TESFile>,       // 00
    pub small_files: BSTArray<*mut TESFile>, // 18
}

const _: () = assert!(core::mem::size_of::<TESFileCollection>() == 0x30);

/// C++ `RE::TESDataHandler::RUNTIME_DATA`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RUNTIME_DATA {
    pub master_save: bool,       // 00
    pub block_save: bool,        // 01
    pub save_load_game: bool,    // 02
    pub auto_saving: bool,       // 03
    pub exporting_plugin: bool,  // 04
    pub clearing_data: bool,     // 05
    pub has_desired_files: bool, // 06
    pub checking_models: bool,   // 07
    pub loading_files: bool,     // 08
    pub dont_remove_ids: bool,   // 09
}

const _: () = assert!(core::mem::size_of::<RUNTIME_DATA>() == 0xA);

pub type TESDataHandlerBase = BSTSingletonSDM<TESDataHandler>;
const FORM_ARRAY_COUNT: usize = FormType::Max as usize;

const _: () = assert!(core::mem::size_of::<TESDataHandlerBase>() == 0x1);

/// C++ `RE::TESDataHandler`
///
/// This Rust struct models the common prefix shared by SE/AE and VR.
/// The trailing runtime-divergent tail starting at `0xD70` is accessed through
/// runtime-aware helper methods instead of baked into the physical layout.
#[repr(C)]
pub struct TESDataHandler {
    pub base: TESDataHandlerBase,                                // 000
    pub pad001: u8,                                              // 001
    pub pad002: u16,                                             // 002
    pub pad004: u32,                                             // 004
    pub object_list: *mut TESObjectList,                         // 008
    pub form_arrays: [BSTArray<*mut TESForm>; FORM_ARRAY_COUNT], // 010
    pub region_list: *mut TESRegionList,                         // D00
    pub interior_cells: NiTPrimitiveArray<*mut TESObjectCELL>,   // D08
    pub addon_nodes: NiTPrimitiveArray<*mut BGSAddonNode>,       // D20
    pub bad_forms: NiTList<*mut TESForm>,                        // D38
    pub next_id: FormID,                                         // D50
    pub padd54: u32,                                             // D54
    pub active_file: *mut TESFile,                               // D58
    pub files: BSSimpleList<*mut TESFile>,                       // D60
}

const _: () = assert!(core::mem::size_of::<TESDataHandler>() == 0xD70);
const _: () = assert!(core::mem::offset_of!(TESDataHandler, object_list) == 0x8);
const _: () = assert!(core::mem::offset_of!(TESDataHandler, form_arrays) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESDataHandler, region_list) == 0xD00);
const _: () = assert!(core::mem::offset_of!(TESDataHandler, files) == 0xD60);

inherit!(TESDataHandler : TESDataHandlerBase);

static mut VR_COMPILED_FILE_COLLECTION: *mut TESFileCollection = core::ptr::null_mut();

impl TESDataHandler {
    pub const GEOMETRY_RUNTIME_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0xDA0, 0xDA0, 0x1570);
    pub const GAME_SETTINGS_LOAD_STATE_OFFSET: VariantOffset =
        VariantOffset::new(0xDAA, 0xDAA, 0x157A);
    pub const REGION_DATA_MANAGER_OFFSET: VariantOffset = VariantOffset::new(0xDB0, 0xDB0, 0x1580);
    pub const MERCHANT_INVENTORY_OFFSET: VariantOffset = VariantOffset::new(0xDB8, 0xDB8, 0x1588);
    pub const COMPILED_FILE_COLLECTION_OFFSET: VariantOffset =
        VariantOffset::new(0xD70, 0xD70, 0x0);
    pub const VR_LOADED_MOD_COUNT_FALLBACK_OFFSET: VariantOffset =
        VariantOffset::new(0x0, 0x0, 0xD70);
    pub const VR_LOADED_MODS_FALLBACK_OFFSET: VariantOffset = VariantOffset::new(0x0, 0x0, 0xD78);

    // RELOCATION_ID SE: 514141, AE: 400269
    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut TESDataHandler => RelocationID::new(514141, 400269)
    }

    #[inline]
    pub fn get_singleton(vresl: bool) -> *mut TESDataHandler {
        if crate::runtime::is_vr() && vresl {
            unsafe {
                if VR_COMPILED_FILE_COLLECTION.is_null() {
                    let module_name_storage = core_util::create_utf16_string::<12>("skyrimvresl");
                    let module_name = core_util::WideStr::from_slice(&module_name_storage);
                    let module = GetModuleHandleW(module_name.as_ptr());
                    if !module.is_null() {
                        let proc =
                            GetProcAddress(module, b"GetCompiledFileCollectionExtern\0".as_ptr());
                        if let Some(fn_ptr) = proc {
                            let get_compiled_file_collection: unsafe extern "C" fn() -> *const TESFileCollection =
                                core::mem::transmute(fn_ptr);
                            let ptr = get_compiled_file_collection() as *mut TESFileCollection;
                            if !ptr.is_null() {
                                VR_COMPILED_FILE_COLLECTION = ptr;
                            }
                        }
                    }
                }
            }
        }

        *Self::singleton_ptr()
    }

    // RELOCATION_ID SE: 13597, AE: 13693
    crate::relocation_func! {
        pub fn add_form_to_data_handler(&mut self, form: *mut TESForm) -> bool => RelocationID::new(13597, 13693)
    }

    // RELOCATION_ID SE: 13657, AE: 13766
    crate::relocation_func! {
        pub fn load_scripts(&mut self) -> u32 => RelocationID::new(13657, 13766)
    }

    pub fn lookup_form(&self, local_form_id: FormID, mod_name: &str) -> *mut TESForm {
        let form_id = self.lookup_form_id(local_form_id, mod_name);
        if form_id == 0 {
            core::ptr::null_mut()
        } else {
            TESForm::lookup_by_id(form_id).unwrap_or(core::ptr::null_mut())
        }
    }

    pub fn lookup_form_raw(&self, raw_form_id: FormID, mod_name: &str) -> *mut TESForm {
        let form_id = self.lookup_form_id_raw(raw_form_id, mod_name);
        if form_id == 0 {
            core::ptr::null_mut()
        } else {
            TESForm::lookup_by_id(form_id).unwrap_or(core::ptr::null_mut())
        }
    }

    pub fn lookup_form_typed<T: FormCastable>(
        &self,
        local_form_id: FormID,
        mod_name: &str,
    ) -> *mut T {
        let form = self.lookup_form(local_form_id, mod_name);
        if form.is_null() {
            return core::ptr::null_mut();
        }

        if unsafe { (*form).is(T::TARGET_FORM_TYPE) } {
            form.cast::<T>()
        } else {
            core::ptr::null_mut()
        }
    }

    pub fn lookup_form_raw_typed<T: FormCastable>(
        &self,
        raw_form_id: FormID,
        mod_name: &str,
    ) -> *mut T {
        let form = self.lookup_form_raw(raw_form_id, mod_name);
        if form.is_null() {
            return core::ptr::null_mut();
        }

        if unsafe { (*form).is(T::TARGET_FORM_TYPE) } {
            form.cast::<T>()
        } else {
            core::ptr::null_mut()
        }
    }

    pub fn lookup_form_id(&self, local_form_id: FormID, mod_name: &str) -> FormID {
        let file = self.lookup_mod_by_name(mod_name);
        if file.is_null() {
            return 0;
        }

        let file = unsafe { &*file };
        if file.compile_index == 0xFF {
            return 0;
        }

        if crate::runtime::is_vr() && unsafe { VR_COMPILED_FILE_COLLECTION.is_null() } {
            (local_form_id & 0x00FF_FFFF) | ((file.compile_index as FormID) << 24)
        } else {
            let mut form_id = (file.compile_index as FormID) << 24;
            form_id += (file.small_file_compile_index as FormID) << 12;
            form_id + local_form_id
        }
    }

    pub fn lookup_form_id_raw(&self, raw_form_id: FormID, mod_name: &str) -> FormID {
        let file = self.lookup_mod_by_name(mod_name);
        if file.is_null() {
            return 0;
        }

        let file = unsafe { &*file };
        if file.compile_index == 0xFF {
            return 0;
        }

        if file.master_ptrs.is_null() {
            return 0;
        }

        let mut raw_index = ((raw_form_id & 0xFF00_0000) >> 24) as u32;
        if crate::runtime::is_vr() && unsafe { VR_COMPILED_FILE_COLLECTION.is_null() } {
            if raw_index >= file.master_count {
                return 0;
            }

            let master = unsafe { *file.master_ptrs.add(raw_index as usize) };
            if master.is_null() {
                return 0;
            }

            let master = unsafe { &*master };
            (raw_form_id & 0x00FF_FFFF) | ((master.compile_index as FormID) << 24)
        } else {
            let is_light = raw_index == 0xFE;
            if is_light {
                raw_index = (raw_form_id & 0x00FF_F000) >> 12;
            }

            let mut index = 0u32;
            for i in 0..file.master_count {
                let master = unsafe { *file.master_ptrs.add(i as usize) };
                if master.is_null() {
                    continue;
                }

                let master = unsafe { &*master };
                if (master.compile_index == 0xFE) != is_light {
                    continue;
                }

                if index == raw_index {
                    return (raw_form_id & 0x00FF_FFFF) | ((master.compile_index as FormID) << 24);
                }

                index += 1;
            }
            0
        }
    }

    pub fn lookup_mod_by_name(&self, mod_name: &str) -> *const TESFile {
        for file in self.files.iter().copied() {
            if file.is_null() {
                continue;
            }
            let file_ref = unsafe { &*file };
            if Self::mod_name_matches(file_ref.file_name.as_ptr(), mod_name) {
                return file;
            }
        }
        core::ptr::null()
    }

    #[inline]
    pub fn get_mod_index(&self, mod_name: &str) -> Option<u8> {
        let mod_file = self.lookup_mod_by_name(mod_name);
        if mod_file.is_null() {
            None
        } else {
            Some(unsafe { (*mod_file).compile_index })
        }
    }

    #[inline]
    pub fn get_loaded_mods_mut(&mut self) -> *mut *mut TESFile {
        if crate::runtime::is_vr() {
            let compiled = unsafe { VR_COMPILED_FILE_COLLECTION };
            if !compiled.is_null() {
                return unsafe { (*compiled).files.data_mut() };
            }

            // Skyrim VR fallback when SkyrimVRESL is unavailable.
            self.vr_loaded_mods_fallback_ptr()
        } else {
            unsafe { (*self.compiled_file_collection_ptr()).files.data_mut() }
        }
    }

    #[inline]
    pub fn get_loaded_mods(&self) -> *const *const TESFile {
        if crate::runtime::is_vr() {
            let compiled = unsafe { VR_COMPILED_FILE_COLLECTION };
            if !compiled.is_null() {
                return unsafe { (*compiled).files.data().cast() };
            }

            // Skyrim VR fallback when SkyrimVRESL is unavailable.
            self.vr_loaded_mods_fallback_ptr().cast_const().cast()
        } else {
            unsafe { (*self.compiled_file_collection_ptr()).files.data().cast() }
        }
    }

    #[inline]
    pub fn get_loaded_mod_count(&self) -> u8 {
        if crate::runtime::is_vr() {
            let compiled = unsafe { VR_COMPILED_FILE_COLLECTION };
            if !compiled.is_null() {
                return unsafe { (*compiled).files.len() as u8 };
            }

            self.vr_loaded_mod_count_fallback() as u8
        } else {
            unsafe { (*self.compiled_file_collection_ptr()).files.len() as u8 }
        }
    }

    #[inline]
    pub fn get_loaded_light_mods(&self) -> *const *const TESFile {
        if crate::runtime::is_vr() {
            let compiled = unsafe { VR_COMPILED_FILE_COLLECTION };
            if compiled.is_null() {
                return core::ptr::null();
            }
            unsafe { (*compiled).small_files.data().cast() }
        } else {
            unsafe {
                (*self.compiled_file_collection_ptr())
                    .small_files
                    .data()
                    .cast()
            }
        }
    }

    #[inline]
    pub fn get_loaded_light_mod_count(&self) -> u16 {
        if crate::runtime::is_vr() {
            let compiled = unsafe { VR_COMPILED_FILE_COLLECTION };
            if compiled.is_null() {
                return 0;
            }
            unsafe { (*compiled).small_files.len() as u16 }
        } else {
            unsafe { (*self.compiled_file_collection_ptr()).small_files.len() as u16 }
        }
    }

    pub fn lookup_loaded_mod_by_name(&self, mod_name: &str) -> *const TESFile {
        let mut file = self.get_loaded_mods();
        for _ in 0..self.get_loaded_mod_count() {
            if file.is_null() {
                break;
            }

            let current = unsafe { *file };
            if !current.is_null()
                && Self::mod_name_matches(unsafe { (*current).file_name.as_ptr() }, mod_name)
            {
                return current;
            }

            file = unsafe { file.add(1) };
        }
        core::ptr::null()
    }

    pub fn lookup_loaded_mod_by_index(&self, index: u8) -> *const TESFile {
        let mut file = self.get_loaded_mods();
        for _ in 0..self.get_loaded_mod_count() {
            if file.is_null() {
                break;
            }

            let current = unsafe { *file };
            if !current.is_null() && unsafe { (*current).compile_index == index } {
                return current;
            }

            file = unsafe { file.add(1) };
        }
        core::ptr::null()
    }

    #[inline]
    pub fn get_loaded_mod_index(&self, mod_name: &str) -> Option<u8> {
        let mod_file = self.lookup_loaded_mod_by_name(mod_name);
        if mod_file.is_null() {
            None
        } else {
            Some(unsafe { (*mod_file).compile_index })
        }
    }

    pub fn lookup_loaded_light_mod_by_name(&self, mod_name: &str) -> *const TESFile {
        let mut file = self.get_loaded_light_mods();
        for _ in 0..self.get_loaded_light_mod_count() {
            if file.is_null() {
                break;
            }

            let current = unsafe { *file };
            if !current.is_null()
                && Self::mod_name_matches(unsafe { (*current).file_name.as_ptr() }, mod_name)
            {
                return current;
            }

            file = unsafe { file.add(1) };
        }
        core::ptr::null()
    }

    pub fn lookup_loaded_light_mod_by_index(&self, index: u16) -> *const TESFile {
        let mut file = self.get_loaded_light_mods();
        for _ in 0..self.get_loaded_light_mod_count() {
            if file.is_null() {
                break;
            }

            let current = unsafe { *file };
            if !current.is_null() && unsafe { (*current).small_file_compile_index == index } {
                return current;
            }

            file = unsafe { file.add(1) };
        }
        core::ptr::null()
    }

    #[inline]
    pub fn get_loaded_light_mod_index(&self, mod_name: &str) -> Option<u16> {
        let mod_file = self.lookup_loaded_light_mod_by_name(mod_name);
        if mod_file.is_null() {
            None
        } else {
            Some(unsafe { (*mod_file).small_file_compile_index })
        }
    }

    // RELOCATION_ID SE: 13618, AE: 13716
    crate::relocation_func! {
        pub fn get_ext_cell_data_from_file_by_editor_id(&mut self, cell_id: *const c_char, out_x: &mut i32, out_y: &mut i32) -> *mut TESWorldSpace
            => RelocationID::new(13618, 13716)
    }

    #[inline(always)]
    pub const fn is_generated_id(&self, form_id: FormID) -> bool {
        form_id >= 0xFF00_0000
    }

    // RELOCATION_ID SE: 13635, AE: 13740
    crate::relocation_func! {
        pub fn get_next_id(&mut self) -> FormID => RelocationID::new(13635, 13740)
    }

    #[inline]
    pub fn get_form_array(&mut self, form_type: FormType) -> &mut BSTArray<*mut TESForm> {
        let idx = form_type as usize;
        debug_assert!(idx < FORM_ARRAY_COUNT);
        &mut self.form_arrays[idx]
    }

    #[inline]
    pub fn get_form_array_typed<T: FormCastable>(&mut self) -> &mut BSTArray<*mut T> {
        let array = self.get_form_array(T::TARGET_FORM_TYPE) as *mut BSTArray<*mut TESForm>;
        unsafe { &mut *array.cast::<BSTArray<*mut T>>() }
    }

    // RELOCATION_ID SE: 13625, AE: 13723
    crate::relocation_func! {
        pub fn create_reference_at_location(
            &mut self,
            base: *mut TESBoundObject,
            location: &NiPoint3,
            rotation: &NiPoint3,
            target_cell: *mut TESObjectCELL,
            self_world_space: *mut TESWorldSpace,
            already_created_ref: *mut TESObjectREFR,
            primitive: *mut BGSPrimitive,
            linked_room_ref_handle: &ObjectRefHandle,
            force_persist: bool,
            arg11: bool
        ) -> ObjectRefHandle => RelocationID::new(13625, 13723)
    }

    runtime_data_accessor! {
        pub fn get_geometry_runtime_data() -> RUNTIME_DATA {
            offset: Self::GEOMETRY_RUNTIME_DATA_OFFSET
        }
    }

    runtime_data_mut_accessor! {
        pub fn get_geometry_runtime_data_mut() -> RUNTIME_DATA {
            offset: Self::GEOMETRY_RUNTIME_DATA_OFFSET
        }
    }

    runtime_pointer_accessor! {
        pub fn get_game_settings_load_state() -> u8 {
            offset: Self::GAME_SETTINGS_LOAD_STATE_OFFSET
        }
    }

    runtime_pointer_accessor! {
        pub fn get_region_data_manager() -> *mut TESRegionDataManager {
            offset: Self::REGION_DATA_MANAGER_OFFSET
        }
    }

    runtime_pointer_accessor! {
        pub fn get_merchant_inventory() -> *mut InventoryChanges {
            offset: Self::MERCHANT_INVENTORY_OFFSET
        }
    }

    runtime_data_ptr_accessor! {
        fn compiled_file_collection_ptr() -> TESFileCollection {
            offset: Self::COMPILED_FILE_COLLECTION_OFFSET
        }
    }

    runtime_data_ptr_accessor! {
        fn vr_loaded_mods_fallback_ptr() -> *mut TESFile {
            offset: Self::VR_LOADED_MODS_FALLBACK_OFFSET
        }
    }

    runtime_pointer_accessor! {
        fn vr_loaded_mod_count_fallback() -> u32 {
            offset: Self::VR_LOADED_MOD_COUNT_FALLBACK_OFFSET
        }
    }

    #[inline]
    fn mod_name_matches(file_name: *const c_char, mod_name: &str) -> bool {
        if file_name.is_null() {
            return false;
        }
        let file_name = core_util::ptr_to_str(file_name);
        file_name.len() == mod_name.len() && file_name.eq_ignore_ascii_case(mod_name)
    }
}
