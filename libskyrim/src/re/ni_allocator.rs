use core::ffi::{c_char, c_void};

use crate::offsets::offsets_rtti::RTTI_NiAllocator;
use crate::offsets::offsets_vtable::VTABLE_NiAllocator;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiMemEventType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiMemEventType {
    Unknown = 0,
    OperNew = 1,
    OperNewArray = 2,
    OperDelete = 3,
    OperDeleteArray = 4,
    Malloc = 5,
    Realloc = 6,
    AlignedMalloc = 7,
    AlignedRealloc = 8,
    Free = 9,
    AlignedFree = 10,
    ExternalAlloc = 11,
    ExternalFree = 12,
}

const _: () = assert!(core::mem::size_of::<NiMemEventType>() == 0x4);

/// C++ `RE::NiAllocator`
#[repr(C)]
pub struct NiAllocator {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<NiAllocator>() == 0x8);
const _: () = assert!(core::mem::offset_of!(NiAllocator, vtable) == 0x00);

impl RttiType for NiAllocator {
    const RTTI: VariantID = RTTI_NiAllocator;
}

impl AsRef<NiAllocator> for NiAllocator {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiAllocator> for NiAllocator {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl NiAllocator {
    pub const RTTI: VariantID = RTTI_NiAllocator;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiAllocator;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_ALLOCATE: usize = 0x01;
        pub fn allocate(
            size_in_bytes: &mut usize,
            alignment: &mut usize,
            event_type: NiMemEventType,
            provide_accurate_size_on_deallocate: bool,
            file: *const c_char,
            line: i32,
            function: *const c_char
        ) -> *mut c_void
    }

    virtual_method! {
        pub const VFUNC_DEALLOCATE: usize = 0x02;
        pub fn deallocate(memory: *mut c_void, event_type: NiMemEventType, size_in_bytes: usize)
    }

    virtual_method! {
        pub const VFUNC_REALLOCATE: usize = 0x03;
        pub fn reallocate(
            memory: *mut c_void,
            size_in_bytes: &mut usize,
            alignment: &mut usize,
            event_type: NiMemEventType,
            provide_accurate_size_on_deallocate: bool,
            size_current: usize,
            file: *const c_char,
            line: i32,
            function: *const c_char
        ) -> *mut c_void
    }

    virtual_method! {
        pub const VFUNC_TRACK_ALLOCATE: usize = 0x04;
        pub fn track_allocate(
            memory: *const c_void,
            size_in_bytes: usize,
            event_type: NiMemEventType,
            file: *const c_char,
            line: i32,
            function: *const c_char
        ) -> bool
    }

    virtual_method! {
        pub const VFUNC_TRACK_DEALLOCATE: usize = 0x05;
        pub fn track_deallocate(memory: *const c_void, event_type: NiMemEventType) -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_06: usize = 0x06;
        pub fn unk_06()
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE: usize = 0x07;
        pub fn initialize()
    }

    virtual_method! {
        pub const VFUNC_SHUTDOWN: usize = 0x08;
        pub fn shutdown()
    }

    virtual_method! {
        pub const VFUNC_VERIFY_ADDRESS: usize = 0x09;
        pub fn verify_address(memory: *const c_void) -> bool
    }
}

pub trait NiAllocatorExt {
    fn dtor(&mut self);
    fn allocate(
        &mut self,
        size_in_bytes: &mut usize,
        alignment: &mut usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void;
    fn deallocate(&mut self, memory: *mut c_void, event_type: NiMemEventType, size_in_bytes: usize);
    fn reallocate(
        &mut self,
        memory: *mut c_void,
        size_in_bytes: &mut usize,
        alignment: &mut usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        size_current: usize,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void;
    fn track_allocate(
        &self,
        memory: *const c_void,
        size_in_bytes: usize,
        event_type: NiMemEventType,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> bool;
    fn track_deallocate(&self, memory: *const c_void, event_type: NiMemEventType) -> bool;
    fn unk_06(&mut self);
    fn initialize(&mut self);
    fn shutdown(&mut self);
    fn verify_address(&self, memory: *const c_void) -> bool;
}

impl<T: AsRef<NiAllocator> + AsMut<NiAllocator>> NiAllocatorExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        NiAllocator::dtor(self.as_mut())
    }

    #[inline(always)]
    fn allocate(
        &mut self,
        size_in_bytes: &mut usize,
        alignment: &mut usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void {
        NiAllocator::allocate(
            self.as_mut(),
            size_in_bytes,
            alignment,
            event_type,
            provide_accurate_size_on_deallocate,
            file,
            line,
            function,
        )
    }

    #[inline(always)]
    fn deallocate(
        &mut self,
        memory: *mut c_void,
        event_type: NiMemEventType,
        size_in_bytes: usize,
    ) {
        NiAllocator::deallocate(self.as_mut(), memory, event_type, size_in_bytes)
    }

    #[inline(always)]
    fn reallocate(
        &mut self,
        memory: *mut c_void,
        size_in_bytes: &mut usize,
        alignment: &mut usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        size_current: usize,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> *mut c_void {
        NiAllocator::reallocate(
            self.as_mut(),
            memory,
            size_in_bytes,
            alignment,
            event_type,
            provide_accurate_size_on_deallocate,
            size_current,
            file,
            line,
            function,
        )
    }

    #[inline(always)]
    fn track_allocate(
        &self,
        memory: *const c_void,
        size_in_bytes: usize,
        event_type: NiMemEventType,
        file: *const c_char,
        line: i32,
        function: *const c_char,
    ) -> bool {
        self.as_ref()
            .track_allocate(memory, size_in_bytes, event_type, file, line, function)
    }

    #[inline(always)]
    fn track_deallocate(&self, memory: *const c_void, event_type: NiMemEventType) -> bool {
        self.as_ref().track_deallocate(memory, event_type)
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        NiAllocator::unk_06(self.as_mut())
    }

    #[inline(always)]
    fn initialize(&mut self) {
        NiAllocator::initialize(self.as_mut())
    }

    #[inline(always)]
    fn shutdown(&mut self) {
        NiAllocator::shutdown(self.as_mut())
    }

    #[inline(always)]
    fn verify_address(&self, memory: *const c_void) -> bool {
        self.as_ref().verify_address(memory)
    }
}
