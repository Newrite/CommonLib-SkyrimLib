//! Translation of `RE::BSTArray.h` / `RE::BSTArray.cpp`.
//!
//! Contains:
//! - `BSTArrayBase` — base class storing the array size
//! - `BSTArrayHeapAllocator` — default heap allocator using engine's `RE::malloc`/`RE::free`
//! - `BSScrapArrayAllocator` — per-thread scrap heap allocator
//! - `BSTArray<T, A>` — generic dynamic array (C++ `BSTArray<T, Allocator>`)
//! - `BSScrapArray<T>` — type alias for `BSTArray<T, BSScrapArrayAllocator>`
//! - `BSStaticArray<T>` — non-owning static array (pointer + size)
//! - `BSTSmallSharedArray<T>` — small shared array with inline storage for 0-1 elements

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;
use bytemuck::Zeroable;

use crate::re::scrap_heap::ScrapHeap;

// ─── BSTArrayBase ────────────────────────────────────────────────────────────

/// C++ `RE::BSTArrayBase`.
/// Base class for `BSTArray`, stores the element count.
///
/// Layout: `{ _size: u32 }` — 0x4 bytes total.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSTArrayBase {
    size: u32, // 0x00
}

const _: () = assert!(core::mem::size_of::<BSTArrayBase>() == 0x4);

impl BSTArrayBase {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { size: 0 }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.size
    }

    #[inline(always)]
    pub fn set_size(&mut self, new_size: u32) {
        self.size = new_size;
    }
}

// ─── BSTArrayAllocator trait ─────────────────────────────────────────────────

/// Trait replacing C++ `IAllocatorFunctor` — defines the allocator interface
/// used by `BSTArray`.
///
/// # Safety
/// Implementations must return memory blocks that are valid for `size` bytes
/// and properly aligned for any `T` that will be stored.
pub unsafe trait BSTArrayAllocator {
    fn data(&self) -> *mut u8;
    fn capacity(&self) -> u32;
    /// Allocate `size` bytes. Returns a zeroed memory block.
    fn allocate(&mut self, size: usize) -> *mut u8;
    /// Deallocate a previously allocated block.
    fn deallocate(&mut self, ptr: *mut u8);
    /// Update the stored data pointer and capacity after a reallocation.
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, elem_size: usize);
}

// ─── BSTArrayHeapAllocator ───────────────────────────────────────────────────

/// C++ `RE::BSTArrayHeapAllocator`.
/// Default allocator for `BSTArray` — uses `RE::malloc` / `RE::free`
/// through the C++ bridge, exactly matching the engine's allocation path.
///
/// Layout: `{ _data: *mut u8, _capacity: u32, _pad0c: u32 }` — 0x10 bytes total.
#[repr(C)]
pub struct BSTArrayHeapAllocator {
    _data: *mut u8,     // 0x00
    _capacity: u32,     // 0x08
    _pad0c: u32,        // 0x0C
}

const _: () = assert!(core::mem::size_of::<BSTArrayHeapAllocator>() == 0x10);

impl BSTArrayHeapAllocator {
    pub const fn new() -> Self {
        Self {
            _data: ptr::null_mut(),
            _capacity: 0,
            _pad0c: 0,
        }
    }
}

unsafe impl BSTArrayAllocator for BSTArrayHeapAllocator {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        self._data
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return ptr::null_mut();
        }
        unsafe {
            // RE::malloc → MemoryManager::GetSingleton()->Allocate(size, 0, false)
            let mem = crate::ffi::commonlib_malloc(size);
            if mem.is_null() {
                panic!("BSTArrayHeapAllocator: out of memory");
            }
            ptr::write_bytes(mem as *mut u8, 0, size);
            mem as *mut u8
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            unsafe {
                // RE::free → MemoryManager::GetSingleton()->Deallocate(ptr, false)
                crate::ffi::commonlib_free(ptr as *mut c_void);
            }
        }
    }

    #[inline(always)]
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, _elem_size: usize) {
        self._data = data;
        self._capacity = capacity;
    }
}

// ─── BSTSmallArrayHeapAllocator<N> ───────────────────────────────────────────

