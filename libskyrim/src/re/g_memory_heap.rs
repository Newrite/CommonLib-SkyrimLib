#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};

use core_util::{EnumSet, inherit};

use crate::re::{
    GHeapAllocEngine, GHeapDebugStorage, GHeapID, GHeapMemVisitor, GHeapSegVisitor, GList,
    GListNode, GLock, GStatBag, GSysAllocPaged,
};
use crate::rex::W32::GetCurrentThreadId;

/// C++ `RE::GMemoryHeap::MemReportType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GMemoryHeapMemReportType {
    kBrief = 0,
    kSummary = 1,
    kMedium = 2,
    kFull = 3,
    kSimple = 4,
    kSimpleBrief = 5,
    kFileSummary = 6,
    kHeapsOnly = 7,
}

/// C++ `RE::GMemoryHeap::HeapFlags`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GMemoryHeapHeapFlags {
    kNone = 0,
    kThreadUnsafe = 1 << 0,
    kFastTinyBlocks = 1 << 1,
    kFixedGranularity = 1 << 2,
    kRoot = 1 << 3,
    kNoDebugInfo = 1 << 4,
    kUserDebug = 1 << 12,
}

core_util::impl_enumset_type!(GMemoryHeapHeapFlags => u32);

/// C++ `RE::GMemoryHeap::RootHeapParameters`
pub struct GMemoryHeapRootHeapParameters;

impl GMemoryHeapRootHeapParameters {
    pub const MIN_ALIGN: usize = 16;
    pub const GRANULARITY: usize = 16 * 1024;
    pub const RESERVE: usize = 16 * 1024;
    pub const THRESHOLD: usize = 256 * 1024;
    pub const LIMIT: usize = 0;
}

/// C++ `RE::GMemoryHeap::HeapDesc`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GMemoryHeapHeapDesc {
    pub flags: EnumSet<GMemoryHeapHeapFlags, u32>, // 00
    pub pad04: u32,                                // 04
    pub min_align: usize,                          // 08
    pub granularity: usize,                        // 10
    pub reserve: usize,                            // 18
    pub threshold: usize,                          // 20
    pub limit: usize,                              // 28
    pub heap_id: GHeapID,                          // 30
    pub arena: usize,                              // 38
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapHeapDesc>() == 0x40);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, flags) == 0x0);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, min_align) == 0x8);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, granularity) == 0x10);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, reserve) == 0x18);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, threshold) == 0x20);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, limit) == 0x28);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, heap_id) == 0x30);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapDesc, arena) == 0x38);

impl GMemoryHeapHeapDesc {
    #[inline(always)]
    pub fn new(
        flags: EnumSet<GMemoryHeapHeapFlags, u32>,
        min_align: usize,
        granularity: usize,
        reserve: usize,
        threshold: usize,
        limit: usize,
        heap_id: GHeapID,
        arena: usize,
    ) -> Self {
        Self {
            flags,
            pad04: 0,
            min_align,
            granularity,
            reserve,
            threshold,
            limit,
            heap_id,
            arena,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.flags = EnumSet::from_underlying(GMemoryHeapHeapFlags::kNone as u32);
        self.min_align = 16;
        self.granularity = 0;
        self.reserve = 0;
        self.threshold = usize::MAX;
        self.limit = 0;
        self.heap_id = GHeapID::kReserved;
        self.arena = 0;
    }
}

impl Default for GMemoryHeapHeapDesc {
    #[inline(always)]
    fn default() -> Self {
        Self::new(
            EnumSet::from_underlying(GMemoryHeapHeapFlags::kNone as u32),
            16,
            8 * 1024,
            8 * 1024,
            usize::MAX,
            0,
            GHeapID::kReserved,
            0,
        )
    }
}

pub trait GMemoryHeapHeapDescExt: AsRef<GMemoryHeapHeapDesc> + AsMut<GMemoryHeapHeapDesc> {
    #[inline(always)]
    fn clear(&mut self) {
        self.as_mut().clear()
    }
}

impl<T> GMemoryHeapHeapDescExt for T where T: AsRef<GMemoryHeapHeapDesc> + AsMut<GMemoryHeapHeapDesc>
{}

/// C++ `RE::GMemoryHeap::RootHeapDesc`
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct GMemoryHeapRootHeapDesc {
    pub base: GMemoryHeapHeapDesc, // 00
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapRootHeapDesc>() == 0x40);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapRootHeapDesc, base) == 0x0);

