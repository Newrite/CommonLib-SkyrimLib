#![allow(non_camel_case_types)]

use core::ffi::c_char;

use core_util::inherit;

use crate::re::{
    GASRefCountCollector, GASStringManager, GFxExporterInfo, GFxMovieView, GFxResource,
    GFxStateBag, GMemoryHeap, GMemoryHeapHeapDesc, GMemoryHeapLimitHandler, GPtr, GPtrTarget,
    GRectF, GRefCountBase, GStatGroups,
};

/// C++ `RE::GFxMovieDef::FileAttrFlags`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieDefFileAttrFlags {
    kUseNetwork = 1 << 0,
    kHasMetadata = 1 << 4,
}

/// C++ `RE::GFxMovieDef::VisitResourceMask`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieDefVisitResourceMask {
    kNestedMovies = 1 << 15,
    kFonts = 1 << 0,
    kBitmaps = 1 << 1,
    kGradientImages = 1 << 2,
    kEditTextFields = 1 << 3,
    kSounds = 1 << 4,
    kSprite = 1 << 5,
    kAllLocalImages = (1 << 1) | (1 << 2),
    kAllImages = (1 << 1) | (1 << 2) | (1 << 15),
}

/// C++ `RE::GFxMovieDef::MemoryParams`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GFxMovieDefMemoryParams {
    pub desc: GMemoryHeapHeapDesc,       // 00
    pub heap_limit_multiplier: f32,      // 40
    pub max_collection_roots: u32,       // 44
    pub frames_between_collections: u32, // 48
    pub pad4c: u32,                      // 4C
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefMemoryParams>() == 0x50);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryParams, desc) == 0x0);
const _: () =
    assert!(core::mem::offset_of!(GFxMovieDefMemoryParams, heap_limit_multiplier) == 0x40);

impl GFxMovieDefMemoryParams {
    #[inline(always)]
    pub fn new(memory_arena: usize) -> Self {
        let mut desc = GMemoryHeapHeapDesc::default();
        desc.arena = memory_arena;
        Self {
            desc,
            heap_limit_multiplier: 0.25,
            max_collection_roots: u32::MAX,
            frames_between_collections: u32::MAX,
            pad4c: 0,
        }
    }
}

impl Default for GFxMovieDefMemoryParams {
    #[inline(always)]
    fn default() -> Self {
        Self::new(0)
    }
}

/// C++ `RE::GFxMovieDef::MemoryContext`
#[repr(C)]
pub struct GFxMovieDefMemoryContext {
    pub base: GRefCountBase<GFxMovieDefMemoryContext, { GStatGroups::DEFAULT_MEM as u32 }>, // 00
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefMemoryContext>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryContext, base) == 0x0);

inherit!(GFxMovieDefMemoryContext : GRefCountBase<GFxMovieDefMemoryContext, { GStatGroups::DEFAULT_MEM as u32 }>, base);

impl GFxMovieDefMemoryContext {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
}

/// C++ `RE::GFxMovieDef::MemoryContextImpl::HeapLimit`
#[repr(C)]
pub struct GFxMovieDefMemoryContextImplHeapLimit {
    pub base: GMemoryHeapLimitHandler, // 00
    pub unk08: u64,                    // 08
    pub unk10: u64,                    // 10
    pub unk18: u64,                    // 18
    pub unk20: u64,                    // 20
    pub unk28: u64,                    // 28
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefMemoryContextImplHeapLimit>() == 0x30);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryContextImplHeapLimit, base) == 0x0);

inherit!(GFxMovieDefMemoryContextImplHeapLimit : GMemoryHeapLimitHandler, base);

impl GFxMovieDefMemoryContextImplHeapLimit {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_ON_EXCEED_LIMIT: usize = 0x01; pub fn on_exceed_limit(heap: *mut GMemoryHeap, over_limit: usize) -> bool }
    crate::virtual_method! { pub const VFUNC_ON_FREE_SEGMENT: usize = 0x02; pub fn on_free_segment(heap: *mut GMemoryHeap, freeing_size: usize) }
}