/// C++ `RE::BSTSmallArrayHeapAllocator<N>`.
/// An allocator with `N` bytes of inline storage. If the requested allocation
/// fits within the inline buffer, no heap allocation occurs.
///
/// C++ uses bitfields `_capacity: 31` and `_local: 1` packed into a single `u32`.
/// MSVC packs them as: capacity in bits 0-30, local flag in bit 31.
///
/// Layout: `{ _capacity_and_local: u32, _pad04: u32, _data: union { heap, local[N] } }`
#[repr(C)]
pub struct BSTSmallArrayHeapAllocator<const N: usize> {
    /// Packed bitfield: bits 0-30 = capacity, bit 31 = local flag
    _capacity_and_local: u32,  // 0x00
    _pad04: u32,               // 0x04
    _data: SmallArrayData<N>,  // 0x08
}

/// Internal union for `BSTSmallArrayHeapAllocator`.
/// `heap` is a pointer to heap-allocated storage.
/// `local` is inline stack storage of `N` bytes.
#[repr(C)]
pub union SmallArrayData<const N: usize> {
    pub heap: *mut u8,
    pub local: [u8; N],
}

/// Bitfield constants for `BSTSmallArrayHeapAllocator`.
const CAPACITY_MASK: u32 = 0x7FFF_FFFF; // bits 0-30
const LOCAL_FLAG: u32 = 0x8000_0000;     // bit 31

impl<const N: usize> BSTSmallArrayHeapAllocator<N> {
    pub const fn new() -> Self {
        Self {
            // _capacity = 0, _local = 1 → bit 31 set
            _capacity_and_local: LOCAL_FLAG,
            _pad04: 0,
            _data: SmallArrayData { local: [0u8; N] },
        }
    }

    /// Returns true if using inline (local) storage.
    #[inline(always)]
    fn is_local(&self) -> bool {
        (self._capacity_and_local & LOCAL_FLAG) != 0
    }

    /// Sets the local flag.
    #[inline(always)]
    fn set_local(&mut self, local: bool) {
        if local {
            self._capacity_and_local |= LOCAL_FLAG;
        } else {
            self._capacity_and_local &= CAPACITY_MASK;
        }
    }

    /// Gets capacity from the packed bitfield (bits 0-30).
    #[inline(always)]
    fn get_capacity(&self) -> u32 {
        self._capacity_and_local & CAPACITY_MASK
    }

    /// Sets capacity in the packed bitfield (preserving the local flag).
    #[inline(always)]
    fn set_capacity(&mut self, cap: u32) {
        debug_assert!(cap <= CAPACITY_MASK, "capacity exceeds 31-bit max");
        self._capacity_and_local = (self._capacity_and_local & LOCAL_FLAG) | (cap & CAPACITY_MASK);
    }
}

impl<const N: usize> Default for BSTSmallArrayHeapAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<const N: usize> BSTArrayAllocator for BSTSmallArrayHeapAllocator<N> {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        if self.is_local() {
            // Return pointer to inline storage
            unsafe { self._data.local.as_ptr() as *mut u8 }
        } else {
            unsafe { self._data.heap }
        }
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self.get_capacity()
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > N {
            // Exceeds inline storage — allocate from engine heap
            unsafe {
                let mem = crate::ffi::commonlib_malloc(size);
                if mem.is_null() {
                    panic!("BSTSmallArrayHeapAllocator: out of memory");
                }
                ptr::write_bytes(mem as *mut u8, 0, size);
                mem as *mut u8
            }
        } else {
            // Fits in inline storage
            unsafe { self._data.local.as_mut_ptr() }
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        // Only free if the pointer is NOT our inline buffer
        let local_ptr = unsafe { self._data.local.as_ptr() as *mut u8 };
        if ptr != local_ptr && !ptr.is_null() {
            unsafe {
                crate::ffi::commonlib_free(ptr as *mut c_void);
            }
        }
    }

    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, elem_size: usize) {
        self.set_capacity(capacity);
        if (capacity as usize) * elem_size > N {
            // Switched to heap mode
            self.set_local(false);
            self._data = SmallArrayData { heap: data };
        }
        // If fits in local, keep local flag set (data is already in local buffer)
    }
}

// ─── BSScrapArrayAllocator ───────────────────────────────────────────────────

/// C++ `RE::BSScrapArrayAllocator`.
/// Allocator using the engine's per-thread `ScrapHeap`.
/// All engine calls go through the C++ bridge for exact parity with CommonLib.
///
/// Layout: `{ _allocator: *mut ScrapHeap, _data: *mut u8, _capacity: u32, _pad14: u32 }` — 0x18 bytes.
#[repr(C)]
pub struct BSScrapArrayAllocator {
    _allocator: *mut ScrapHeap,  // 0x00
    _data: *mut u8,              // 0x08
    _capacity: u32,              // 0x10
    _pad14: u32,                 // 0x14
}

