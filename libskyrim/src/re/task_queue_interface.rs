use crate::re::{
    Actor, ActorHandle, NiAVObject, NiNode, NiPoint3, SpellItem, TESObjectREFR, TESWeather,
};
use crate::relocation::RelocationID;

/// C++ `RE::TaskQueueInterface`
#[repr(C)]
pub struct TaskQueueInterface {
    pub pad0: u8, // 00
}

const _: () = assert!(core::mem::size_of::<TaskQueueInterface>() == 0x1);
const _: () = assert!(core::mem::offset_of!(TaskQueueInterface, pad0) == 0x00);

impl TaskQueueInterface {
    crate::relocation_variable! {
        fn singleton() -> *mut TaskQueueInterface => RelocationID::new(517228, 403759), is_ptr
    }

    crate::relocation_func! {
        pub fn should_use_task_queue() -> bool => RelocationID::new(38079, 39033)
    }

    crate::relocation_func! {
        pub fn queue_node_attach(
            &mut self,
            obj: *mut NiAVObject,
            root: *mut NiNode,
            arg3: bool,
            arg4: bool,
        ) => RelocationID::new(35922, 36897)
    }

    crate::relocation_func! {
        pub fn queue_node_detach(&mut self, obj: *mut NiAVObject) => RelocationID::new(35923, 36898)
    }

    crate::relocation_func! {
        pub fn queue_update_destructible_object(
            &mut self,
            refr: *mut TESObjectREFR,
            damage: f32,
            arg3: bool,
            cause: *mut TESObjectREFR,
        ) => RelocationID::new(35934, 36909)
    }

    crate::relocation_func! {
        pub fn queue_actor_knock_paralyze(&mut self, actor: *mut Actor) => RelocationID::new(35941, 36916)
    }

    crate::relocation_func! {
        pub fn queue_add_ripple(&mut self, scale: f32, pos: &NiPoint3) => RelocationID::new(35978, 36953)
    }

    crate::relocation_func! {
        pub fn queue_force_weather(
            &mut self,
            weather: *mut TESWeather,
            force_override: bool,
        ) => RelocationID::new(35991, 36966)
    }

    crate::relocation_func! {
        pub fn queue_update_ni_object(&mut self, obj: *mut NiAVObject) => RelocationID::new(35929, 36904)
    }

    crate::relocation_func! {
        pub fn queue_actor_disarm(
            &mut self,
            target: &mut ActorHandle,
            caster: &mut ActorHandle,
        ) => RelocationID::new(36010, 36985)
    }

    crate::relocation_func! {
        pub fn queue_remove_spell(
            &mut self,
            actor: &mut ActorHandle,
            spell_item: *mut SpellItem,
        ) => RelocationID::new(35987, 36962)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut TaskQueueInterface {
        Self::singleton()
    }
}
