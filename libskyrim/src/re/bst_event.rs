use core::marker::PhantomData;
use core::ptr;

use crate::re::bs_atomic::{BSSpinLock, BSSpinLockGuard};
use crate::re::bst_array::{BSTArray, BSTArrayHeapAllocator};

/// C++ `RE::BSEventNotifyControl`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BSEventNotifyControl {
    Continue = 0,
    Stop = 1,
}

impl BSEventNotifyControl {
    #[inline(always)]
    pub const fn is_continue(self) -> bool {
        matches!(self, Self::Continue)
    }

    #[inline(always)]
    pub const fn is_stop(self) -> bool {
        matches!(self, Self::Stop)
    }
}

/// C++ `RE::BSTEventSink<T>`
#[repr(C)]
pub struct BSTEventSink<T> {
    pub vtable: *const usize, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<BSTEventSink<()>>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSTEventSink<()>, vtable) == 0x0);

impl<T> BSTEventSink<T> {
    pub const PROCESS_EVENT: usize = 0x1;

    /// # Safety
    /// `self` must point to a valid engine object whose vtable slot `01`
    /// matches `BSTEventSink<T>::ProcessEvent`.
    #[inline(always)]
    pub unsafe fn process_event(
        &mut self,
        event: *const T,
        event_source: *mut BSTEventSource<T>,
    ) -> BSEventNotifyControl {
        let func: extern "C" fn(
            *mut Self,
            *const T,
            *mut BSTEventSource<T>,
        ) -> BSEventNotifyControl = unsafe {
            crate::relocation::virtual_function(self as *const Self, Self::PROCESS_EVENT)
        };
        func(self, event, event_source)
    }
}

/// C++ `RE::BSTEventSource<T>`
#[repr(C)]
pub struct BSTEventSource<T> {
    pub sinks: BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>, // 00
    pub pending_registers: BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>, // 18
    pub pending_unregisters: BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>, // 30
    pub lock: BSSpinLock,                                             // 48
    pub notifying: bool,                                              // 50
    pub pad51: u8,                                                    // 51
    pub pad52: u16,                                                   // 52
    pub pad54: u32,                                                   // 54
}

const _: () = assert!(core::mem::size_of::<BSTEventSource<()>>() == 0x58);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, sinks) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, pending_registers) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, pending_unregisters) == 0x30);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, lock) == 0x48);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, notifying) == 0x50);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, pad51) == 0x51);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, pad52) == 0x52);
const _: () = assert!(core::mem::offset_of!(BSTEventSource<()>, pad54) == 0x54);