inherit!(GMemoryHeapRootHeapDesc : GMemoryHeapHeapDesc, base);

impl Default for GMemoryHeapRootHeapDesc {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl GMemoryHeapRootHeapDesc {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: GMemoryHeapHeapDesc::new(
                EnumSet::from_underlying(GMemoryHeapHeapFlags::kNone as u32),
                GMemoryHeapRootHeapParameters::MIN_ALIGN,
                GMemoryHeapRootHeapParameters::GRANULARITY,
                GMemoryHeapRootHeapParameters::RESERVE,
                GMemoryHeapRootHeapParameters::THRESHOLD,
                GMemoryHeapRootHeapParameters::LIMIT,
                GHeapID::kGlobal,
                0,
            ),
        }
    }
}

/// C++ `RE::GMemoryHeap::HeapInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GMemoryHeapHeapInfo {
    pub desc: GMemoryHeapHeapDesc, // 00
    pub parent: *mut GMemoryHeap,  // 40
    pub name: *mut c_char,         // 48
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapHeapInfo>() == 0x50);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapInfo, desc) == 0x0);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapInfo, parent) == 0x40);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapHeapInfo, name) == 0x48);

/// C++ `RE::GMemoryHeap::HeapVisitor`
#[repr(C)]
pub struct GMemoryHeapHeapVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapHeapVisitor>() == 0x8);

impl GMemoryHeapHeapVisitor {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x1;
        pub fn visit(parent_heap: *mut GMemoryHeap, child_heap: *mut GMemoryHeap)
    }
}

/// C++ `RE::GMemoryHeap::LimitHandler`
#[repr(C)]
pub struct GMemoryHeapLimitHandler {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapLimitHandler>() == 0x8);

impl GMemoryHeapLimitHandler {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_ON_EXCEED_LIMIT: usize = 0x1;
        pub fn on_exceed_limit(heap: *mut GMemoryHeap, over_limit: usize) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_ON_FREE_SEGMENT: usize = 0x2;
        pub fn on_free_segment(heap: *mut GMemoryHeap, freeing_size: usize)
    }
}

pub trait GMemoryHeapLimitHandlerExt:
    AsRef<GMemoryHeapLimitHandler> + AsMut<GMemoryHeapLimitHandler>
{
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn on_exceed_limit(&mut self, heap: *mut GMemoryHeap, over_limit: usize) -> bool {
        self.as_mut().on_exceed_limit(heap, over_limit)
    }

    #[inline(always)]
    fn on_free_segment(&mut self, heap: *mut GMemoryHeap, freeing_size: usize) {
        self.as_mut().on_free_segment(heap, freeing_size)
    }
}

impl<T> GMemoryHeapLimitHandlerExt for T where
    T: AsRef<GMemoryHeapLimitHandler> + AsMut<GMemoryHeapLimitHandler>
{
}

/// C++ `RE::GMemoryHeap::HeapTracer`
#[repr(C)]
pub struct GMemoryHeapHeapTracer {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapHeapTracer>() == 0x8);

impl GMemoryHeapHeapTracer {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_ON_CREATE_HEAP: usize = 0x1;
        pub fn on_create_heap(heap: *const GMemoryHeap)
    }

    crate::virtual_method! {
        pub const VFUNC_ON_DESTROY_HEAP: usize = 0x2;
        pub fn on_destroy_heap(heap: *const GMemoryHeap)
    }

    crate::virtual_method! {
        pub const VFUNC_ON_ALLOC: usize = 0x3;
        pub fn on_alloc(
            heap: *const GMemoryHeap,
            size: usize,
            align: usize,
            sid: u32,
            ptr: *const c_void
        )
    }

    crate::virtual_method! {
        pub const VFUNC_ON_REALLOC: usize = 0x4;
        pub fn on_realloc(
            heap: *const GMemoryHeap,
            old_ptr: *const c_void,
            new_size: usize,
            new_ptr: *const c_void
        )
    }

    crate::virtual_method! {
        pub const VFUNC_ON_FREE: usize = 0x5;
        pub fn on_free(heap: *const GMemoryHeap, ptr: *const c_void)
    }
}

