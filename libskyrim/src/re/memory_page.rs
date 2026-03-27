/// C++ `RE::BSScript::MemoryPage`
#[repr(C)]
pub struct MemoryPage {
    pub page_size: u32, // 00
    pub buf: [u8; 0],   // 04
}

const _: () = assert!(core::mem::size_of::<MemoryPage>() == 0x4);
const _: () = assert!(core::mem::offset_of!(MemoryPage, page_size) == 0x00);
const _: () = assert!(core::mem::offset_of!(MemoryPage, buf) == 0x04);

impl MemoryPage {
    #[inline(always)]
    pub fn get_data<T>(&self) -> *mut T {
        self.buf.as_ptr().cast_mut().cast::<T>()
    }

    #[inline(always)]
    pub fn get_head(&self) -> *mut core::ffi::c_void {
        self.buf.as_ptr().cast_mut().cast()
    }

    #[inline(always)]
    pub fn get_tail(&self) -> *mut core::ffi::c_void {
        unsafe {
            self.buf
                .as_ptr()
                .add(self.page_size as usize)
                .cast_mut()
                .cast()
        }
    }

    #[inline(always)]
    pub fn is_in_range(&self, ptr: *const core::ffi::c_void) -> bool {
        let head = self.get_head() as usize;
        let tail = self.get_tail() as usize;
        let ptr = ptr as usize;
        head <= ptr && ptr < tail
    }
}

impl crate::re::bst_smart_pointer::BSTSmartPointerAutoDeletable for MemoryPage {
    #[inline(always)]
    unsafe fn bst_delete(&self) {
        unsafe {
            crate::ffi::commonlib_free((self as *const Self).cast_mut().cast());
        }
    }
}
