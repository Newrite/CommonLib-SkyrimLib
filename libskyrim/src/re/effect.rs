use crate::re::TESCondition;
use crate::re::{EffectSetting, EffectSettingDataFlag};
use crate::relocation::RelocationID;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct EffectItem {
    pub magnitude: f32, // 0x00
    pub area: u32,      // 0x04
    pub duration: u32,  // 0x08
}

const _: () = assert!(core::mem::size_of::<EffectItem>() == 0xC);

#[repr(C)]
#[derive(Default)]
pub struct Effect {
    pub effect_item: EffectItem,         // 0x00 - EFIT
    pub pad0c: u32,                      // 0x0C
    pub base_effect: *mut EffectSetting, // 0x10 - EFID
    pub cost: f32,                       // 0x18
    pub pad1c: u32,                      // 0x1C
    pub conditions: TESCondition,        // 0x20 - CTDA
}

const _: () = assert!(core::mem::size_of::<Effect>() == 0x28);
const _: () = assert!(core::mem::offset_of!(Effect, base_effect) == 0x10);
const _: () = assert!(core::mem::offset_of!(Effect, conditions) == 0x20);

impl Effect {
    #[inline]
    fn essentially_equal(a: f32, b: f32) -> bool {
        let epsilon = f32::EPSILON;
        (a - b).abs() <= a.abs().min(b.abs()) * epsilon
    }

    #[inline]
    fn base_effect_ref(&self) -> Option<&EffectSetting> {
        unsafe { self.base_effect.as_ref() }
    }

    pub fn copy_from(&mut self, other: *const Effect) {
        self.copy(other)
    }

    pub fn get_magnitude(&self) -> f32 {
        if self
            .base_effect_ref()
            .is_some_and(|base| base.data.flags.any(EffectSettingDataFlag::NoMagnitude))
        {
            0.0
        } else {
            self.effect_item.magnitude
        }
    }

    pub fn get_area(&self) -> u32 {
        if self
            .base_effect_ref()
            .is_some_and(|base| base.data.flags.any(EffectSettingDataFlag::NoArea))
        {
            0
        } else {
            self.effect_item.area
        }
    }

    pub fn get_duration(&self) -> u32 {
        if self
            .base_effect_ref()
            .is_some_and(|base| base.data.flags.any(EffectSettingDataFlag::NoDuration))
        {
            0
        } else {
            self.effect_item.duration
        }
    }

    pub fn is_hostile(&self) -> bool {
        self.base_effect_ref()
            .is_some_and(EffectSetting::is_hostile)
    }

    pub fn is_match(
        &self,
        base: *mut EffectSetting,
        mag: f32,
        area: u32,
        dur: u32,
        cost: f32,
    ) -> bool {
        self.base_effect == base
            && Self::essentially_equal(self.effect_item.magnitude, mag)
            && self.effect_item.area == area
            && self.effect_item.duration == dur
            && Self::essentially_equal(self.cost, cost)
    }

    crate::relocation_func! {
        fn copy(&mut self, other: *const Effect) => RelocationID::new(10909, 10997)
    }

    crate::relocation_func! {
        pub fn set_duration(&mut self, duration: i32) => RelocationID::new(10924, 11012)
    }

    crate::relocation_func! {
        pub fn set_magnitude(&mut self, magnitude: f32) => RelocationID::new(10920, 11008)
    }
}
