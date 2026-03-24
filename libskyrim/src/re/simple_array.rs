/// C++ `RE::SimpleArray<T>`
#[repr(C)]
pub struct SimpleArray<T> {
    pub data: *mut T, // 00
}

const _: () = assert!(core::mem::size_of::<SimpleArray<u8>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(SimpleArray<u8>, data) == 0x00);

impl<T> Default for SimpleArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleArray<T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
        }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn len(&self) -> usize {
        if self.data.is_null() {
            0
        } else {
            unsafe { *((self.data.cast::<usize>()).sub(1)) }
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len()) }
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len()) }
        }
    }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}
