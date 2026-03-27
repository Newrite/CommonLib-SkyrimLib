use core_util::inherit;

use crate::re::{
    BSIntrusiveRefCounted, BSTAutoPointer, BSTSmallArray, BSTSmartPointer,
    BSTSmartPointerIntrusiveRefCountable, CodeTasklet, IMemoryPagePolicy, IProfilePolicy,
    IStackCallbackFunctor, MemoryPage, StackFrame, VMStackID, Variable,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackState {
    Running = 0,
    Finished = 1,
    WaitingOnMemory = 2,
    WaitingOnLatentFunction = 3,
    WaitingOnOtherStackForCall = 4,
    WaitingOnOtherStackForReturn = 5,
    WaitingOnOtherStackForReturnNoPop = 6,
    RetryReturnNoPop = 7,
    RetryCall = 8,
}

core_util::impl_enumset_type!(StackState => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FreezeState {
    Unfrozen = 0,
    Freezing = 1,
    Frozen = 2,
}

core_util::impl_enumset_type!(FreezeState => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackType {
    Normal = 0,
    PropertyInitialize = 1,
    Initialize = 2,
}

core_util::impl_enumset_type!(StackType => u32);

#[repr(C)]
pub struct MemoryPageData {
    pub page: BSTAutoPointer<MemoryPage>, // 00
    pub available_memory_in_bytes: u32,   // 08
    pub pad0c: u32,                       // 0C
}

const _: () = assert!(core::mem::size_of::<MemoryPageData>() == 0x10);

#[repr(C)]
pub struct Stack {
    pub base: BSIntrusiveRefCounted,                        // 00
    pub pad04: u32,                                         // 04
    pub policy: *mut IMemoryPagePolicy,                     // 08
    pub profile_policy: *mut IProfilePolicy,                // 10
    pub pages: BSTSmallArray<MemoryPageData, 0x30>,         // 18
    pub frames: u32,                                        // 58
    pub pad5c: u32,                                         // 5C
    pub top: *mut StackFrame,                               // 60
    pub state: core_util::EnumSet<StackState, u32>,         // 68
    pub freeze_state: core_util::EnumSet<FreezeState, u32>, // 6C
    pub return_value: Variable,                             // 70
    pub stack_id: VMStackID,                                // 80
    pub stack_type: core_util::EnumSet<StackType, u32>,     // 84
    pub owning_tasklet: BSTSmartPointer<CodeTasklet>,       // 88
    pub callback: BSTSmartPointer<IStackCallbackFunctor>,   // 90
    pub next_stack: BSTSmartPointer<Stack>,                 // 98
}

const _: () = assert!(core::mem::size_of::<Stack>() == 0xA0);

inherit!(Stack : BSIntrusiveRefCounted);

impl BSTSmartPointerIntrusiveRefCountable for Stack {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let this = (self as *const Self).cast_mut();
        unsafe {
            (*this).dtor();
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

impl Stack {
    #[inline(always)]
    pub fn get_page_for_frame(&self, frame: *const StackFrame) -> u32 {
        let pages = unsafe { self.pages.as_slice() };
        for (i, pair) in pages.iter().enumerate() {
            if !pair.page.is_null() && unsafe { (*pair.page.get()).is_in_range(frame.cast()) } {
                return i as u32;
            }
        }
        u32::MAX
    }

    #[inline(always)]
    pub fn get_stack_frame_variable(
        &self,
        frame: *const StackFrame,
        index: u32,
        page_hint: u32,
    ) -> &mut Variable {
        let func = unsafe {
            crate::relocation::Relocation::<
                extern "C" fn(*const Self, *const StackFrame, u32, u32) -> *mut Variable,
            >::new(crate::relocation::RelocationID::new(97746, 104484))
            .get()
        };
        unsafe { &mut *func(self, frame, index, page_hint) }
    }

    crate::relocation_func! {
        pub fn dtor(&mut self) => crate::relocation::RelocationID::new(97742, 104480)
    }
}
