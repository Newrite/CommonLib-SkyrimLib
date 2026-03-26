use core::ffi::c_char;
use core::ptr;

use crate::re::{
    Actor, ActorValue, GameSettingCollection, IMessageBoxCallback, INIPrefSettingCollection,
    INISettingCollection, InventoryEntryData, NiPoint3, NiPointer, RefHandle, Setting,
    TESObjectREFR,
};
use crate::relocation::RelocationID;

crate::relocation_func! {
    fn lookup_reference_by_handle_actor_impl(
        handle: &RefHandle,
        refr_out: &mut NiPointer<Actor>
    ) -> bool => RelocationID::new(12204, 12332)
}

crate::relocation_func! {
    fn lookup_reference_by_handle_refr_impl(
        handle: &RefHandle,
        refr_out: &mut NiPointer<TESObjectREFR>
    ) -> bool => RelocationID::new(12204, 12332)
}

crate::relocation_func! {
    pub fn create_message(
        message: *const c_char,
        callback: *mut IMessageBoxCallback,
        arg3: u32,
        arg4: u32,
        arg5: u32,
        button_text: *const c_char,
        secondary_button_text: *const c_char
    ) => RelocationID::new(51420, 52269)
}

crate::relocation_func! {
    pub fn create_ref_handle(handle_out: &mut RefHandle, ref_to: *mut TESObjectREFR)
        => RelocationID::new(12193, 12326)
}

crate::relocation_func! {
    pub fn get_armor_final_rating(
        armor_entry_data: *mut InventoryEntryData,
        armor_perks: f32,
        skill_multiplier: f32
    ) -> f32 => RelocationID::new(15779, 16017)
}

crate::relocation_variable! {
    fn duration_of_application_runtime() -> &'static u32 => RelocationID::new(523662, 410201)
}

crate::relocation_variable! {
    fn seconds_since_last_frame_raw() -> &'static f32 => RelocationID::new(523660, 410199)
}

crate::relocation_func! {
    pub fn play_sound(editor_id: *const c_char) => RelocationID::new(52054, 52939)
}

crate::relocation_func! {
    pub fn random_bool_chance(chance: f32) -> bool => RelocationID::new(26009, 0)
}

crate::relocation_func! {
    pub fn shake_camera(strength: f32, position: &NiPoint3, duration: f32)
        => RelocationID::new(32275, 33012)
}

crate::relocation_func! {
    pub fn flash_hud_meter(actor_value: ActorValue) => RelocationID::new(51907, 52845)
}

#[inline(always)]
pub fn debug_message_box(message: *const c_char) {
    let setting_collection = GameSettingCollection::get_singleton();
    assert!(
        !setting_collection.is_null(),
        "GameSettingCollection::GetSingleton returned null"
    );

    let ok_setting = unsafe { (*setting_collection).get_setting_str("sOk") };
    assert!(
        !ok_setting.is_null(),
        "GameSettingCollection is missing sOk"
    );

    let ok_button_text = unsafe { (*ok_setting).get_string() };
    create_message(
        message,
        ptr::null_mut(),
        0,
        4,
        10,
        ok_button_text,
        ptr::null(),
    );
}

#[inline(always)]
pub fn get_duration_of_application_run_time() -> u32 {
    *duration_of_application_runtime()
}

#[inline(always)]
pub fn get_ini_setting(name: *const c_char) -> *mut Setting {
    let ini_prefs = INIPrefSettingCollection::get_singleton();
    let setting = if ini_prefs.is_null() {
        ptr::null_mut()
    } else {
        unsafe { (*ini_prefs).base.get_setting(core_util::ptr_to_str(name)) }
    };

    if !setting.is_null() {
        return setting;
    }

    let ini = INISettingCollection::get_singleton();
    if ini.is_null() {
        ptr::null_mut()
    } else {
        unsafe { (*ini).get_setting(core_util::ptr_to_str(name)) }
    }
}

#[inline(always)]
pub fn get_seconds_since_last_frame() -> f32 {
    *seconds_since_last_frame_raw()
}

#[inline(always)]
pub fn lookup_reference_by_handle_actor(
    handle: &RefHandle,
    refr_out: &mut NiPointer<Actor>,
) -> bool {
    lookup_reference_by_handle_actor_impl(handle, refr_out)
}

#[inline(always)]
pub fn lookup_reference_by_handle_refr(
    handle: &RefHandle,
    refr_out: &mut NiPointer<TESObjectREFR>,
) -> bool {
    lookup_reference_by_handle_refr_impl(handle, refr_out)
}