const _: () = assert!(core::mem::size_of::<BSScrapArrayAllocator>() == 0x18);

impl BSScrapArrayAllocator {
    pub const fn new() -> Self {
        Self {
            _allocator: ptr::null_mut(),
            _data: ptr::null_mut(),
            _capacity: 0,
            _pad14: 0,
        }
    }
}

unsafe impl BSTArrayAllocator for BSScrapArrayAllocator {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        self._data
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        // Lazy-initialize the scrap heap from MemoryManager (mirrors BSTArray.cpp)
        if self._allocator.is_null() {
            unsafe {
                let mgr = crate::ffi::commonlib_memory_manager_get_singleton();
                if !mgr.is_null() {
                    self._allocator = crate::ffi::commonlib_memory_manager_get_thread_scrap_heap(mgr)
                        as *mut ScrapHeap;
                }
            }
        }
        assert!(!self._allocator.is_null(), "BSScrapArrayAllocator: no ScrapHeap");

        unsafe {
            let mem = crate::ffi::commonlib_scrap_heap_allocate(
                self._allocator as *mut c_void,
                size,
                core::mem::align_of::<*mut u8>(), // alignof(void*)
            );
            assert!(!mem.is_null(), "BSScrapArrayAllocator: allocation failed");
            ptr::write_bytes(mem as *mut u8, 0, size);
            mem as *mut u8
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        if self._allocator.is_null() {
            assert!(ptr.is_null(), "BSScrapArrayAllocator: no allocator for dealloc");
            return;
        }
        unsafe {
            crate::ffi::commonlib_scrap_heap_deallocate(
                self._allocator as *mut c_void,
                ptr as *mut c_void,
            );
        }
    }

    #[inline(always)]
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, _elem_size: usize) {
        self._data = data;
        self._capacity = capacity;
    }
}

// ─── BSTArray<T, A> ─────────────────────────────────────────────────────────

/// C++ `RE::BSTArray<T, Allocator>`.
///
/// A generic dynamic array composed of an allocator (first) and `BSTArrayBase` (second).
/// Memory layout matches C++ multiple inheritance order: `Allocator` then `BSTArrayBase`.
///
/// Default allocator is `BSTArrayHeapAllocator`.
#[repr(C)]
pub struct BSTArray<T, A: BSTArrayAllocator = BSTArrayHeapAllocator> {
    allocator: A,
    base: BSTArrayBase,
    _marker: PhantomData<*mut T>,
}

/// Default capacity for new allocations (Beth default).
const BST_ARRAY_DF_CAP: u32 = 4;

/// Growth factor (2.0 as in the original).
const BST_ARRAY_GROWTH_FACTOR: f32 = 2.0;

// Provide Default implementations for the allocators
impl Default for BSTArrayHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BSScrapArrayAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: BSTArrayAllocator + Default> BSTArray<T, A> {
    /// Creates a new empty `BSTArray`.
    pub fn new() -> Self {
        Self {
            allocator: A::default(),
            base: BSTArrayBase::new(),
            _marker: PhantomData,
        }
    }
}

impl<T, A: BSTArrayAllocator> BSTArray<T, A> {
    /// Number of elements in the array.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.base.len()
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    /// Current capacity (in number of elements).
    #[inline(always)]
    pub fn capacity(&self) -> u32 {
        self.allocator.capacity()
    }

    /// Returns a raw pointer to the first element, or null if empty.
    #[inline(always)]
    pub fn data(&self) -> *const T {
        self.allocator.data() as *const T
    }

    /// Returns a mutable raw pointer to the first element.
    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut T {
        self.allocator.data() as *mut T
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid and that
    /// `T` is the correct element type matching the engine's array.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        let len = self.len() as usize;
        if len == 0 {
            &[]
        } else {
            core::slice::from_raw_parts(self.data(), len)
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len() as usize;
        if len == 0 {
            &mut []
        } else {
            core::slice::from_raw_parts_mut(self.data_mut(), len)
        }
    }

    /// Reserves capacity for at least `new_cap` elements.
    pub fn reserve(&mut self, new_cap: u32) {
        if new_cap > self.capacity() {
            self.change_capacity(new_cap);
        }
    }

    /// Shrinks capacity to match the current size.
    pub fn shrink_to_fit(&mut self) {
        let new_cap = self.len();
        if new_cap != self.capacity() {
            self.change_capacity(new_cap);
        }
    }

    /// Adds an element to the end of the array.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    pub unsafe fn push(&mut self, value: T) {
        if self.len() == self.capacity() {
            self.grow_capacity();
        }
        let size = self.len();
        self.base.set_size(size + 1);
        let dst = self.data_mut().add(size as usize);
        ptr::write(dst, value);
    }