/// C++ `RE::GMemoryHeap::RootStats`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GMemoryHeapRootStats {
    pub sys_mem_footprint: usize,      // 00
    pub sys_mem_used_space: usize,     // 08
    pub page_map_footprint: usize,     // 10
    pub page_map_used_space: usize,    // 18
    pub bookkeeping_footprint: usize,  // 20
    pub bookkeeping_used_space: usize, // 28
    pub debug_info_footprint: usize,   // 30
    pub debug_info_used_space: usize,  // 38
    pub user_debug_footprint: usize,   // 40
    pub user_debug_used_space: usize,  // 48
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapRootStats>() == 0x50);

/// C++ `RE::GMemoryHeap`
#[repr(C)]
pub struct GMemoryHeap {
    pub base: GListNode<GMemoryHeap>,    // 00
    pub vtable: *const usize,            // 10
    pub self_size: usize,                // 18
    pub ref_count: u32,                  // 20
    pub pad24: u32,                      // 24
    pub owner_thread_id: usize,          // 28
    pub auto_release: *mut c_void,       // 30
    pub info: GMemoryHeapHeapInfo,       // 38
    pub child_heaps: GList<GMemoryHeap>, // 88
    pub heap_lock: GLock,                // 98
    pub use_locks: bool,                 // C0
    pub track_debug_info: bool,          // C1
    pub padc2: u16,                      // C2
    pub padc4: u32,                      // C4
}

const _: () = assert!(core::mem::size_of::<GMemoryHeap>() == 0xC8);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, vtable) == 0x10);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, self_size) == 0x18);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, ref_count) == 0x20);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, owner_thread_id) == 0x28);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, auto_release) == 0x30);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, info) == 0x38);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, child_heaps) == 0x88);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, heap_lock) == 0x98);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, use_locks) == 0xC0);
const _: () = assert!(core::mem::offset_of!(GMemoryHeap, track_debug_info) == 0xC1);

inherit!(GMemoryHeap : GListNode<GMemoryHeap>, base);

