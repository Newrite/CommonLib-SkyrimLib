/// C++ `RE::SimpleArray<T>`
#[repr(C)]
pub struct SimpleArray<T> {
    pub data: *mut T, // 00
}

const _: () = assert!(core::mem::size_of::<SimpleArray<u8>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(SimpleArray<u8>, data) == 0x00);

impl<T> Default for SimpleArray<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleArray<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn data(&self) -> *mut T {
        self.data
    }

    #[inline(always)]
    pub fn begin(&self) -> *mut T {
        self.data()
    }

    #[inline(always)]
    pub fn cbegin(&self) -> *const T {
        self.begin()
    }

    #[inline(always)]
    pub fn end(&self) -> *mut T {
        if self.data.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { self.data.add(self.size()) }
        }
    }

    #[inline(always)]
    pub fn cend(&self) -> *const T {
        self.end()
    }

    #[inline(always)]
    pub fn front(&self) -> &T {
        debug_assert!(self.size() > 0);
        unsafe { &*self.data }
    }

    #[inline(always)]
    pub fn back(&self) -> &T {
        debug_assert!(self.size() > 0);
        unsafe { &*self.data.add(self.size() - 1) }
    }

    #[inline(always)]
    pub fn with_count(count: usize) -> Self
    where
        T: Default,
    {
        if count == 0 {
            return Self::new();
        }

        let header_bytes = if core::mem::align_of::<T>() > core::mem::align_of::<usize>() {
            core::mem::size_of::<T>()
        } else {
            core::mem::size_of::<usize>()
        };
        let bytes = header_bytes + core::mem::size_of::<T>() * count;
        let head = unsafe { crate::ffi::commonlib_malloc(bytes) }.cast::<u8>();
        if head.is_null() {
            return Self::new();
        }

        unsafe {
            head.cast::<usize>().write(count);
        }

        let data = if core::mem::align_of::<T>() > core::mem::align_of::<usize>() {
            unsafe { head.cast::<T>().add(1) }
        } else {
            unsafe { head.cast::<usize>().add(1).cast::<T>() }
        };

        for i in 0..count {
            unsafe { data.add(i).write(T::default()) };
        }

        Self { data }
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.empty()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        if self.data.is_null() {
            0
        } else {
            unsafe { *(self.get_head().cast::<usize>()) }
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size()
    }

    #[inline(always)]
    pub fn get_head(&self) -> *mut core::ffi::c_void {
        debug_assert!(!self.data.is_null());
        if core::mem::align_of::<T>() > core::mem::align_of::<usize>() {
            unsafe { self.data.sub(1).cast() }
        } else {
            unsafe { self.data.cast::<usize>().sub(1).cast() }
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        if self.data.is_null() {
            return;
        }

        unsafe {
            core::ptr::drop_in_place(self.as_mut_slice());
            crate::ffi::commonlib_free(self.get_head());
        }
        self.data = core::ptr::null_mut();
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.size()) }
        }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.size()) }
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T> Drop for SimpleArray<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.clear();
    }
}
