#![allow(non_camel_case_types)]

/// C++ `RE::GArrayDefaultPolicy`
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct GArrayDefaultPolicy {
    pub capacity: usize, // 00
}

const _: () = assert!(core::mem::size_of::<GArrayDefaultPolicy>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GArrayDefaultPolicy, capacity) == 0x0);

impl Clone for GArrayDefaultPolicy {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self::default()
    }
}

pub trait GArraySizePolicy {
    fn get_min_capacity(&self) -> usize;
    fn get_granularity(&self) -> usize;
    fn never_shrinking(&self) -> bool;
    fn get_capacity(&self) -> usize;
    fn set_capacity(&mut self, capacity: usize);
}

impl GArraySizePolicy for GArrayDefaultPolicy {
    #[inline(always)]
    fn get_min_capacity(&self) -> usize {
        0
    }

    #[inline(always)]
    fn get_granularity(&self) -> usize {
        4
    }

    #[inline(always)]
    fn never_shrinking(&self) -> bool {
        false
    }

    #[inline(always)]
    fn get_capacity(&self) -> usize {
        self.capacity
    }

    #[inline(always)]
    fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
    }
}