impl GMemoryHeap {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_CREATE_ARENA: usize = 0x1;
        pub fn create_arena(arena: usize, sys_alloc: *mut GSysAllocPaged)
    }

    crate::virtual_method! {
        pub const VFUNC_DESTROY_ARENA: usize = 0x2;
        pub fn destroy_arena(arena: usize)
    }

    crate::virtual_method! {
        pub const VFUNC_ARENA_IS_EMPTY: usize = 0x3;
        pub fn arena_is_empty(arena: usize) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CREATE_HEAP: usize = 0x4;
        pub fn create_heap(name: *const c_char, desc: &GMemoryHeapHeapDesc) -> *mut GMemoryHeap
    }

    crate::virtual_method! {
        pub const VFUNC_SET_LIMIT_HANDLER: usize = 0x5;
        pub fn set_limit_handler(handler: *mut GMemoryHeapLimitHandler)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_LIMIT: usize = 0x6;
        pub fn set_limit(new_limit: usize)
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_REF: usize = 0x7;
        pub fn add_ref()
    }

    crate::virtual_method! {
        pub const VFUNC_RELEASE: usize = 0x8;
        pub fn release()
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOC: usize = 0x9;
        pub fn alloc(size: usize) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOC_ALIGNED: usize = 0xA;
        pub fn alloc_aligned(size: usize, align: usize) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_REALLOC: usize = 0xB;
        pub fn realloc(old_ptr: *mut c_void, new_size: usize) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_FREE: usize = 0xC;
        pub fn free(ptr: *mut c_void)
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOC_AUTO_HEAP: usize = 0xD;
        pub fn alloc_auto_heap(this_ptr: *const c_void, size: usize) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOC_AUTO_HEAP_ALIGNED: usize = 0xE;
        pub fn alloc_auto_heap_aligned(this_ptr: *const c_void, size: usize, align: usize)
            -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ALLOC_HEAP: usize = 0xF;
        pub fn get_alloc_heap(this_ptr: *const c_void) -> *mut GMemoryHeap
    }

    crate::virtual_method! {
        pub const VFUNC_GET_USABLE_SIZE: usize = 0x10;
        pub fn get_usable_size(ptr: *const c_void) -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOC_SYS_DIRECT: usize = 0x11;
        pub fn alloc_sys_direct(size: usize) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_FREE_SYS_DIRECT: usize = 0x12;
        pub fn free_sys_direct(ptr: *mut c_void, size: usize)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STATS: usize = 0x13;
        pub fn get_stats(bag: *mut GStatBag) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FOOTPRINT: usize = 0x14;
        pub fn get_footprint() -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TOTAL_FOOTPRINT: usize = 0x15;
        pub fn get_total_footprint() -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_USED_SPACE: usize = 0x16;
        pub fn get_used_space() -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TOTAL_USED_SPACE: usize = 0x17;
        pub fn get_total_used_space() -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ROOT_STATS: usize = 0x18;
        pub fn get_root_stats(stats: *mut GMemoryHeapRootStats)
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT_MEM: usize = 0x19;
        pub fn visit_mem(visitor: *mut GHeapMemVisitor, flags: u32)
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT_ROOT_SEGMENTS: usize = 0x1A;
        pub fn visit_root_segments(visitor: *mut GHeapSegVisitor)
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT_HEAP_SEGMENTS: usize = 0x1B;
        pub fn visit_heap_segments(visitor: *mut GHeapSegVisitor)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TRACER: usize = 0x1C;
        pub fn set_tracer(tracer: *mut GMemoryHeapHeapTracer)
    }

    crate::virtual_method! {
        pub const VFUNC_DESTROY_ITSELF: usize = 0x1D;
        pub fn destroy_itself()
    }

    crate::virtual_method! {
        pub const VFUNC_ULTIMATE_CHECK_INTERNAL: usize = 0x1E;
        pub fn ultimate_check_internal()
    }

    crate::virtual_method! {
        pub const VFUNC_RELEASE_CACHED_MEM: usize = 0x1F;
        pub fn release_cached_mem()
    }

    crate::virtual_method! {
        pub const VFUNC_DUMP_MEMORY_LEAKS_INTERNAL: usize = 0x20;
        pub fn dump_memory_leaks_internal() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CHECK_INTEGRITY_INTERNAL: usize = 0x21;
        pub fn check_integrity_internal()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_USER_DEBUG_STATS: usize = 0x22;
        pub fn get_user_debug_stats(stats: *mut GMemoryHeapRootStats)
    }

    #[inline(always)]
    pub fn create_heap_with_params(
        &mut self,
        name: *const c_char,
        flags: EnumSet<GMemoryHeapHeapFlags, u32>,
        min_align: usize,
        granularity: usize,
        reserve: usize,
        threshold: usize,
        limit: usize,
        heap_id: GHeapID,
        arena: usize,
    ) -> *mut GMemoryHeap {
        let desc = GMemoryHeapHeapDesc::new(
            flags,
            min_align,
            granularity,
            reserve,
            threshold,
            limit,
            heap_id,
            arena,
        );
        self.create_heap(name, &desc)
    }

    #[inline(always)]
    pub fn get_heap_info(&self, info: &mut GMemoryHeapHeapInfo) {
        *info = self.info;
    }

    #[inline(always)]
    pub fn get_name(&self) -> *const c_char {
        self.info.name.cast_const()
    }

    #[inline(always)]
    pub fn get_id(&self) -> GHeapID {
        self.info.desc.heap_id
    }

    #[inline(always)]
    pub fn get_parent_heap(&self) -> *mut GMemoryHeap {
        self.info.parent
    }

    #[inline(always)]
    pub fn get_flags(&self) -> EnumSet<GMemoryHeapHeapFlags, u32> {
        // TODO: CommonLib's `GetFlags()` returns the `HeapFlags` enum-class
        // bitmask directly. Keep exposing `EnumSet` here because Rust enums
        // cannot honestly represent arbitrary combined flag bits as a single
        // safe enum value.
        self.info.desc.flags
    }

    #[inline(always)]
    pub fn get_granularity(&self) -> usize {
        self.info.desc.granularity
    }

    #[inline(always)]
    pub fn get_limit(&self) -> usize {
        self.info.desc.limit
    }

    #[inline(always)]
    pub fn is_thread_safe(&self) -> bool {
        self.info
            .desc
            .flags
            .none(GMemoryHeapHeapFlags::kThreadUnsafe)
    }

    #[inline(always)]
    pub fn release_on_free(&mut self, ptr: *mut c_void) {
        self.auto_release = ptr;
    }

    #[inline(always)]
    pub fn assign_to_current_thread(&mut self) {
        let thread_id = unsafe { GetCurrentThreadId() as usize };
        debug_assert!(self.owner_thread_id == 0 || self.owner_thread_id == thread_id);
        self.owner_thread_id = thread_id;
    }

    #[inline(always)]
    pub fn dump_memory_leaks(&mut self) -> bool {
        self.dump_memory_leaks_internal()
    }

    #[inline(always)]
    pub fn ultimate_check(&mut self) {
        self.ultimate_check_internal()
    }

    #[inline(always)]
    pub fn check_integrity(&mut self) {
        self.check_integrity_internal()
    }
}

