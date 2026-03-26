use crate::re::{BSTEventSink, BSTEventSource, HudModeChangeEvent};

/// Source-backed but currently disputed `0x68..0x77` block inside
/// `RE::HudMoveChangeEventSource`.
///
/// The vendored CommonLibVR header simultaneously spells this range as
/// `std::uint64_t unk68; std::uint32_t unk6C; std::uint32_t unk70;
/// std::uint32_t unk74;`, which overlaps and therefore cannot be represented as
/// a valid C layout one-to-one. Keep the exact byte span honest until the VR
/// owner is RE-confirmed more precisely.
#[repr(C)]
pub struct HudMoveChangeEventSourceDisputedBlock {
    pub bytes: [u8; 0x10], // 68
}

const _: () = assert!(core::mem::size_of::<HudMoveChangeEventSourceDisputedBlock>() == 0x10);

/// VR-only C++ `RE::HudMoveChangeEventSource`.
///
/// CommonLibVR itself notes that the class name/usage still needs RE, but the
/// current header does provide a concrete owner layout over
/// `BSTEventSource<HudModeChangeEvent>`, so keep that source-backed surface
/// here instead of leaving a payload-only TODO in `hud_mode_change_event.rs`.
#[repr(C)]
pub struct HudMoveChangeEventSource {
    pub event_source: BSTEventSource<HudModeChangeEvent>, // 00
    pub unk58: u64,                                       // 58
    pub unk60: u64,                                       // 60
    pub unk68_through_unk74: HudMoveChangeEventSourceDisputedBlock, // 68
    pub unk78: *mut core::ffi::c_void,                    // 78 - SkyrimVR.exe+0x1EAAB34
    pub unk80: u64,                                       // 80
    pub unk88: *mut core::ffi::c_void, // 88 - allocated array, size 32, observed entry size 0x18
    pub unk90: [u64; 10],              // 90
    pub unknown_tail: [u8; 0x08],      // E0
}

const _: () = assert!(core::mem::size_of::<HudMoveChangeEventSource>() == 0xE8);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, event_source) == 0x00);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk58) == 0x58);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk60) == 0x60);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk68_through_unk74) == 0x68);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk78) == 0x78);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk80) == 0x80);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk88) == 0x88);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unk90) == 0x90);
const _: () = assert!(core::mem::offset_of!(HudMoveChangeEventSource, unknown_tail) == 0xE0);

core_util::inherit!(HudMoveChangeEventSource => BSTEventSource<HudModeChangeEvent>, event_source);

impl HudMoveChangeEventSource {
    // TODO: Replace `unk68_through_unk74` with the real named members once the
    // conflicting `HudModeChangeEvent.h` declarations for offsets `0x68..0x77`
    // are RE-confirmed and no longer overlap in the source.
    // TODO: Replace `unknown_tail` with the real `0xE0..0xE7` members once the
    // CommonLibVR field list is reconciled with its own `sizeof(...) == 0xE8`
    // assertion. The current header under-describes the final 8 bytes.

    #[inline(always)]
    pub fn event_source(&self) -> &BSTEventSource<HudModeChangeEvent> {
        &self.event_source
    }

    #[inline(always)]
    pub fn event_source_mut(&mut self) -> &mut BSTEventSource<HudModeChangeEvent> {
        &mut self.event_source
    }

    #[inline(always)]
    pub unsafe fn add_event_sink(&mut self, sink: *mut BSTEventSink<HudModeChangeEvent>) {
        unsafe { self.event_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_event_sink(&mut self, sink: *mut BSTEventSink<HudModeChangeEvent>) {
        unsafe { self.event_source.remove_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn send_event(&mut self, event: *const HudModeChangeEvent) {
        unsafe { self.event_source.send_event(event) }
    }

    // TODO: `HudMoveChangeEventSource::GetSingleton()` is declared in
    // `RE/H/HudModeChangeEvent.h`, but CommonLibVR provides no `.cpp` body,
    // `RELOCATION_ID(...)`, RTTI, VTABLE entry, or global relocation for it.
    // Add a real relocation-backed singleton accessor once the owner source is
    // confirmed instead of inventing a null-returning or guessed stub here.
}
