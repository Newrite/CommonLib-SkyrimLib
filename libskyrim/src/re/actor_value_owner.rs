use crate::offsets::offsets_rtti::RTTI_ActorValueOwner;
use crate::offsets::offsets_vtable::VTABLE_ActorValueOwner;
use crate::re::{ActorValue, ActorValueModifier};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
struct ActorValueInfoFlagsView {
    _pad0: [u8; 0x60],
    flags: u32,
}

impl ActorValueInfoFlagsView {
    const FLAG_INVERTED: u32 = 1 << 9;

    #[inline(always)]
    fn is_inverted(&self) -> bool {
        self.flags & Self::FLAG_INVERTED != 0
    }
}

crate::relocation_func! {
    fn get_actor_value_info_impl(actor_value: ActorValue) -> *mut ActorValueInfoFlagsView => RelocationID::new(26569, 27202)
}

/// C++ `RE::ActorValueOwner`
#[repr(C)]
pub struct ActorValueOwner {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<ActorValueOwner>() == 0x8);
const _: () = assert!(core::mem::offset_of!(ActorValueOwner, vtable) == 0x00);

impl RttiType for ActorValueOwner {
    const RTTI: VariantID = RTTI_ActorValueOwner;
}

impl AsRef<ActorValueOwner> for ActorValueOwner {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ActorValueOwner> for ActorValueOwner {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl ActorValueOwner {
    pub const RTTI: VariantID = RTTI_ActorValueOwner;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorValueOwner;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_GET_ACTOR_VALUE: usize = 0x01;
        pub fn get_actor_value(actor_value: ActorValue) -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_PERMANENT_ACTOR_VALUE: usize = 0x02;
        pub fn get_permanent_actor_value(actor_value: ActorValue) -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_BASE_ACTOR_VALUE: usize = 0x03;
        pub fn get_base_actor_value(actor_value: ActorValue) -> f32
    }

    virtual_method! {
        pub const VFUNC_SET_BASE_ACTOR_VALUE: usize = 0x04;
        pub fn set_base_actor_value(&mut self, actor_value: ActorValue, value: f32)
    }

    virtual_method! {
        pub const VFUNC_MOD_BASE_ACTOR_VALUE: usize = 0x05;
        pub fn mod_base_actor_value(&mut self, actor_value: ActorValue, value: f32)
    }

    virtual_method! {
        pub const VFUNC_MOD_ACTOR_VALUE: usize = 0x06;
        pub fn mod_actor_value(
            &mut self,
            modifier: ActorValueModifier,
            actor_value: ActorValue,
            value: f32
        )
    }

    virtual_method! {
        pub const VFUNC_SET_ACTOR_VALUE: usize = 0x07;
        pub fn set_actor_value(&mut self, actor_value: ActorValue, value: f32)
    }

    virtual_method! {
        pub const VFUNC_GET_IS_PLAYER_OWNER: usize = 0x08;
        pub fn get_is_player_owner() -> bool
    }

    crate::relocation_func! {
        pub fn get_armor_rating_skill_multiplier(&self, skill_level: f32) -> f32 => RelocationID::new(25858, 26424)
    }

    crate::relocation_func! {
        pub fn get_clamped_actor_value(&self, actor_value: ActorValue) -> f32 => RelocationID::new(26616, 27284)
    }

    #[inline(always)]
    pub fn damage_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        let actor_value_info = get_actor_value_info_impl(actor_value);
        let damage = if unsafe { actor_value_info.as_ref() }.is_some_and(|info| info.is_inverted())
        {
            value.abs()
        } else {
            -value.abs()
        };
        self.mod_actor_value(ActorValueModifier::Damage, actor_value, damage);
    }

    #[inline(always)]
    pub fn restore_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        let actor_value_info = get_actor_value_info_impl(actor_value);
        let damage = if unsafe { actor_value_info.as_ref() }.is_some_and(|info| info.is_inverted())
        {
            -value.abs()
        } else {
            value.abs()
        };
        self.mod_actor_value(ActorValueModifier::Damage, actor_value, damage);
    }
}

pub trait ActorValueOwnerExt {
    fn get_actor_value(&self, actor_value: ActorValue) -> f32;
    fn get_permanent_actor_value(&self, actor_value: ActorValue) -> f32;
    fn get_base_actor_value(&self, actor_value: ActorValue) -> f32;
    fn set_base_actor_value(&mut self, actor_value: ActorValue, value: f32);
    fn mod_base_actor_value(&mut self, actor_value: ActorValue, value: f32);
    fn mod_actor_value(
        &mut self,
        modifier: ActorValueModifier,
        actor_value: ActorValue,
        value: f32,
    );
    fn set_actor_value(&mut self, actor_value: ActorValue, value: f32);
    fn get_is_player_owner(&self) -> bool;
    fn get_armor_rating_skill_multiplier(&self, skill_level: f32) -> f32;
    fn get_clamped_actor_value(&self, actor_value: ActorValue) -> f32;
    fn damage_actor_value(&mut self, actor_value: ActorValue, value: f32);
    fn restore_actor_value(&mut self, actor_value: ActorValue, value: f32);
}

impl<T: AsRef<ActorValueOwner> + AsMut<ActorValueOwner>> ActorValueOwnerExt for T {
    #[inline(always)]
    fn get_actor_value(&self, actor_value: ActorValue) -> f32 {
        self.as_ref().get_actor_value(actor_value)
    }

    #[inline(always)]
    fn get_permanent_actor_value(&self, actor_value: ActorValue) -> f32 {
        self.as_ref().get_permanent_actor_value(actor_value)
    }

    #[inline(always)]
    fn get_base_actor_value(&self, actor_value: ActorValue) -> f32 {
        self.as_ref().get_base_actor_value(actor_value)
    }

    #[inline(always)]
    fn set_base_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        ActorValueOwner::set_base_actor_value(self.as_mut(), actor_value, value)
    }

    #[inline(always)]
    fn mod_base_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        ActorValueOwner::mod_base_actor_value(self.as_mut(), actor_value, value)
    }

    #[inline(always)]
    fn mod_actor_value(
        &mut self,
        modifier: ActorValueModifier,
        actor_value: ActorValue,
        value: f32,
    ) {
        ActorValueOwner::mod_actor_value(self.as_mut(), modifier, actor_value, value)
    }

    #[inline(always)]
    fn set_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        ActorValueOwner::set_actor_value(self.as_mut(), actor_value, value)
    }

    #[inline(always)]
    fn get_is_player_owner(&self) -> bool {
        self.as_ref().get_is_player_owner()
    }

    #[inline(always)]
    fn get_armor_rating_skill_multiplier(&self, skill_level: f32) -> f32 {
        self.as_ref().get_armor_rating_skill_multiplier(skill_level)
    }

    #[inline(always)]
    fn get_clamped_actor_value(&self, actor_value: ActorValue) -> f32 {
        self.as_ref().get_clamped_actor_value(actor_value)
    }

    #[inline(always)]
    fn damage_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        ActorValueOwner::damage_actor_value(self.as_mut(), actor_value, value)
    }

    #[inline(always)]
    fn restore_actor_value(&mut self, actor_value: ActorValue, value: f32) {
        ActorValueOwner::restore_actor_value(self.as_mut(), actor_value, value)
    }
}