pub trait GMemoryHeapExt: AsRef<GMemoryHeap> + AsMut<GMemoryHeap> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn create_arena(&mut self, arena: usize, sys_alloc: *mut GSysAllocPaged) {
        self.as_mut().create_arena(arena, sys_alloc)
    }

    #[inline(always)]
    fn destroy_arena(&mut self, arena: usize) {
        self.as_mut().destroy_arena(arena)
    }

    #[inline(always)]
    fn arena_is_empty(&mut self, arena: usize) -> bool {
        self.as_mut().arena_is_empty(arena)
    }

    #[inline(always)]
    fn create_heap(&mut self, name: *const c_char, desc: &GMemoryHeapHeapDesc) -> *mut GMemoryHeap {
        self.as_mut().create_heap(name, desc)
    }

    #[inline(always)]
    fn set_limit_handler(&mut self, handler: *mut GMemoryHeapLimitHandler) {
        self.as_mut().set_limit_handler(handler)
    }

    #[inline(always)]
    fn set_limit(&mut self, new_limit: usize) {
        self.as_mut().set_limit(new_limit)
    }

    #[inline(always)]
    fn add_ref(&mut self) {
        self.as_mut().add_ref()
    }

    #[inline(always)]
    fn release(&mut self) {
        self.as_mut().release()
    }

    #[inline(always)]
    fn alloc(&mut self, size: usize) -> *mut c_void {
        self.as_mut().alloc(size)
    }

    #[inline(always)]
    fn alloc_aligned(&mut self, size: usize, align: usize) -> *mut c_void {
        self.as_mut().alloc_aligned(size, align)
    }

    #[inline(always)]
    fn realloc(&mut self, old_ptr: *mut c_void, new_size: usize) -> *mut c_void {
        self.as_mut().realloc(old_ptr, new_size)
    }

    #[inline(always)]
    fn free(&mut self, ptr: *mut c_void) {
        self.as_mut().free(ptr)
    }

    #[inline(always)]
    fn alloc_auto_heap(&mut self, this_ptr: *const c_void, size: usize) -> *mut c_void {
        self.as_mut().alloc_auto_heap(this_ptr, size)
    }

    #[inline(always)]
    fn alloc_auto_heap_aligned(
        &mut self,
        this_ptr: *const c_void,
        size: usize,
        align: usize,
    ) -> *mut c_void {
        self.as_mut().alloc_auto_heap_aligned(this_ptr, size, align)
    }

    #[inline(always)]
    fn get_alloc_heap(&mut self, this_ptr: *const c_void) -> *mut GMemoryHeap {
        self.as_mut().get_alloc_heap(this_ptr)
    }

    #[inline(always)]
    fn get_usable_size(&mut self, ptr: *const c_void) -> usize {
        self.as_mut().get_usable_size(ptr)
    }

    #[inline(always)]
    fn alloc_sys_direct(&mut self, size: usize) -> *mut c_void {
        self.as_mut().alloc_sys_direct(size)
    }

    #[inline(always)]
    fn free_sys_direct(&mut self, ptr: *mut c_void, size: usize) {
        self.as_mut().free_sys_direct(ptr, size)
    }

    #[inline(always)]
    fn get_stats(&mut self, bag: *mut GStatBag) -> bool {
        self.as_mut().get_stats(bag)
    }

    #[inline(always)]
    fn get_footprint(&self) -> usize {
        self.as_ref().get_footprint()
    }

    #[inline(always)]
    fn get_total_footprint(&self) -> usize {
        self.as_ref().get_total_footprint()
    }

    #[inline(always)]
    fn get_used_space(&self) -> usize {
        self.as_ref().get_used_space()
    }

    #[inline(always)]
    fn get_total_used_space(&self) -> usize {
        self.as_ref().get_total_used_space()
    }

    #[inline(always)]
    fn get_root_stats(&mut self, stats: *mut GMemoryHeapRootStats) {
        self.as_mut().get_root_stats(stats)
    }

    #[inline(always)]
    fn visit_mem(&mut self, visitor: *mut GHeapMemVisitor, flags: u32) {
        self.as_mut().visit_mem(visitor, flags)
    }

    #[inline(always)]
    fn visit_root_segments(&mut self, visitor: *mut GHeapSegVisitor) {
        self.as_mut().visit_root_segments(visitor)
    }

    #[inline(always)]
    fn visit_heap_segments(&self, visitor: *mut GHeapSegVisitor) {
        self.as_ref().visit_heap_segments(visitor)
    }

    #[inline(always)]
    fn set_tracer(&mut self, tracer: *mut GMemoryHeapHeapTracer) {
        self.as_mut().set_tracer(tracer)
    }

    #[inline(always)]
    fn destroy_itself(&mut self) {
        self.as_mut().destroy_itself()
    }

    #[inline(always)]
    fn ultimate_check_internal(&mut self) {
        self.as_mut().ultimate_check_internal()
    }

    #[inline(always)]
    fn release_cached_mem(&mut self) {
        self.as_mut().release_cached_mem()
    }

    #[inline(always)]
    fn dump_memory_leaks_internal(&mut self) -> bool {
        self.as_mut().dump_memory_leaks_internal()
    }

    #[inline(always)]
    fn check_integrity_internal(&self) {
        self.as_ref().check_integrity_internal()
    }

    #[inline(always)]
    fn get_user_debug_stats(&self, stats: *mut GMemoryHeapRootStats) {
        self.as_ref().get_user_debug_stats(stats)
    }

    #[inline(always)]
    fn create_heap_with_params(
        &mut self,
        name: *const c_char,
        flags: EnumSet<GMemoryHeapHeapFlags, u32>,
        min_align: usize,
        granularity: usize,
        reserve: usize,
        threshold: usize,
        limit: usize,
        heap_id: GHeapID,
        arena: usize,
    ) -> *mut GMemoryHeap {
        self.as_mut().create_heap_with_params(
            name,
            flags,
            min_align,
            granularity,
            reserve,
            threshold,
            limit,
            heap_id,
            arena,
        )
    }

    #[inline(always)]
    fn get_heap_info(&self, info: &mut GMemoryHeapHeapInfo) {
        self.as_ref().get_heap_info(info)
    }

    #[inline(always)]
    fn get_name(&self) -> *const c_char {
        self.as_ref().get_name()
    }

    #[inline(always)]
    fn get_id(&self) -> GHeapID {
        self.as_ref().get_id()
    }

    #[inline(always)]
    fn get_parent_heap(&self) -> *mut GMemoryHeap {
        self.as_ref().get_parent_heap()
    }

    #[inline(always)]
    fn get_flags(&self) -> EnumSet<GMemoryHeapHeapFlags, u32> {
        self.as_ref().get_flags()
    }

    #[inline(always)]
    fn get_granularity(&self) -> usize {
        self.as_ref().get_granularity()
    }

    #[inline(always)]
    fn get_limit(&self) -> usize {
        self.as_ref().get_limit()
    }

    #[inline(always)]
    fn is_thread_safe(&self) -> bool {
        self.as_ref().is_thread_safe()
    }

    #[inline(always)]
    fn release_on_free(&mut self, ptr: *mut c_void) {
        self.as_mut().release_on_free(ptr)
    }

    #[inline(always)]
    fn assign_to_current_thread(&mut self) {
        self.as_mut().assign_to_current_thread()
    }

    #[inline(always)]
    fn dump_memory_leaks(&mut self) -> bool {
        self.as_mut().dump_memory_leaks()
    }

    #[inline(always)]
    fn ultimate_check(&mut self) {
        self.as_mut().ultimate_check()
    }

    #[inline(always)]
    fn check_integrity(&mut self) {
        self.as_mut().check_integrity()
    }
}

impl<T> GMemoryHeapExt for T where T: AsRef<GMemoryHeap> + AsMut<GMemoryHeap> {}

/// C++ `RE::GMemoryHeapPT`
#[repr(C)]
pub struct GMemoryHeapPT {
    pub base: GMemoryHeap,                     // 00
    pub engine: *mut GHeapAllocEngine,         // C8
    pub debug_storage: *mut GHeapDebugStorage, // D0
}

const _: () = assert!(core::mem::size_of::<GMemoryHeapPT>() == 0xD8);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapPT, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapPT, engine) == 0xC8);
const _: () = assert!(core::mem::offset_of!(GMemoryHeapPT, debug_storage) == 0xD0);

inherit!(GMemoryHeapPT : GMemoryHeap, base);
