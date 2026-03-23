use crate::re::EffectSetting;
use crate::re::TESCondition;
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
    // EffectSetting::data.flags is the first field of EffectSettingData, and
    // EffectSetting::data begins at offset 0x68.
    const EFFECT_SETTING_FLAGS_OFFSET: usize = 0x68;
    const EFFECT_SETTING_FLAG_HOSTILE: u32 = 1 << 0;
    const EFFECT_SETTING_FLAG_NO_DURATION: u32 = 1 << 9;
    const EFFECT_SETTING_FLAG_NO_MAGNITUDE: u32 = 1 << 10;
    const EFFECT_SETTING_FLAG_NO_AREA: u32 = 1 << 11;

    #[inline]
    fn essentially_equal(a: f32, b: f32) -> bool {
        let epsilon = f32::EPSILON;
        (a - b).abs() <= a.abs().min(b.abs()) * epsilon
    }

    #[inline]
    fn effect_setting_flags(&self) -> u32 {
        unsafe {
            *((self.base_effect as *const u8).add(Self::EFFECT_SETTING_FLAGS_OFFSET) as *const u32)
        }
    }

    pub fn copy_from(&mut self, other: *const Effect) {
        self.copy(other)
    }

    pub fn get_magnitude(&self) -> f32 {
        if self.effect_setting_flags() & Self::EFFECT_SETTING_FLAG_NO_MAGNITUDE != 0 {
            0.0
        } else {
            self.effect_item.magnitude
        }
    }

    pub fn get_area(&self) -> u32 {
        if self.effect_setting_flags() & Self::EFFECT_SETTING_FLAG_NO_AREA != 0 {
            0
        } else {
            self.effect_item.area
        }
    }

    pub fn get_duration(&self) -> u32 {
        if self.effect_setting_flags() & Self::EFFECT_SETTING_FLAG_NO_DURATION != 0 {
            0
        } else {
            self.effect_item.duration
        }
    }

    pub fn is_hostile(&self) -> bool {
        self.effect_setting_flags() & Self::EFFECT_SETTING_FLAG_HOSTILE != 0
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