/// C++ `RE::GFxMovieDef::MemoryContextImpl`
#[repr(C)]
pub struct GFxMovieDefMemoryContextImpl {
    pub base: GFxMovieDefMemoryContext, // 00
    pub heap: *mut GMemoryHeap,         // 10
    // TODO: `GFxMovieDef.h` stores `refCountCollector` as `GPtr<GASRefCountCollector>`.
    // The vendored tree still only forward-declares that pointee, so keep the
    // raw pointer here until a real intrusive-refcounted translation exists.
    pub ref_count_collector: *mut GASRefCountCollector, // 18
    pub string_manager: GPtr<GASStringManager>,         // 20
    pub unk28: u64,                                     // 28
    pub heap_limit: GFxMovieDefMemoryContextImplHeapLimit, // 30
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefMemoryContextImpl>() == 0x60);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryContextImpl, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryContextImpl, heap) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefMemoryContextImpl, heap_limit) == 0x30);

inherit!(GFxMovieDefMemoryContextImpl : GFxMovieDefMemoryContext, base);

/// C++ `RE::GFxMovieDef::ImportVisitor`
#[repr(C)]
pub struct GFxMovieDefImportVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefImportVisitor>() == 0x8);

impl GFxMovieDefImportVisitor {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_VISIT: usize = 0x01; pub fn visit(parent_def: *mut GFxMovieDef, import_def: *mut GFxMovieDef, imported_movie_filename: *const c_char) }
}

/// C++ `RE::GFxMovieDef::ResourceVisitor`
#[repr(C)]
pub struct GFxMovieDefResourceVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefResourceVisitor>() == 0x8);

impl GFxMovieDefResourceVisitor {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_VISIT: usize = 0x01; pub fn visit(movie_def: *mut GFxMovieDef, resource: *mut GFxResource, id: crate::re::GFxResourceID, export_name: *const c_char) }
}

/// C++ `RE::GFxMovieDef`
#[repr(C)]
pub struct GFxMovieDef {
    pub base: GFxResource,      // 00
    pub state_bag: GFxStateBag, // 18
}

const _: () = assert!(core::mem::size_of::<GFxMovieDef>() == 0x20);
const _: () = assert!(core::mem::offset_of!(GFxMovieDef, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDef, state_bag) == 0x18);

inherit!(GFxMovieDef : GFxResource, base);
inherit!(GFxMovieDef => GFxStateBag, state_bag);

