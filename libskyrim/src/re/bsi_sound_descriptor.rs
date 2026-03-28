use crate::offsets::offsets_rtti::RTTI_BSISoundDescriptor;
use crate::offsets::offsets_vtable::VTABLE_BSISoundDescriptor;
use crate::re::bs_resource_id::BSResourceID;
use crate::re::bsaudio_monitor::BSAudioMonitorRequest;
use crate::re::bsi_playback_characteristics::BSIPlaybackCharacteristics;
use crate::re::bsi_sound_category::BSISoundCategory;
use crate::re::bsi_sound_output_model::BSISoundOutputModel;
use crate::re::bst_array::BSTSmallArray;
use crate::re::tes_form::FormID;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSISoundDescriptor::Resolution`
#[repr(C)]
pub struct BSISoundDescriptorResolution {
    pub resource_id: BSResourceID,                                 // 00
    pub form_id: FormID,                                           // 0C
    pub alternate_form_id: FormID,                                 // 10
    pub flags: u32,                                                // 14
    pub playback_characteristics: *mut BSIPlaybackCharacteristics, // 18
    pub output_model: *mut BSISoundOutputModel,                    // 20
    pub sound_category: *mut BSISoundCategory,                     // 28
    pub monitor_requests: BSTSmallArray<BSAudioMonitorRequest, 2>, // 30
}

const _: () = assert!(core::mem::size_of::<BSTSmallArray<BSAudioMonitorRequest, 2>>() == 0x18);
const _: () = assert!(core::mem::size_of::<BSISoundDescriptorResolution>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptorResolution, resource_id) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptorResolution, form_id) == 0x0C);
const _: () =
    assert!(core::mem::offset_of!(BSISoundDescriptorResolution, alternate_form_id) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptorResolution, flags) == 0x14);
const _: () =
    assert!(core::mem::offset_of!(BSISoundDescriptorResolution, playback_characteristics) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptorResolution, output_model) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptorResolution, sound_category) == 0x28);
const _: () =
    assert!(core::mem::offset_of!(BSISoundDescriptorResolution, monitor_requests) == 0x30);

/// C++ `RE::BSISoundDescriptor`
#[repr(C)]
pub struct BSISoundDescriptor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSISoundDescriptor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSISoundDescriptor, vtable) == 0x00);

impl RttiType for BSISoundDescriptor {
    const RTTI: VariantID = RTTI_BSISoundDescriptor;
}

impl BSISoundDescriptor {
    pub const RTTI: VariantID = RTTI_BSISoundDescriptor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSISoundDescriptor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_DO_RESOLVE: usize = 0x01;
        pub fn do_resolve(resolution: *mut BSISoundDescriptorResolution) -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_02: usize = 0x02;
        pub fn unk_02()
    }

    #[inline(always)]
    pub fn resolve(&self, resolution: &mut BSISoundDescriptorResolution) -> bool {
        self.do_resolve(resolution as *mut BSISoundDescriptorResolution)
    }
}

pub trait BSISoundDescriptorExt {
    fn do_resolve(&mut self, resolution: *mut BSISoundDescriptorResolution) -> bool;
    fn resolve(&self, resolution: &mut BSISoundDescriptorResolution) -> bool;
    fn unk_02(&mut self);
}

impl<T: AsRef<BSISoundDescriptor> + AsMut<BSISoundDescriptor>> BSISoundDescriptorExt for T {
    fn do_resolve(&mut self, resolution: *mut BSISoundDescriptorResolution) -> bool {
        self.as_mut().do_resolve(resolution)
    }

    fn resolve(&self, resolution: &mut BSISoundDescriptorResolution) -> bool {
        self.as_ref().resolve(resolution)
    }

    fn unk_02(&mut self) {
        self.as_mut().unk_02()
    }
}