    /// Removes and returns the last element.
    ///
    /// # Safety
    /// The caller must ensure the array is non-empty and data pointer is valid.
    pub unsafe fn pop(&mut self) -> T {
        assert!(!self.is_empty());
        let new_size = self.len() - 1;
        let val = ptr::read(self.data().add(new_size as usize));
        self.base.set_size(new_size);
        val
    }

    /// Clears all elements, calling drop on each.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    pub unsafe fn clear(&mut self) {
        if !self.is_empty() {
            self.change_size(0);
        }
    }

    /// Resizes the array to `new_size` elements.
    /// New elements are zero-initialized.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    ///
    /// # Compile Error
    /// This will fail to compile if `T` is a complex C++ class (like `TESForm`)
    /// that cannot be safely zero-initialized.
    pub unsafe fn resize(&mut self, new_size: u32)
    where
        T: Zeroable
    {
        if new_size != self.len() {
            self.change_size(new_size);
        }
    }

    /// Releases all memory and resets the array.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    pub unsafe fn release(&mut self) {
        self.clear();
        self.change_capacity(0);
    }

    // ── Private helpers ──────────────────────────────────────────────────

    /// Typed allocation: allocates memory for `num` elements.
    fn allocate_elements(&mut self, num: u32) -> *mut T {
        let size = (num as usize) * core::mem::size_of::<T>();
        self.allocator.allocate(size) as *mut T
    }

    /// Update allocator traits using element size.
    fn set_allocator_traits_typed(&mut self, data: *mut T, capacity: u32) {
        self.allocator.set_allocator_traits(
            data as *mut u8,
            capacity,
            core::mem::size_of::<T>(),
        );
    }

    /// Change the allocated capacity, copying existing data.
    fn change_capacity(&mut self, new_capacity: u32) {
        let new_data = if new_capacity > 0 {
            self.allocate_elements(new_capacity)
        } else {
            ptr::null_mut()
        };

        let old_data = self.data_mut();
        if !old_data.is_null() {
            let old_capacity = self.capacity();
            if !new_data.is_null() {
                let to_copy = core::cmp::min(old_capacity, new_capacity) as usize;
                let bytes_to_copy = to_copy * core::mem::size_of::<T>();
                unsafe {
                    ptr::copy_nonoverlapping(
                        old_data as *const u8,
                        new_data as *mut u8,
                        bytes_to_copy,
                    );
                }
            }
            self.allocator.deallocate(old_data as *mut u8);
        }

        self.set_allocator_traits_typed(new_data, new_capacity);
    }

    /// Change the size, constructing or destroying as needed.
    unsafe fn change_size(&mut self, new_size: u32)
    where
        T: Zeroable
    {
        if new_size > self.capacity() {
            self.grow_capacity_to(new_size);
        }

        let old_size = self.len();
        if new_size > old_size {
            let data = self.data_mut();
            ptr::write_bytes(
                data.add(old_size as usize) as *mut u8,
                0,
                (new_size - old_size) as usize * core::mem::size_of::<T>(),
            );
        } else if new_size < old_size {
            let data = self.data_mut();
            for i in new_size..old_size {
                ptr::drop_in_place(data.add(i as usize));
            }
        }

        self.base.set_size(new_size);
    }

    /// Calculate the next capacity value (doubles, or starts at `DF_CAP`).
    fn next_capacity(&self) -> u32 {
        Self::next_capacity_from(self.capacity())
    }

    fn next_capacity_from(hint: u32) -> u32 {
        if hint > 0 {
            // ceil(hint * 2.0) — matches C++ std::ceil(cap * GROWTH_FACTOR)
            let grown = (hint as f32) * BST_ARRAY_GROWTH_FACTOR;
            let truncated = grown as u32;
            if (truncated as f32) < grown {
                truncated + 1
            } else {
                truncated
            }
        } else {
            BST_ARRAY_DF_CAP
        }
    }

    fn grow_capacity(&mut self) {
        let next = self.next_capacity();
        self.change_capacity(next);
    }

    fn grow_capacity_to(&mut self, hint: u32) {
        let next = Self::next_capacity_from(hint);
        self.change_capacity(next);
    }
}

