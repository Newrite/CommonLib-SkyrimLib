use alloc::vec::Vec;
use core::ops::{Deref, DerefMut, Index, IndexMut};

use crate::re::type_traits::PapyrusScalar;
use crate::re::{Array, BSTSmartPointer, Variable};

pub trait PapyrusReferenceArrayElement: PapyrusScalar {
    unsafe fn unpack_reference_array_element(src: &Variable) -> Self;

    fn pack_reference_array_element(self, dst: &mut Variable) -> bool;
}

/// C++ `RE::BSScript::reference_array<T>`
///
/// This wrapper mirrors the source-backed semantics from `ReferenceArray.h`:
/// it unwraps a `Variable`-backed Papyrus array into a local `Vec<T>` on
/// construction and writes element changes back into the wrapped array on drop.
pub struct ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    unwrapped: Vec<T>,
    wrapped: BSTSmartPointer<Array>,
}

#[allow(non_camel_case_types)]
pub type reference_array<T> = ReferenceArray<T>;

impl<T> Default for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            unwrapped: Vec::new(),
            wrapped: BSTSmartPointer::default(),
        }
    }

    #[inline(always)]
    pub fn from_variable(wrapped: &Variable) -> Self {
        let mut out = Self::new();
        out.assign_variable(wrapped);
        out
    }

    #[inline(always)]
    pub fn assign_variable(&mut self, wrapped: &Variable) {
        self.do_unwrap(wrapped);
    }

    #[inline(always)]
    pub fn at(&self, pos: usize) -> &T {
        &self.unwrapped[pos]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, pos: usize) -> &mut T {
        &mut self.unwrapped[pos]
    }

    #[inline(always)]
    pub fn front(&self) -> &T {
        &self.unwrapped[0]
    }

    #[inline(always)]
    pub fn front_mut(&mut self) -> &mut T {
        &mut self.unwrapped[0]
    }

    #[inline(always)]
    pub fn back(&self) -> &T {
        let last = self.unwrapped.len() - 1;
        &self.unwrapped[last]
    }

    #[inline(always)]
    pub fn back_mut(&mut self) -> &mut T {
        let last = self.unwrapped.len() - 1;
        &mut self.unwrapped[last]
    }

    #[inline(always)]
    pub fn data(&self) -> *const T {
        self.unwrapped.as_ptr()
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut T {
        self.unwrapped.as_mut_ptr()
    }

    #[inline(always)]
    pub fn begin(&self) -> *const T {
        self.data()
    }

    #[inline(always)]
    pub fn begin_mut(&mut self) -> *mut T {
        self.data_mut()
    }

    #[inline(always)]
    pub fn cbegin(&self) -> *const T {
        self.begin()
    }

    #[inline(always)]
    pub fn end(&self) -> *const T {
        unsafe { self.data().add(self.unwrapped.len()) }
    }

    #[inline(always)]
    pub fn end_mut(&mut self) -> *mut T {
        unsafe { self.data_mut().add(self.unwrapped.len()) }
    }

    #[inline(always)]
    pub fn cend(&self) -> *const T {
        self.end()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.unwrapped.is_empty()
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.unwrapped.len()
    }

    #[inline(always)]
    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self, other);
    }

    #[inline(always)]
    fn do_wrap(&mut self) {
        if self.wrapped.is_null() {
            return;
        }

        let array = unsafe { &mut *self.wrapped.get() };
        debug_assert!(self.unwrapped.len() <= array.size() as usize);
        let size = core::cmp::min(self.unwrapped.len(), array.size() as usize);
        for (i, value) in self.unwrapped.drain(..size).enumerate() {
            let _ = value.pack_reference_array_element(array.get_mut(i as u32));
        }
    }

    #[inline(always)]
    fn do_unwrap(&mut self, wrapped: &Variable) {
        debug_assert!(wrapped.is_array());
        let arr = wrapped.get_array();
        if self.wrapped == arr {
            return;
        }

        self.unwrapped.clear();
        self.wrapped = arr;
        if self.wrapped.is_null() {
            return;
        }

        let wrapped_array = unsafe { &*self.wrapped.get() };
        self.unwrapped.reserve(wrapped_array.size() as usize);
        for i in 0..wrapped_array.size() {
            self.unwrapped
                .push(unsafe { T::unpack_reference_array_element(wrapped_array.get(i)) });
        }
    }
}

impl<T> Drop for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    fn drop(&mut self) {
        self.do_wrap();
    }
}

impl<T> Deref for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.unwrapped.as_slice()
    }
}

impl<T> DerefMut for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.unwrapped.as_mut_slice()
    }
}

impl<T> Index<usize> for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.unwrapped[index]
    }
}

impl<T> IndexMut<usize> for ReferenceArray<T>
where
    T: PapyrusReferenceArrayElement,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.unwrapped[index]
    }
}
