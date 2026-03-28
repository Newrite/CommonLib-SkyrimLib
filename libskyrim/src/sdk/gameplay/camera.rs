//! Camera-oriented gameplay helpers.

use crate::re::PlayerCamera;
use crate::sdk::core::GameRef;

#[inline(always)]
pub fn singleton() -> GameRef<PlayerCamera> {
    unsafe { GameRef::from_raw(PlayerCamera::get_singleton()) }
}

#[inline(always)]
pub fn force_first_person() -> bool {
    unsafe { singleton().with_mut_unchecked(|camera| camera.force_first_person()) }
}

#[inline(always)]
pub fn force_third_person() -> bool {
    unsafe { singleton().with_mut_unchecked(|camera| camera.force_third_person()) }
}

#[inline(always)]
pub fn is_in_bleedout_mode() -> bool {
    singleton().is_in_bleedout_mode()
}

#[inline(always)]
pub fn is_in_first_person() -> bool {
    singleton().is_in_first_person()
}

#[inline(always)]
pub fn is_in_third_person() -> bool {
    singleton().is_in_third_person()
}
