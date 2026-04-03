use core::ffi::c_char;
use core::mem::size_of;

use alloc::rc::Rc;
use core::cell::Cell;

use crate::sdk::core::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
use crate::skse::Message;

use super::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pair {
    left: u32,
    right: u32,
}

#[test]
fn message_ref_views_sender_and_payload() {
    let sender = c"SKSE";
    let payload = Pair { left: 1, right: 2 };
    let message = Message {
        sender: sender.as_ptr().cast::<c_char>(),
        msg_type: Message::SKSE_DATA_LOADED,
        data_len: size_of::<Pair>() as u32,
        data: (&payload as *const Pair).cast_mut().cast(),
    };

    let view = MessageRef::new(&message);
    assert_eq!(view.kind(), Some(MessageKind::DataLoaded));
    assert_eq!(view.sender(), Some("SKSE"));
    assert_eq!(view.data_as::<Pair>(), Some(&payload));
    assert_eq!(view.data_copy::<Pair>(), Some(payload));
    assert_eq!(view.typed::<Pair>().map(|typed| *typed), Some(payload));
}

#[test]
fn message_ref_rejects_misaligned_payloads() {
    let mut bytes = [0u8; 8];
    let message = Message {
        sender: core::ptr::null(),
        msg_type: Message::SKSE_POST_LOAD,
        data_len: 4,
        data: unsafe { bytes.as_mut_ptr().add(1) }.cast(),
    };

    let view = MessageRef::new(&message);
    assert_eq!(view.data_as::<u32>(), None);
}

#[test]
fn message_kinds_map_to_lifecycle_phases() {
    assert_eq!(
        MessageKind::PostLoad.plugin_phase(),
        Some(PluginLifecyclePhase::PostLoad)
    );
    assert_eq!(
        MessageKind::NewGame.game_lifecycle_phase(),
        Some(GameLifecyclePhase::NewGame)
    );
    assert_eq!(
        MessageKind::DataLoaded.lifecycle_phase(),
        Some(LifecyclePhase::Plugin(PluginLifecyclePhase::DataLoaded))
    );
    assert_eq!(
        MessageKind::DeleteGame.lifecycle_phase(),
        Some(LifecyclePhase::Game(GameLifecyclePhase::DeleteGame))
    );
}

#[test]
fn message_ref_exposes_lifecycle_phases() {
    let message = Message {
        sender: core::ptr::null(),
        msg_type: Message::SKSE_SAVE_GAME,
        data_len: 0,
        data: core::ptr::null_mut(),
    };

    let view = MessageRef::new(&message);
    assert_eq!(view.plugin_phase(), None);
    assert_eq!(
        view.game_lifecycle_phase(),
        Some(GameLifecyclePhase::SaveGame)
    );
    assert_eq!(
        view.lifecycle_phase(),
        Some(LifecyclePhase::Game(GameLifecyclePhase::SaveGame))
    );
}

#[test]
fn sender_filtered_helpers_only_fire_for_matching_sender() {
    let seen = Rc::new(Cell::new(0u32));
    let seen_ref = Rc::clone(&seen);
    let sender = c"SKSE";

    let callback = {
        let sender = sender;
        move |message: MessageRef<'_>| {
            if message.sender_matches(sender) {
                seen_ref.set(seen_ref.get() + 1);
            }
        }
    };

    let good = Message {
        sender: sender.as_ptr().cast::<c_char>(),
        msg_type: Message::SKSE_DATA_LOADED,
        data_len: 0,
        data: core::ptr::null_mut(),
    };
    let bad_sender = c"Other";
    let bad = Message {
        sender: bad_sender.as_ptr().cast::<c_char>(),
        msg_type: Message::SKSE_DATA_LOADED,
        data_len: 0,
        data: core::ptr::null_mut(),
    };

    callback(MessageRef::new(&good));
    assert_eq!(seen.get(), 1);
    if MessageRef::new(&bad).sender_matches(sender) {
        seen.set(seen.get() + 1);
    }
    assert_eq!(seen.get(), 1);
}

#[test]
fn typed_slice_view_wraps_payload_slice() {
    let payload = [1u32, 2, 3, 4];
    let message = Message {
        sender: core::ptr::null(),
        msg_type: Message::SKSE_POST_LOAD,
        data_len: (size_of::<u32>() * payload.len()) as u32,
        data: payload.as_ptr().cast_mut().cast(),
    };

    let view = MessageRef::new(&message);
    let typed = view
        .typed_slice::<u32>()
        .expect("slice payload should decode");
    assert_eq!(typed.message().kind(), Some(MessageKind::PostLoad));
    assert_eq!(typed.payload(), payload.as_slice());
}
