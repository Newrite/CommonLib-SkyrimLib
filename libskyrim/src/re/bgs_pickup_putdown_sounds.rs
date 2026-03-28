use crate::offsets::offsets_rtti::RTTI_BGSPickupPutdownSounds;
use crate::offsets::offsets_vtable::VTABLE_BGSPickupPutdownSounds;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

#[repr(C)]
pub struct BGSPickupPutdownSounds {
    pub base: BaseFormComponent,                    // 00
    pub pickup_sound: *mut BGSSoundDescriptorForm,  // 08 - YNAM
    pub putdown_sound: *mut BGSSoundDescriptorForm, // 10 - ZNAM
}

const _: () = assert!(core::mem::size_of::<BGSPickupPutdownSounds>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSPickupPutdownSounds, pickup_sound) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSPickupPutdownSounds, putdown_sound) == 0x10);

impl RttiType for BGSPickupPutdownSounds {
    const RTTI: VariantID = RTTI_BGSPickupPutdownSounds;
}

inherit!(BGSPickupPutdownSounds : BaseFormComponent);

impl BGSPickupPutdownSounds {
    pub const RTTI: VariantID = RTTI_BGSPickupPutdownSounds;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPickupPutdownSounds;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(&mut self, rhs: *mut BaseFormComponent)
    }

    #[inline(always)]
    pub const fn get_pickup_sound(&self) -> *mut BGSSoundDescriptorForm {
        self.pickup_sound
    }

    #[inline(always)]
    pub fn get_pickup_sound_ref(&self) -> Option<&BGSSoundDescriptorForm> {
        unsafe { self.get_pickup_sound().as_ref() }
    }

    #[inline(always)]
    pub const fn get_putdown_sound(&self) -> *mut BGSSoundDescriptorForm {
        self.putdown_sound
    }

    #[inline(always)]
    pub fn get_putdown_sound_ref(&self) -> Option<&BGSSoundDescriptorForm> {
        unsafe { self.get_putdown_sound().as_ref() }
    }
}

pub trait BGSPickupPutdownSoundsExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_pickup_sound(&self) -> *mut BGSSoundDescriptorForm;
    fn get_pickup_sound_ref(&self) -> Option<&BGSSoundDescriptorForm>;
    fn get_putdown_sound(&self) -> *mut BGSSoundDescriptorForm;
    fn get_putdown_sound_ref(&self) -> Option<&BGSSoundDescriptorForm>;
}

impl<T: AsRef<BGSPickupPutdownSounds> + AsMut<BGSPickupPutdownSounds>> BGSPickupPutdownSoundsExt
    for T
{
    fn dtor(&mut self) {
        BGSPickupPutdownSounds::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        BGSPickupPutdownSounds::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        BGSPickupPutdownSounds::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        BGSPickupPutdownSounds::copy_component(self.as_mut(), rhs)
    }

    fn get_pickup_sound(&self) -> *mut BGSSoundDescriptorForm {
        BGSPickupPutdownSounds::get_pickup_sound(self.as_ref())
    }

    fn get_pickup_sound_ref(&self) -> Option<&BGSSoundDescriptorForm> {
        BGSPickupPutdownSounds::get_pickup_sound_ref(self.as_ref())
    }

    fn get_putdown_sound(&self) -> *mut BGSSoundDescriptorForm {
        BGSPickupPutdownSounds::get_putdown_sound(self.as_ref())
    }

    fn get_putdown_sound_ref(&self) -> Option<&BGSSoundDescriptorForm> {
        BGSPickupPutdownSounds::get_putdown_sound_ref(self.as_ref())
    }
}
