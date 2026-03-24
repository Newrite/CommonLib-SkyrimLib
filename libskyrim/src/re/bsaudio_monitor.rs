/// C++ `RE::BSAudioMonitor::Request`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BSAudioMonitorRequest {
    pub monitor: u16,    // 00
    pub send_level: u16, // 02
}

const _: () = assert!(core::mem::size_of::<BSAudioMonitorRequest>() == 0x4);
const _: () = assert!(core::mem::offset_of!(BSAudioMonitorRequest, monitor) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSAudioMonitorRequest, send_level) == 0x02);

impl BSAudioMonitorRequest {
    #[inline(always)]
    pub const fn new(monitor: u16, send_level: u16) -> Self {
        Self {
            monitor,
            send_level,
        }
    }

    #[inline(always)]
    pub const fn qid(self) -> u32 {
        self.monitor as u32
    }

    #[inline(always)]
    pub const fn q_send_level(self) -> u16 {
        self.send_level
    }
}

/// C++ `RE::BSAudioMonitor::Receiver`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSAudioMonitorReceiver {
    pub amplitude: *const f32, // 00
}

const _: () = assert!(core::mem::size_of::<BSAudioMonitorReceiver>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSAudioMonitorReceiver, amplitude) == 0x00);

impl BSAudioMonitorReceiver {
    #[inline(always)]
    pub fn new(amplitude: &f32) -> Self {
        Self {
            amplitude: core::ptr::from_ref(amplitude),
        }
    }

    #[inline(always)]
    pub fn q_amplitude(self) -> f32 {
        debug_assert!(!self.amplitude.is_null());
        unsafe { *self.amplitude }
    }
}