impl<T> Default for BSTEventSource<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BSTEventSource<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            sinks: BSTArray::new(),
            pending_registers: BSTArray::new(),
            pending_unregisters: BSTArray::new(),
            lock: BSSpinLock::new(),
            notifying: false,
            pad51: 0,
            pad52: 0,
            pad54: 0,
        }
    }

    /// # Safety
    /// `event_sink` must be a valid `BSTEventSink<T>` pointer for every future
    /// dispatch that may reach this source.
    pub unsafe fn add_event_sink(&mut self, event_sink: *mut BSTEventSink<T>) {
        if event_sink.is_null() {
            return;
        }

        let _guard = unsafe { BSSpinLockGuard::from_ptr(ptr::addr_of_mut!(self.lock)) };

        if self.notifying {
            if !contains_sink(&self.pending_registers, event_sink) {
                unsafe { self.pending_registers.push(event_sink) };
            }
        } else if !contains_sink(&self.sinks, event_sink) {
            unsafe { self.sinks.push(event_sink) };
        }

        remove_sink(&mut self.pending_unregisters, event_sink);
    }

    /// C++ template forwarding overload:
    /// `template <class SinkEvent> void AddEventSink(BSTEventSink<SinkEvent>*)`
    ///
    /// # Safety
    /// The caller must ensure the sink ABI is compatible with `BSTEventSink<T>`.
    #[inline(always)]
    pub unsafe fn add_event_sink_unchecked<SinkEvent>(
        &mut self,
        sink: *mut BSTEventSink<SinkEvent>,
    ) {
        unsafe { self.add_event_sink(sink.cast()) };
    }

    /// # Safety
    /// `event_sink` must be a valid `BSTEventSink<T>` pointer for every future
    /// dispatch that may reach this source.
    pub unsafe fn prepend_event_sink(&mut self, event_sink: *mut BSTEventSink<T>) {
        if event_sink.is_null() {
            return;
        }

        let _guard = unsafe { BSSpinLockGuard::from_ptr(ptr::addr_of_mut!(self.lock)) };

        if self.notifying {
            if !contains_sink(&self.pending_registers, event_sink) {
                unsafe { push_front_sink(&mut self.pending_registers, event_sink) };
            }
        } else if !contains_sink(&self.sinks, event_sink) {
            unsafe { push_front_sink(&mut self.sinks, event_sink) };
        }

        remove_sink(&mut self.pending_unregisters, event_sink);
    }

    /// C++ template forwarding overload:
    /// `template <class SinkEvent> void PrependEventSink(BSTEventSink<SinkEvent>*)`
    ///
    /// # Safety
    /// The caller must ensure the sink ABI is compatible with `BSTEventSink<T>`.
    #[inline(always)]
    pub unsafe fn prepend_event_sink_unchecked<SinkEvent>(
        &mut self,
        sink: *mut BSTEventSink<SinkEvent>,
    ) {
        unsafe { self.prepend_event_sink(sink.cast()) };
    }

    /// # Safety
    /// `event_sink` must be a pointer previously registered with this source or
    /// a compatible engine sink pointer scheduled for removal.
    pub unsafe fn remove_event_sink(&mut self, event_sink: *mut BSTEventSink<T>) {
        if event_sink.is_null() {
            return;
        }

        let _guard = unsafe { BSSpinLockGuard::from_ptr(ptr::addr_of_mut!(self.lock)) };

        if self.notifying {
            if !contains_sink(&self.pending_unregisters, event_sink) {
                unsafe { self.pending_unregisters.push(event_sink) };
            }
        } else {
            remove_sink(&mut self.sinks, event_sink);
        }

        remove_sink(&mut self.pending_registers, event_sink);
    }

    /// # Safety
    /// `event` must point to a valid `T` for the duration of the call, and all
    /// currently registered sink pointers must remain valid.
    pub unsafe fn send_event(&mut self, event: *const T) {
        let _guard = unsafe { BSSpinLockGuard::from_ptr(ptr::addr_of_mut!(self.lock)) };

        let was_notifying = self.notifying;
        self.notifying = true;

        if !was_notifying && !self.pending_registers.is_empty() {
            let pending = unsafe { self.pending_registers.as_slice() };
            for &to_add in pending {
                if !contains_sink(&self.sinks, to_add) {
                    unsafe { self.sinks.push(to_add) };
                }
            }
            unsafe { self.pending_registers.clear() };
        }

        let this = self as *mut Self;
        let sinks_ptr = self.sinks.data();
        let sinks_len = self.sinks.len() as usize;
        for index in 0..sinks_len {
            let sink = unsafe { *sinks_ptr.add(index) };
            if contains_sink(&self.pending_unregisters, sink) {
                continue;
            }

            let notify = unsafe { (*sink).process_event(event, this) };
            if notify.is_stop() {
                break;
            }
        }

        self.notifying = was_notifying;
        if !was_notifying && !self.pending_unregisters.is_empty() {
            let pending = unsafe { self.pending_unregisters.as_slice() };
            for &to_remove in pending {
                remove_sink(&mut self.sinks, to_remove);
            }
            unsafe { self.pending_unregisters.clear() };
        }
    }

    /// C++ `BSTEventSource::operator()(const Event*)`
    ///
    /// # Safety
    /// Same requirements as `send_event`.
    #[inline(always)]
    pub unsafe fn invoke(&mut self, event: *const T) {
        unsafe { self.send_event(event) };
    }
}

#[inline(always)]
fn contains_sink<T>(
    array: &BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>,
    sink: *mut BSTEventSink<T>,
) -> bool {
    unsafe { array.as_slice().contains(&sink) }
}

unsafe fn push_front_sink<T>(
    array: &mut BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>,
    sink: *mut BSTEventSink<T>,
) {
    unsafe {
        array.push(sink);
        let len = array.len() as usize;
        let data = array.data_mut();
        if len > 1 {
            ptr::copy(data, data.add(1), len - 1);
        }
        ptr::write(data, sink);
    }
}

fn remove_sink<T>(
    array: &mut BSTArray<*mut BSTEventSink<T>, BSTArrayHeapAllocator>,
    sink: *mut BSTEventSink<T>,
) -> bool {
    let index = unsafe {
        array
            .as_slice()
            .iter()
            .position(|&candidate| candidate == sink)
    };
    let Some(index) = index else {
        return false;
    };

    unsafe {
        let len = array.len() as usize;
        let data = array.data_mut();
        if index + 1 < len {
            ptr::copy(data.add(index + 1), data.add(index), len - index - 1);
        }
        let _ = array.pop();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_prepend_remove_keep_expected_order() {
        let mut source = BSTEventSource::<()>::new();
        let a = 0x1000usize as *mut BSTEventSink<()>;
        let b = 0x2000usize as *mut BSTEventSink<()>;
        let c = 0x3000usize as *mut BSTEventSink<()>;

        unsafe {
            source.add_event_sink(a);
            source.add_event_sink(b);
            source.prepend_event_sink(c);
        }

        let sinks = unsafe { source.sinks.as_slice() };
        assert_eq!(sinks, &[c, a, b]);

        unsafe {
            source.remove_event_sink(a);
        }

        let sinks = unsafe { source.sinks.as_slice() };
        assert_eq!(sinks, &[c, b]);
    }
}