impl<T, A: BSTArrayAllocator> Drop for BSTArray<T, A> {
    fn drop(&mut self) {
        unsafe { self.release(); }
    }
}

/// Type alias for `BSTArray<T, BSScrapArrayAllocator>`.
/// C++ `RE::BSScrapArray<T>`.
pub type BSScrapArray<T> = BSTArray<T, BSScrapArrayAllocator>;

/// C++ `template <class T, std::uint32_t N = 1> using BSTSmallArray = BSTArray<T, BSTSmallArrayHeapAllocator<sizeof(T) * N>>`.
///
/// Note: In Rust, const generic expressions like `{core::mem::size_of::<T>() * N}` require
/// `#![feature(generic_const_exprs)]` which is unstable. Instead, the caller must specify
/// the byte count directly. For convenience, the default `N` matches `sizeof(T) * 1`
/// for pointer-sized types (8 bytes).
///
/// Usage: `BSTSmallArray<MyType, {core::mem::size_of::<MyType>()}>` for N=1,
/// or manually compute `sizeof(T) * count`.
pub type BSTSmallArray<T, const N: usize> = BSTArray<T, BSTSmallArrayHeapAllocator<N>>;

// ─── BSStaticArray<T> ────────────────────────────────────────────────────────

/// C++ `RE::BSStaticArray<T>`.
/// A non-owning view over a contiguous block of elements.
///
/// Layout: `{ _data: *mut T, _size: u32, _pad: u32 }` — 0x10 bytes.
#[repr(C)]
pub struct BSStaticArray<T> {
    _data: *mut T,    // 0x00
    _size: u32,       // 0x08
    _pad0c: u32,      // 0x0C — padding for alignment
}

// Size assertion: ptr(8) + u32(4) + pad(4) = 0x10
const _: () = assert!(core::mem::size_of::<BSStaticArray<u8>>() == 0x10);

impl<T> BSStaticArray<T> {
    /// Number of elements.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self._size
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self._size == 0
    }

    /// Raw pointer to the first element.
    #[inline(always)]
    pub fn data(&self) -> *const T {
        self._data
    }

    /// Mutable raw pointer to the first element.
    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut T {
        self._data
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid for `len()` elements.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            core::slice::from_raw_parts(self._data, self._size as usize)
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        if self.is_empty() {
            &mut []
        } else {
            core::slice::from_raw_parts_mut(self._data, self._size as usize)
        }
    }
}

// ─── BSTSmallSharedArray<T> ──────────────────────────────────────────────────

/// C++ `RE::BSTSmallSharedArray<T>`.
///
/// An array optimized for 0 or 1 elements (stored inline) that falls back to
/// heap allocation for 2+ elements.
///
/// When `size > 1`, `_data.heap` is a pointer to a heap-allocated array.
/// When `size <= 1`, `_data.local` stores the single element inline.
///
/// # Safety
/// This type uses a union internally. Access methods are `unsafe` because
/// the caller must know whether the data is in inline or heap mode.
#[repr(C)]
pub struct BSTSmallSharedArray<T> {
    _size: u32,                          // 0x00
    _pad04: u32,                         // 0x04
    _data: BSTSmallSharedArrayData<T>,   // 0x08
}

/// Internal union for `BSTSmallSharedArray`.
#[repr(C)]
pub union BSTSmallSharedArrayData<T> {
    pub heap: *mut T,
    pub local: ManuallyDrop<MaybeUninit<T>>,
}

impl<T> BSTSmallSharedArray<T> {
    /// Number of elements.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self._size
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self._size == 0
    }

    /// Returns a pointer to the data.
    /// If `size > 1`, returns the heap pointer.
    /// If `size <= 1`, returns the address of the inline local storage.
    ///
    /// # Safety
    /// Must only be called when the array is non-empty.
    #[inline]
    pub unsafe fn data(&self) -> *const T {
        if self._size > 1 {
            self._data.heap
        } else {
            (*self._data.local).as_ptr()
        }
    }

    /// Returns a mutable pointer to the data.
    ///
    /// # Safety
    /// Must only be called when the array is non-empty.
    #[inline]
    pub unsafe fn data_mut(&mut self) -> *mut T {
        if self._size > 1 {
            self._data.heap
        } else {
            (*self._data.local).as_mut_ptr()
        }
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the data is properly initialized.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            core::slice::from_raw_parts(self.data(), self._size as usize)
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        if self.is_empty() {
            &mut []
        } else {
            core::slice::from_raw_parts_mut(self.data_mut(), self._size as usize)
        }
    }
}
