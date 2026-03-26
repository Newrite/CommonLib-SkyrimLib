use crate::re::ActorHandle;

/// C++ `RE::BGSActorEvent`
#[repr(C)]
pub struct BGSActorEvent {
    pub actor: ActorHandle, // 00
}

const _: () = assert!(core::mem::size_of::<BGSActorEvent>() == 0x04);
const _: () = assert!(core::mem::offset_of!(BGSActorEvent, actor) == 0x00);
