use core::ffi::{c_char, c_void};

use crate::re::ni_allocator::{NiAllocator, NiMemEventType};
use crate::relocation::RelocationID;

/// C++ `RE::NiMemManager`
#[repr(C)]
pub struct NiMemManager {
    pub allocator: *mut NiAllocator, // 00
}

const _: () = assert!(core::mem::size_of::<NiMemManager>() == 0x8);
const _: () = assert!(core::mem::offset_of!(NiMemManager, allocator) == 0x0);

impl NiMemManager {
    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut NiMemManager => RelocationID::new(523759, 410319)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut NiMemManager {
        *Self::singleton_ptr()
    }

    #[inline]
    pub fn allocate(
        &self,
        size_in_bytes: usize,
        alignment: usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        source_file: *const c_char,
        source_line: i32,
        function: *const c_char,
    ) -> *mut c_void {
        assert!(!self.allocator.is_null());

        let mut size_in_bytes = size_in_bytes;
        let mut alignment = alignment;
        let mem = unsafe {
            (*self.allocator).allocate(
                &mut size_in_bytes,
                &mut alignment,
                event_type,
                provide_accurate_size_on_deallocate,
                source_file,
                source_line,
                function,
            )
        };
        assert!(!mem.is_null());
        mem
    }

    #[inline]
    pub fn deallocate(
        &self,
        memory: *mut c_void,
        event_type: NiMemEventType,
        size_in_bytes: usize,
    ) {
        assert!(!self.allocator.is_null());
        unsafe {
            (*self.allocator).deallocate(memory, event_type, size_in_bytes);
        }
    }

    #[inline]
    pub fn reallocate(
        &self,
        memory: *mut c_void,
        size_in_bytes: usize,
        alignment: usize,
        event_type: NiMemEventType,
        provide_accurate_size_on_deallocate: bool,
        size_current: usize,
        source_file: *const c_char,
        source_line: i32,
        function: *const c_char,
    ) -> *mut c_void {
        assert!(!self.allocator.is_null());

        let mut size_in_bytes = size_in_bytes;
        let mut alignment = alignment;
        let mem = unsafe {
            (*self.allocator).reallocate(
                memory,
                &mut size_in_bytes,
                &mut alignment,
                event_type,
                provide_accurate_size_on_deallocate,
                size_current,
                source_file,
                source_line,
                function,
            )
        };
        assert!(!mem.is_null());
        mem
    }

    #[inline]
    pub fn track_allocate(
        &self,
        memory: *const c_void,
        size_in_bytes: usize,
        event_type: NiMemEventType,
        source_file: *const c_char,
        source_line: i32,
        function: *const c_char,
    ) -> bool {
        assert!(!self.allocator.is_null());
        unsafe {
            (*self.allocator).track_allocate(
                memory,
                size_in_bytes,
                event_type,
                source_file,
                source_line,
                function,
            )
        }
    }

    #[inline]
    pub fn track_deallocate(&self, memory: *const c_void, event_type: NiMemEventType) -> bool {
        assert!(!self.allocator.is_null());
        unsafe { (*self.allocator).track_deallocate(memory, event_type) }
    }
}