impl GFxMovieDef {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_KEY: usize = 0x01; pub fn get_key(&mut self) -> crate::re::GFxResourceKey }
    crate::virtual_method! { pub const VFUNC_GET_RESOURCE_TYPE_CODE: usize = 0x02; pub fn get_resource_type_code(&self) -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_RESOURCE_REPORT: usize = 0x03; pub fn get_resource_report(&mut self) -> *mut crate::re::GFxResourceReport }
    crate::virtual_method! { pub const VFUNC_GET_VERSION: usize = 0x04; pub fn get_version() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_LOADING_FRAME: usize = 0x05; pub fn get_loading_frame() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_WIDTH: usize = 0x06; pub fn get_width() -> f32 }
    crate::virtual_method! { pub const VFUNC_GET_HEIGHT: usize = 0x07; pub fn get_height() -> f32 }
    crate::virtual_method! { pub const VFUNC_GET_FRAME_COUNT: usize = 0x08; pub fn get_frame_count() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_FRAME_RATE: usize = 0x09; pub fn get_frame_rate() -> f32 }
    crate::virtual_method! { pub const VFUNC_GET_FRAME_RECT: usize = 0x0A; pub fn get_frame_rect() -> GRectF }
    crate::virtual_method! { pub const VFUNC_GET_SWF_FLAGS: usize = 0x0B; pub fn get_swf_flags() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_FILE_URL: usize = 0x0C; pub fn get_file_url() -> *const c_char }
    crate::virtual_method! { pub const VFUNC_WAIT_FOR_LOAD_FINISH: usize = 0x0D; pub fn wait_for_load_finish(cancel: bool) }
    crate::virtual_method! { pub const VFUNC_WAIT_FOR_FRAME: usize = 0x0E; pub fn wait_for_frame(frame: u32) }
    crate::virtual_method! { pub const VFUNC_GET_FILE_ATTRIBUTES: usize = 0x0F; pub fn get_file_attributes() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_METADATA: usize = 0x10; pub fn get_metadata(buff: *mut c_char, buff_size: u32) -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_LOAD_DATA_HEAP: usize = 0x11; pub fn get_load_data_heap() -> *mut GMemoryHeap }
    crate::virtual_method! { pub const VFUNC_GET_BIND_DATA_HEAP: usize = 0x12; pub fn get_bind_data_heap() -> *mut GMemoryHeap }
    crate::virtual_method! { pub const VFUNC_GET_IMAGE_HEAP: usize = 0x13; pub fn get_image_heap() -> *mut GMemoryHeap }
    crate::virtual_method! { pub const VFUNC_GET_MOVIE_DATA_RESOURCE: usize = 0x14; pub fn get_movie_data_resource() -> *mut GFxResource }
    crate::virtual_method! { pub const VFUNC_GET_EXPORTER_INFO: usize = 0x15; pub fn get_exporter_info() -> *const GFxExporterInfo }
    crate::virtual_method! { pub const VFUNC_CREATE_MEMORY_CONTEXT: usize = 0x16; pub fn create_memory_context(heap_name: *const c_char, mem_params: &GFxMovieDefMemoryParams, debug_heap: bool) -> *mut GFxMovieDefMemoryContext }
    crate::virtual_method! { pub const VFUNC_CREATE_INSTANCE_WITH_CONTEXT: usize = 0x17; pub fn create_instance_with_context(mem_context: *mut GFxMovieDefMemoryContext, init_first_frame: bool) -> *mut GFxMovieView }
    crate::virtual_method! { pub const VFUNC_CREATE_INSTANCE: usize = 0x18; pub fn create_instance(mem_params: &GFxMovieDefMemoryParams, init_first_frame: bool) -> *mut GFxMovieView }
    crate::virtual_method! { pub const VFUNC_VISIT_IMPORTED_MOVIES: usize = 0x19; pub fn visit_imported_movies(visitor: *mut GFxMovieDefImportVisitor) }
    crate::virtual_method! { pub const VFUNC_VISIT_RESOURCES: usize = 0x1A; pub fn visit_resources(visitor: *mut GFxMovieDefResourceVisitor, visit_mask: GFxMovieDefVisitResourceMask) }
    crate::virtual_method! { pub const VFUNC_GET_RESOURCE: usize = 0x1B; pub fn get_resource(export_name: *const c_char) -> *mut GFxResource }

    #[inline(always)]
    pub fn create_instance_with_arena(
        &mut self,
        init_first_frame: bool,
        memory_arena: usize,
    ) -> *mut GFxMovieView {
        let params = GFxMovieDefMemoryParams::new(memory_arena);
        self.create_instance(&params, init_first_frame)
    }
}

impl GPtrTarget for GFxMovieDef {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.release();
        }
    }
}

impl AsRef<GFxMovieDef> for GFxMovieDef {
    #[inline(always)]
    fn as_ref(&self) -> &GFxMovieDef {
        self
    }
}

impl AsMut<GFxMovieDef> for GFxMovieDef {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxMovieDef {
        self
    }
}

