use core::marker::PhantomData;
use core::ptr;

/// C++ `RE::GListNode<T>`
#[repr(C)]
pub struct GListNode<T> {
    pub prev: *mut GListNode<T>, // 00
    pub next: *mut GListNode<T>, // 08
}

const _: () = assert!(core::mem::size_of::<GListNode<*mut core::ffi::c_void>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GListNode<*mut core::ffi::c_void>, prev) == 0x0);
const _: () = assert!(core::mem::offset_of!(GListNode<*mut core::ffi::c_void>, next) == 0x8);

impl<T> GListNode<T> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub const fn prev_node(&self) -> *mut GListNode<T> {
        self.prev
    }

    #[inline(always)]
    pub const fn next_node(&self) -> *mut GListNode<T> {
        self.next
    }

    /// # Safety
    /// `self.prev` and `self.next` must point to valid adjacent nodes in the
    /// same intrusive list.
    #[inline(always)]
    pub unsafe fn remove(&mut self) {
        unsafe {
            (*self.prev).next = self.next;
            (*self.next).prev = self.prev;
        }
    }
}

impl<T> Default for GListNode<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

pub trait GListNodeExt<T>: AsRef<GListNode<T>> + AsMut<GListNode<T>> {
    #[inline(always)]
    fn prev_node(&self) -> *mut GListNode<T> {
        self.as_ref().prev_node()
    }

    #[inline(always)]
    fn next_node(&self) -> *mut GListNode<T> {
        self.as_ref().next_node()
    }

    /// # Safety
    /// The node must currently be linked into a valid intrusive list.
    #[inline(always)]
    unsafe fn remove_node(&mut self) {
        unsafe { self.as_mut().remove() }
    }
}

impl<T, U> GListNodeExt<T> for U where U: AsRef<GListNode<T>> + AsMut<GListNode<T>> {}

/// C++ `RE::GList<T>`
#[repr(C)]
pub struct GList<T> {
    pub root: GListNode<T>, // 00
}

const _: () = assert!(core::mem::size_of::<GList<*mut core::ffi::c_void>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GList<*mut core::ffi::c_void>, root) == 0x0);

impl<T> GList<T> {
    #[inline(always)]
    pub fn new() -> Self {
        let mut list = Self {
            root: GListNode::new(),
        };
        let root = core::ptr::from_mut(&mut list.root);
        list.root.next = root;
        list.root.prev = root;
        list
    }

    #[inline(always)]
    pub fn front_node(&self) -> *mut GListNode<T> {
        self.root.next
    }

    #[inline(always)]
    pub fn back_node(&self) -> *mut GListNode<T> {
        self.root.prev
    }

    #[inline(always)]
    pub fn begin(&self) -> GListIterator<T> {
        let end = core::ptr::from_ref(&self.root).cast_mut();
        GListIterator {
            cur: self.root.next,
            end,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn end_node(&self) -> *mut GListNode<T> {
        core::ptr::from_ref(&self.root).cast_mut()
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.root.next == self.end_node()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.begin().count()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        let root = core::ptr::from_mut(&mut self.root);
        self.root.next = root;
        self.root.prev = root;
    }

    // TODO: CommonLib's `GList<T>` casts between `T*` and `GListNode<T>*`
    // through intrusive inheritance. Rust cannot express a universal
    // source-backed `T <-> GListNode<T>` conversion for arbitrary `T`, because
    // generic code here has no way to recover the enclosing `T*` from a node
    // pointer without a proven per-type base-subobject contract. Keep the
    // helper surface node-based until a concrete consumer translates that
    // conversion honestly.
    ///
    /// # Safety
    /// `pos` must be a valid node inside this list and `node` must be a valid
    /// intrusive node not currently linked in another list.
    #[inline(always)]
    pub unsafe fn insert_before_node(
        &mut self,
        pos: *mut GListNode<T>,
        node: *mut GListNode<T>,
    ) -> *mut GListNode<T> {
        unsafe {
            (*node).prev = (*pos).prev;
            (*(*pos).prev).next = node;
            (*node).next = pos;
            (*pos).prev = node;
        }
        node
    }

    /// # Safety
    /// `pos` must be a valid node inside this list and not the sentinel root.
    #[inline(always)]
    pub unsafe fn erase_node(&mut self, pos: *mut GListNode<T>) -> *mut GListNode<T> {
        unsafe {
            (*(*pos).prev).next = (*pos).next;
            (*(*pos).next).prev = (*pos).prev;
            (*pos).next
        }
    }

    /// # Safety
    /// `node` must be a valid intrusive node not currently linked in another
    /// list.
    #[inline(always)]
    pub unsafe fn push_back_node(&mut self, node: *mut GListNode<T>) {
        let root = core::ptr::from_mut(&mut self.root);
        unsafe {
            self.insert_before_node(root, node);
        }
    }

    /// # Safety
    /// The list must be non-empty.
    #[inline(always)]
    pub unsafe fn pop_back_node(&mut self) {
        unsafe {
            self.root.prev = (*self.root.prev).prev;
            (*self.root.prev).next = core::ptr::from_mut(&mut self.root);
        }
    }

    /// # Safety
    /// `node` must be a valid intrusive node not currently linked in another
    /// list.
    #[inline(always)]
    pub unsafe fn push_front_node(&mut self, node: *mut GListNode<T>) {
        let pos = self.root.next;
        unsafe {
            self.insert_before_node(pos, node);
        }
    }

    /// # Safety
    /// The list must be non-empty.
    #[inline(always)]
    pub unsafe fn pop_front_node(&mut self) {
        unsafe {
            self.root.next = (*self.root.next).next;
            (*self.root.next).prev = core::ptr::from_mut(&mut self.root);
        }
    }

    /// # Safety
    /// Every node in `other` must be valid for relinking into `self`.
    #[inline(always)]
    pub unsafe fn merge_nodes(&mut self, other: &mut Self) {
        while !other.empty() {
            let node = other.root.next;
            unsafe {
                other.erase_node(node);
                self.push_front_node(node);
            }
        }
    }
}

impl<T> Default for GList<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

pub struct GListIterator<T> {
    cur: *mut GListNode<T>,
    end: *mut GListNode<T>,
    _marker: PhantomData<*mut T>,
}

impl<T> Iterator for GListIterator<T> {
    type Item = *mut GListNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            let node = self.cur;
            unsafe {
                self.cur = (*node).next;
            }
            Some(node)
        }
    }
}
