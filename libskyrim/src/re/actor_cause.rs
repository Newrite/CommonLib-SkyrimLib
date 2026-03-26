use core::sync::atomic::{AtomicI32, Ordering};

use crate::re::NiRef;
use crate::re::bs_pointer_handle::ActorHandle;
use crate::re::ni_point3::NiPoint3;

/// C++ `RE::ActorCause`
#[repr(C)]
pub struct ActorCause {
    pub actor: ActorHandle,   // 00
    pub origin: NiPoint3,     // 04
    pub actor_cause_id: u32,  // 10
    pub ref_count: AtomicI32, // 14
}

const _: () = assert!(core::mem::size_of::<ActorCause>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ActorCause, actor) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorCause, origin) == 0x04);
const _: () = assert!(core::mem::offset_of!(ActorCause, actor_cause_id) == 0x10);
const _: () = assert!(core::mem::offset_of!(ActorCause, ref_count) == 0x14);

impl Default for ActorCause {
    #[inline]
    fn default() -> Self {
        Self {
            actor: ActorHandle::new(),
            origin: NiPoint3::default(),
            actor_cause_id: 0,
            ref_count: AtomicI32::new(0),
        }
    }
}

impl ActorCause {
    #[inline]
    pub fn dec_ref_count(&self) -> i32 {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline]
    pub fn inc_ref_count(&self) -> i32 {
        self.ref_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    #[inline]
    pub fn get_ref_count(&self) -> i32 {
        self.ref_count.load(Ordering::SeqCst)
    }
}

impl NiRef for ActorCause {
    #[inline(always)]
    fn inc_ref(&self) {
        let _ = self.inc_ref_count();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        let _ = self.dec_ref_count();
    }
}
