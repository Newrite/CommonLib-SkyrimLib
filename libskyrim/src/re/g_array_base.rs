#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::ops::{Add, Deref, DerefMut, Index, IndexMut, Sub};

use crate::re::{GAllocatorTraits, GArrayData, GArrayDefaultPolicy, GArraySizePolicy};

/// C++ `RE::GArrayBase<T>::iterator`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GArrayBaseIterator<T, Allocator: GAllocatorTraits<T>, SizePolicy = GArrayDefaultPolicy> {
    pub array: *mut GArrayBase<GArrayData<T, Allocator, SizePolicy>>, // 00
    pub cur_index: isize,                                             // 08
}

const _: () = assert!(
    core::mem::size_of::<
        GArrayBaseIterator<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
    >() == 0x10
);

/// C++ `RE::GArrayBase<T>::const_iterator`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GArrayBaseConstIterator<
    T,
    Allocator: GAllocatorTraits<T>,
    SizePolicy = GArrayDefaultPolicy,
> {
    pub array: *const GArrayBase<GArrayData<T, Allocator, SizePolicy>>, // 00
    pub cur_index: isize,                                               // 08
}

const _: () = assert!(
    core::mem::size_of::<
        GArrayBaseConstIterator<
            *mut c_void,
            crate::re::GAllocatorGH<*mut c_void>,
            GArrayDefaultPolicy,
        >,
    >() == 0x10
);

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Default
    for GArrayBaseIterator<T, Allocator, SizePolicy>
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            array: core::ptr::null_mut(),
            cur_index: -1,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBaseIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    pub const fn new(
        array: *mut GArrayBase<GArrayData<T, Allocator, SizePolicy>>,
        cur_index: isize,
    ) -> Self {
        Self { array, cur_index }
    }

    #[inline(always)]
    pub fn get_ptr(&self) -> *mut T {
        debug_assert!(!self.array.is_null());
        unsafe { (&*self.array).data.data.add(self.cur_index as usize) }
    }

    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.array.is_null()
            || self.cur_index < 0
            || self.cur_index >= unsafe { (*self.array).get_size() as isize }
    }

    #[inline(always)]
    pub const fn get_index(&self) -> isize {
        self.cur_index
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBaseIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy + Default,
{
    #[inline(always)]
    pub fn remove(&mut self) {
        if !self.is_finished() {
            unsafe {
                (*self.array).remove_at(self.cur_index as usize);
            }
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Deref
    for GArrayBaseIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self.array.is_null());
        unsafe { (*self.array).at(self.cur_index as usize) }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> DerefMut
    for GArrayBaseIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self.array.is_null());
        unsafe { (*self.array).at_mut(self.cur_index as usize) }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Add<i32>
    for GArrayBaseIterator<T, Allocator, SizePolicy>
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            array: self.array,
            cur_index: self.cur_index + rhs as isize,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Sub<i32>
    for GArrayBaseIterator<T, Allocator, SizePolicy>
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            array: self.array,
            cur_index: self.cur_index - rhs as isize,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Sub
    for GArrayBaseIterator<T, Allocator, SizePolicy>
{
    type Output = isize;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.array == rhs.array);
        self.cur_index - rhs.cur_index
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Default
    for GArrayBaseConstIterator<T, Allocator, SizePolicy>
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            array: core::ptr::null(),
            cur_index: -1,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy>
    GArrayBaseConstIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    pub const fn new(
        array: *const GArrayBase<GArrayData<T, Allocator, SizePolicy>>,
        cur_index: isize,
    ) -> Self {
        Self { array, cur_index }
    }

    #[inline(always)]
    pub fn get_ptr(&self) -> *const T {
        debug_assert!(!self.array.is_null());
        unsafe { (&*self.array).data.data.add(self.cur_index as usize) }
    }

    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        self.array.is_null()
            || self.cur_index < 0
            || self.cur_index >= unsafe { (*self.array).get_size() as isize }
    }

    #[inline(always)]
    pub const fn get_index(&self) -> isize {
        self.cur_index
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Deref
    for GArrayBaseConstIterator<T, Allocator, SizePolicy>
where
    SizePolicy: GArraySizePolicy,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        debug_assert!(!self.array.is_null());
        unsafe { (*self.array).at(self.cur_index as usize) }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Add<i32>
    for GArrayBaseConstIterator<T, Allocator, SizePolicy>
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            array: self.array,
            cur_index: self.cur_index + rhs as isize,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Sub<i32>
    for GArrayBaseConstIterator<T, Allocator, SizePolicy>
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            array: self.array,
            cur_index: self.cur_index - rhs as isize,
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Sub
    for GArrayBaseConstIterator<T, Allocator, SizePolicy>
{
    type Output = isize;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(self.array == rhs.array);
        self.cur_index - rhs.cur_index
    }
}

/// C++ `RE::GArrayBase<T>`
#[repr(C)]
pub struct GArrayBase<Storage> {
    pub data: Storage, // 00
}

const _: () = assert!(
    core::mem::size_of::<
        GArrayBase<
            GArrayData<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        >,
    >() == 0x18
);
const _: () = assert!(
    core::mem::offset_of!(
        GArrayBase<
            GArrayData<*mut c_void, crate::re::GAllocatorGH<*mut c_void>, GArrayDefaultPolicy>,
        >,
        data
    ) == 0x0
);

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: Default,
{
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            data: GArrayData::new(),
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Default
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: Default,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Clone
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Clone,
    T: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    pub fn get_size_policy(&self) -> &SizePolicy {
        &self.data.policy
    }

    #[inline(always)]
    pub fn set_size_policy(&mut self, policy: SizePolicy) {
        self.data.policy = policy;
    }

    #[inline(always)]
    pub fn never_shrinking(&self) -> bool {
        self.data.policy.never_shrinking()
    }

    #[inline(always)]
    pub fn get_size(&self) -> usize {
        self.data.size
    }

    #[inline(always)]
    pub fn get_capacity(&self) -> usize {
        self.data.get_capacity()
    }

    #[inline(always)]
    pub fn get_num_bytes(&self) -> usize {
        self.get_capacity() * core::mem::size_of::<T>()
    }

    #[inline(always)]
    pub fn at(&self, index: usize) -> &T {
        debug_assert!(index < self.data.size);
        unsafe { &*self.data.data.add(index) }
    }

    #[inline(always)]
    pub fn at_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.data.size);
        unsafe { &mut *self.data.data.add(index) }
    }

    #[inline(always)]
    pub fn front(&self) -> &T {
        self.at(0)
    }

    #[inline(always)]
    pub fn front_mut(&mut self) -> &mut T {
        self.at_mut(0)
    }

    #[inline(always)]
    pub fn back(&self) -> &T {
        self.at(self.data.size - 1)
    }

    #[inline(always)]
    pub fn back_mut(&mut self) -> &mut T {
        self.at_mut(self.data.size - 1)
    }

    #[inline(always)]
    pub fn begin(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy> {
        GArrayBaseIterator::new(self, 0)
    }

    #[inline(always)]
    pub fn end(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy> {
        GArrayBaseIterator::new(self, self.get_size() as isize)
    }

    #[inline(always)]
    pub fn begin_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy> {
        GArrayBaseConstIterator::new(self, 0)
    }

    #[inline(always)]
    pub fn end_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy> {
        GArrayBaseConstIterator::new(self, self.get_size() as isize)
    }

    #[inline(always)]
    pub fn last(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy> {
        GArrayBaseIterator::new(self, self.get_size() as isize - 1)
    }

    #[inline(always)]
    pub fn last_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy> {
        GArrayBaseConstIterator::new(self, self.get_size() as isize - 1)
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> Index<usize>
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.at(index)
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> IndexMut<usize>
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.at_mut(index)
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
{
    #[inline(always)]
    pub fn clear(&mut self) {
        self.data
            .base
            .resize_no_construct(self as *const Self as *const c_void, 0);
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
    T: Default,
{
    #[inline(always)]
    pub fn with_size(size: i32) -> Self {
        Self {
            data: GArrayData::with_size(size),
        }
    }

    #[inline(always)]
    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size);
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
{
    #[inline(always)]
    pub fn clear_and_release(&mut self) {
        self.data.clear_and_release();
    }

    #[inline(always)]
    pub fn reserve(&mut self, new_capacity: usize) {
        if new_capacity > self.data.get_capacity() {
            self.data.reserve(new_capacity);
        }
    }

    #[inline(always)]
    pub fn pop_back(&mut self) {
        debug_assert!(self.data.size > 0);
        self.data
            .base
            .resize_no_construct(self as *const Self as *const c_void, self.data.size - 1);
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
    T: Clone,
{
    #[inline(always)]
    pub fn with_value(default_value: &T) -> Self {
        let mut out = Self::new();
        out.push_back(default_value);
        out
    }

    #[inline(always)]
    pub fn with_value_and_size(default_value: &T, size: i32) -> Self {
        let len = size.max(0) as usize;
        let mut out = Self::new();
        if len != 0 {
            out.data.reserve(len);
            unsafe {
                Allocator::construct_array_fill(out.data.data, len, default_value);
            }
            out.data.size = len;
        }
        out
    }

    #[inline(always)]
    pub fn value_at(&self, index: usize) -> T {
        self.at(index).clone()
    }

    #[inline(always)]
    pub fn push_back(&mut self, value: &T) {
        self.data.push_back(value);
    }

    #[inline(always)]
    pub fn append_array(&mut self, other: &Self) {
        self.append_slice(other.data.data, other.get_size());
    }

    #[inline(always)]
    pub fn append_slice(&mut self, other: *const T, count: usize) {
        self.data.append(other, count);
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
    T: Default + Clone,
{
    #[inline(always)]
    pub fn insert_at(&mut self, index: usize, value: Option<&T>) {
        debug_assert!(index <= self.data.size);

        self.data.resize(self.data.size + 1);
        if index < self.data.size - 1 {
            unsafe {
                Allocator::copy_array_backward(
                    self.data.data.add(index + 1),
                    self.data.data.add(index),
                    self.data.size - 1 - index,
                );
            }
        }
        unsafe {
            match value {
                Some(value) => Allocator::construct_copy(self.data.data.add(index), value),
                None => Allocator::construct(self.data.data.add(index)),
            }
        }
    }

    #[inline(always)]
    pub fn insert_multiple_at(&mut self, index: usize, count: usize, value: Option<&T>) {
        debug_assert!(index <= self.data.size);

        if count == 0 {
            return;
        }

        self.data.resize(self.data.size + count);
        if index < self.data.size - count {
            unsafe {
                Allocator::copy_array_backward(
                    self.data.data.add(index + count),
                    self.data.data.add(index),
                    self.data.size - count - index,
                );
            }
        }

        unsafe {
            match value {
                Some(value) => {
                    Allocator::construct_array_fill(self.data.data.add(index), count, value)
                }
                None => Allocator::construct_array(self.data.data.add(index), count),
            }
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBase<GArrayData<T, Allocator, SizePolicy>>
where
    SizePolicy: GArraySizePolicy + Default,
{
    #[inline(always)]
    pub fn push_back_alt<S>(&mut self, value: S)
    where
        S: Into<T>,
    {
        self.data.push_back_alt(value);
    }

    #[inline(always)]
    pub fn remove_multiple_at(&mut self, index: usize, count: usize) {
        debug_assert!(index + count <= self.data.size);

        if count == 0 {
            return;
        }

        if self.data.size == count {
            self.clear();
        } else {
            unsafe {
                Allocator::destruct_array(self.data.data.add(index), count);
                Allocator::copy_array_forward(
                    self.data.data.add(index),
                    self.data.data.add(index + count),
                    self.data.size - count - index,
                );
            }
            self.data.size -= count;
        }
    }

    #[inline(always)]
    pub fn remove_at(&mut self, index: usize) {
        debug_assert!(index < self.data.size);

        if self.data.size == 1 {
            self.clear();
        } else {
            unsafe {
                Allocator::destruct(self.data.data.add(index));
                Allocator::copy_array_forward(
                    self.data.data.add(index),
                    self.data.data.add(index + 1),
                    self.data.size - 1 - index,
                );
            }
            self.data.size -= 1;
        }
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy>
    AsRef<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
{
    #[inline(always)]
    fn as_ref(&self) -> &GArrayBase<GArrayData<T, Allocator, SizePolicy>> {
        self
    }
}

impl<T, Allocator: GAllocatorTraits<T>, SizePolicy>
    AsMut<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
    for GArrayBase<GArrayData<T, Allocator, SizePolicy>>
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GArrayBase<GArrayData<T, Allocator, SizePolicy>> {
        self
    }
}

pub trait GArrayBaseExt<T, Allocator: GAllocatorTraits<T>, SizePolicy>:
    AsRef<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
    + AsMut<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
{
    #[inline(always)]
    fn get_size_policy<'a>(&'a self) -> &'a SizePolicy
    where
        SizePolicy: GArraySizePolicy,
        T: 'a,
        Allocator: 'a,
    {
        self.as_ref().get_size_policy()
    }

    #[inline(always)]
    fn set_size_policy(&mut self, policy: SizePolicy)
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_mut().set_size_policy(policy);
    }

    #[inline(always)]
    fn never_shrinking(&self) -> bool
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().never_shrinking()
    }

    #[inline(always)]
    fn get_size(&self) -> usize
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().get_size()
    }

    #[inline(always)]
    fn get_capacity(&self) -> usize
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().get_capacity()
    }

    #[inline(always)]
    fn get_num_bytes(&self) -> usize
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().get_num_bytes()
    }

    #[inline(always)]
    fn at<'a>(&'a self, index: usize) -> &'a T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_ref().at(index)
    }

    #[inline(always)]
    fn at_mut<'a>(&'a mut self, index: usize) -> &'a mut T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_mut().at_mut(index)
    }

    #[inline(always)]
    fn front<'a>(&'a self) -> &'a T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_ref().front()
    }

    #[inline(always)]
    fn front_mut<'a>(&'a mut self) -> &'a mut T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_mut().front_mut()
    }

    #[inline(always)]
    fn back<'a>(&'a self) -> &'a T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_ref().back()
    }

    #[inline(always)]
    fn back_mut<'a>(&'a mut self) -> &'a mut T
    where
        SizePolicy: GArraySizePolicy,
        Allocator: 'a,
        SizePolicy: 'a,
    {
        self.as_mut().back_mut()
    }

    #[inline(always)]
    fn begin(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_mut().begin()
    }

    #[inline(always)]
    fn end(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_mut().end()
    }

    #[inline(always)]
    fn begin_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().begin_const()
    }

    #[inline(always)]
    fn end_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().end_const()
    }

    #[inline(always)]
    fn clear_and_release(&mut self)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
    {
        self.as_mut().clear_and_release();
    }

    #[inline(always)]
    fn clear(&mut self)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Default,
    {
        self.as_mut().clear();
    }

    #[inline(always)]
    fn resize(&mut self, new_size: usize)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Default,
    {
        self.as_mut().resize(new_size);
    }

    #[inline(always)]
    fn reserve(&mut self, new_capacity: usize)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
    {
        self.as_mut().reserve(new_capacity);
    }

    #[inline(always)]
    fn value_at(&self, index: usize) -> T
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Clone,
    {
        self.as_ref().value_at(index)
    }

    #[inline(always)]
    fn push_back(&mut self, value: &T)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Clone,
    {
        self.as_mut().push_back(value);
    }

    #[inline(always)]
    fn push_back_alt<S>(&mut self, value: S)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        S: Into<T>,
    {
        self.as_mut().push_back_alt(value);
    }

    #[inline(always)]
    fn pop_back(&mut self)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
    {
        self.as_mut().pop_back();
    }

    #[inline(always)]
    fn last(&mut self) -> GArrayBaseIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_mut().last()
    }

    #[inline(always)]
    fn last_const(&self) -> GArrayBaseConstIterator<T, Allocator, SizePolicy>
    where
        SizePolicy: GArraySizePolicy,
    {
        self.as_ref().last_const()
    }

    #[inline(always)]
    fn remove_multiple_at(&mut self, index: usize, count: usize)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
    {
        self.as_mut().remove_multiple_at(index, count);
    }

    #[inline(always)]
    fn remove_at(&mut self, index: usize)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
    {
        self.as_mut().remove_at(index);
    }

    #[inline(always)]
    fn insert_at(&mut self, index: usize, value: Option<&T>)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Default + Clone,
    {
        self.as_mut().insert_at(index, value);
    }

    #[inline(always)]
    fn insert_multiple_at(&mut self, index: usize, count: usize, value: Option<&T>)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Default + Clone,
    {
        self.as_mut().insert_multiple_at(index, count, value);
    }

    #[inline(always)]
    fn append_array(&mut self, other: &GArrayBase<GArrayData<T, Allocator, SizePolicy>>)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Clone,
    {
        self.as_mut().append_array(other);
    }

    #[inline(always)]
    fn append_slice(&mut self, other: *const T, count: usize)
    where
        SizePolicy: GArraySizePolicy + Default,
        Allocator: GAllocatorTraits<T>,
        T: Clone,
    {
        self.as_mut().append_slice(other, count);
    }
}

impl<U, T, Allocator: GAllocatorTraits<T>, SizePolicy> GArrayBaseExt<T, Allocator, SizePolicy> for U where
    U: AsRef<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
        + AsMut<GArrayBase<GArrayData<T, Allocator, SizePolicy>>>
{
}
