use crate::re::{BSStaticArray, BSTArray, BSTArrayAllocator, hkArray, hkArrayBase, hkInplaceArray};

use super::super::sealed;
use super::types::ContiguousSequence;

impl<T, A> sealed::Sealed for BSTArray<T, A> where A: BSTArrayAllocator {}

impl<T, A> ContiguousSequence<T> for BSTArray<T, A>
where
    A: BSTArrayAllocator,
{
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len()
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.capacity())
    }
}

impl<T> sealed::Sealed for BSStaticArray<T> {}

impl<T> ContiguousSequence<T> for BSStaticArray<T> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len()
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.len())
    }
}

impl<T> sealed::Sealed for hkArrayBase<T> {}

impl<T> ContiguousSequence<T> for hkArrayBase<T> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.capacity().max(0) as u32)
    }
}

impl<T, Allocator> sealed::Sealed for hkArray<T, Allocator> {}

impl<T, Allocator> ContiguousSequence<T> for hkArray<T, Allocator> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.base.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.base.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.base.capacity().max(0) as u32)
    }
}

impl<T, const N: usize, Allocator> sealed::Sealed for hkInplaceArray<T, N, Allocator> {}

impl<T, const N: usize, Allocator> ContiguousSequence<T> for hkInplaceArray<T, N, Allocator> {
    #[inline(always)]
    fn len(&self) -> u32 {
        self.base.base.len().min(u32::MAX as usize) as u32
    }

    #[inline(always)]
    fn data(&self) -> *const T {
        self.base.base.data()
    }

    #[inline(always)]
    fn capacity_hint(&self) -> Option<u32> {
        Some(self.base.base.capacity().max(0) as u32)
    }
}