pub trait GFxMovieDefExt: AsRef<GFxMovieDef> + AsMut<GFxMovieDef> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn get_key(&mut self) -> crate::re::GFxResourceKey {
        self.as_mut().get_key()
    }

    #[inline(always)]
    fn get_resource_type_code(&self) -> u32 {
        self.as_ref().get_resource_type_code()
    }

    #[inline(always)]
    fn get_resource_report(&mut self) -> *mut crate::re::GFxResourceReport {
        self.as_mut().get_resource_report()
    }

    #[inline(always)]
    fn get_version(&self) -> u32 {
        self.as_ref().get_version()
    }

    #[inline(always)]
    fn get_loading_frame(&self) -> u32 {
        self.as_ref().get_loading_frame()
    }

    #[inline(always)]
    fn get_width(&self) -> f32 {
        self.as_ref().get_width()
    }

    #[inline(always)]
    fn get_height(&self) -> f32 {
        self.as_ref().get_height()
    }

    #[inline(always)]
    fn get_frame_count(&self) -> u32 {
        self.as_ref().get_frame_count()
    }

    #[inline(always)]
    fn get_frame_rate(&self) -> f32 {
        self.as_ref().get_frame_rate()
    }

    #[inline(always)]
    fn get_frame_rect(&self) -> GRectF {
        self.as_ref().get_frame_rect()
    }

    #[inline(always)]
    fn get_swf_flags(&self) -> u32 {
        self.as_ref().get_swf_flags()
    }

    #[inline(always)]
    fn get_file_url(&self) -> *const c_char {
        self.as_ref().get_file_url()
    }

    #[inline(always)]
    fn wait_for_load_finish(&self, cancel: bool) {
        self.as_ref().wait_for_load_finish(cancel)
    }

    #[inline(always)]
    fn wait_for_frame(&self, frame: u32) {
        self.as_ref().wait_for_frame(frame)
    }

    #[inline(always)]
    fn get_file_attributes(&self) -> u32 {
        self.as_ref().get_file_attributes()
    }

    #[inline(always)]
    fn get_metadata(&self, buff: *mut c_char, buff_size: u32) -> u32 {
        self.as_ref().get_metadata(buff, buff_size)
    }

    #[inline(always)]
    fn get_load_data_heap(&self) -> *mut GMemoryHeap {
        self.as_ref().get_load_data_heap()
    }

    #[inline(always)]
    fn get_bind_data_heap(&self) -> *mut GMemoryHeap {
        self.as_ref().get_bind_data_heap()
    }

    #[inline(always)]
    fn get_image_heap(&self) -> *mut GMemoryHeap {
        self.as_ref().get_image_heap()
    }

    #[inline(always)]
    fn get_movie_data_resource(&self) -> *mut GFxResource {
        self.as_ref().get_movie_data_resource()
    }

    #[inline(always)]
    fn get_exporter_info(&self) -> *const GFxExporterInfo {
        self.as_ref().get_exporter_info()
    }

    #[inline(always)]
    fn create_memory_context(
        &mut self,
        heap_name: *const c_char,
        mem_params: &GFxMovieDefMemoryParams,
        debug_heap: bool,
    ) -> *mut GFxMovieDefMemoryContext {
        self.as_mut()
            .create_memory_context(heap_name, mem_params, debug_heap)
    }

    #[inline(always)]
    fn create_instance_with_context(
        &mut self,
        mem_context: *mut GFxMovieDefMemoryContext,
        init_first_frame: bool,
    ) -> *mut GFxMovieView {
        self.as_mut()
            .create_instance_with_context(mem_context, init_first_frame)
    }

    #[inline(always)]
    fn create_instance(
        &mut self,
        mem_params: &GFxMovieDefMemoryParams,
        init_first_frame: bool,
    ) -> *mut GFxMovieView {
        self.as_mut().create_instance(mem_params, init_first_frame)
    }

    #[inline(always)]
    fn visit_imported_movies(&mut self, visitor: *mut GFxMovieDefImportVisitor) {
        self.as_mut().visit_imported_movies(visitor)
    }

    #[inline(always)]
    fn visit_resources(
        &mut self,
        visitor: *mut GFxMovieDefResourceVisitor,
        visit_mask: GFxMovieDefVisitResourceMask,
    ) {
        self.as_mut().visit_resources(visitor, visit_mask)
    }

    #[inline(always)]
    fn get_resource(&self, export_name: *const c_char) -> *mut GFxResource {
        self.as_ref().get_resource(export_name)
    }

    #[inline(always)]
    fn create_instance_with_arena(
        &mut self,
        init_first_frame: bool,
        memory_arena: usize,
    ) -> *mut GFxMovieView {
        self.as_mut()
            .create_instance_with_arena(init_first_frame, memory_arena)
    }
}

impl<T> GFxMovieDefExt for T where T: AsRef<GFxMovieDef> + AsMut<GFxMovieDef> {}
