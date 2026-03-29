#![allow(non_camel_case_types)]

use crate::re::GArraySizePolicy;

/// C++ `RE::GArrayConstPolicy<MinCapacity, Granularity, NeverShrink>`
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GArrayConstPolicy<
    const MIN_CAPACITY: i32 = 0,
    const GRANULARITY: i32 = 4,
    const NEVER_SHRINK: bool = false,
> {
    pub capacity: usize, // 00
}

const _: () = assert!(core::mem::size_of::<GArrayConstPolicy>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GArrayConstPolicy, capacity) == 0x0);

impl<const MIN_CAPACITY: i32, const GRANULARITY: i32, const NEVER_SHRINK: bool> Default
    for GArrayConstPolicy<MIN_CAPACITY, GRANULARITY, NEVER_SHRINK>
{
    #[inline(always)]
    fn default() -> Self {
        Self { capacity: 0 }
    }
}

impl<const MIN_CAPACITY: i32, const GRANULARITY: i32, const NEVER_SHRINK: bool> Clone
    for GArrayConstPolicy<MIN_CAPACITY, GRANULARITY, NEVER_SHRINK>
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<const MIN_CAPACITY: i32, const GRANULARITY: i32, const NEVER_SHRINK: bool> GArraySizePolicy
    for GArrayConstPolicy<MIN_CAPACITY, GRANULARITY, NEVER_SHRINK>
{
    #[inline(always)]
    fn get_min_capacity(&self) -> usize {
        MIN_CAPACITY.max(0) as usize
    }

    #[inline(always)]
    fn get_granularity(&self) -> usize {
        GRANULARITY.max(0) as usize
    }

    #[inline(always)]
    fn never_shrinking(&self) -> bool {
        NEVER_SHRINK
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
