use alloc::ffi::CString;
use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_IAnimationGraphManagerHolder;
use crate::offsets::offsets_vtable::VTABLE_IAnimationGraphManagerHolder;
use crate::re::{
    BSAnimationGraphChannel, BSAnimationGraphManager, BSAnimationUpdateData, BSFixedString,
    BSScrapArray, BSTSmartPointer, BShkbAnimationGraph, NiAVObject, NiPoint3,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::IAnimationGraphManagerHolder`
#[repr(C)]
pub struct IAnimationGraphManagerHolder {
    pub vtable: *const usize,
}

const _: () = assert!(core::mem::size_of::<IAnimationGraphManagerHolder>() == 0x08);
const _: () = assert!(core::mem::offset_of!(IAnimationGraphManagerHolder, vtable) == 0x00);

impl RttiType for IAnimationGraphManagerHolder {
    const RTTI: VariantID = RTTI_IAnimationGraphManagerHolder;
}

impl AsRef<IAnimationGraphManagerHolder> for IAnimationGraphManagerHolder {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IAnimationGraphManagerHolder> for IAnimationGraphManagerHolder {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IAnimationGraphManagerHolder {
    pub const RTTI: VariantID = RTTI_IAnimationGraphManagerHolder;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IAnimationGraphManagerHolder;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_NOTIFY_ANIMATION_GRAPH: usize = 0x01;
        pub fn notify_animation_graph(event_name: &BSFixedString) -> bool
    }

    #[inline(always)]
    pub fn notify_animation_graph_str(&mut self, event_name: &str) -> bool {
        let Ok(event_name) = CString::new(event_name) else {
            return false;
        };

        unsafe {
            crate::ffi::commonlib_notify_animation_graph(
                (self as *mut Self).cast::<c_void>(),
                event_name.as_ptr(),
            )
        }
    }

    crate::virtual_method! {
        pub const VFUNC_GET_ANIMATION_GRAPH_MANAGER_IMPL: usize = 0x02;
        pub fn get_animation_graph_manager_impl(
            out: &mut BSTSmartPointer<BSAnimationGraphManager>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_ANIMATION_GRAPH_MANAGER_IMPL: usize = 0x03;
        pub fn set_animation_graph_manager_impl(
            input: &mut BSTSmartPointer<BSAnimationGraphManager>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_POPULATE_GRAPH_NODES_TO_TARGET: usize = 0x04;
        pub fn populate_graph_nodes_to_target(nodes: &mut BSScrapArray<*mut NiAVObject>) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CONSTRUCT_ANIMATION_GRAPH: usize = 0x05;
        pub fn construct_animation_graph(out: &mut BSTSmartPointer<BShkbAnimationGraph>) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_06: usize = 0x06;
        pub fn unk_06()
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07()
    }

    crate::virtual_method! {
        pub const VFUNC_SETUP_ANIM_EVENT_SINKS: usize = 0x08;
        pub fn setup_anim_event_sinks(anim_graph: &BSTSmartPointer<BShkbAnimationGraph>) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09()
    }

    crate::virtual_method! {
        pub const VFUNC_CREATE_ANIMATION_CHANNELS: usize = 0x0A;
        pub fn create_animation_channels(
            anim_graph_channels: &mut BSScrapArray<BSTSmartPointer<BSAnimationGraphChannel>>
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_POST_CREATE_ANIMATION_GRAPH_MANAGER: usize = 0x0B;
        pub fn post_create_animation_graph_manager(
            anim_graph_mgr: &mut BSTSmartPointer<BSAnimationGraphManager>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0C: usize = 0x0C;
        pub fn unk_0c()
    }

    crate::virtual_method! {
        pub const VFUNC_POST_CHANGE_ANIMATION_MANAGER: usize = 0x0D;
        pub fn post_change_animation_manager(
            arg1: &BSTSmartPointer<BShkbAnimationGraph>,
            arg2: &BSTSmartPointer<BShkbAnimationGraph>
        )
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0E: usize = 0x0E;
        pub fn unk_0e()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_GRAPH_VARIABLE_CACHE_SIZE: usize = 0x0F;
        pub fn get_graph_variable_cache_size() -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_GRAPH_VARIABLE_IMPL1: usize = 0x10;
        pub fn get_graph_variable_impl1(variable_name: &BSFixedString, out: &mut f32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_GRAPH_VARIABLE_IMPL2: usize = 0x11;
        pub fn get_graph_variable_impl2(variable_name: &BSFixedString, out: &mut i32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_GRAPH_VARIABLE_IMPL3: usize = 0x12;
        pub fn get_graph_variable_impl3(variable_name: &BSFixedString, out: &mut bool) -> bool
    }

    #[inline(always)]
    pub fn get_animation_graph_manager(
        &self,
        out: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool {
        self.get_animation_graph_manager_impl(out)
    }

    #[inline(always)]
    pub fn get_graph_variable_float(&self, variable_name: &BSFixedString, out: &mut f32) -> bool {
        self.get_graph_variable_impl1(variable_name, out)
    }

    #[inline(always)]
    pub fn get_graph_variable_int(&self, variable_name: &BSFixedString, out: &mut i32) -> bool {
        self.get_graph_variable_impl2(variable_name, out)
    }

    #[inline(always)]
    pub fn get_graph_variable_bool(&self, variable_name: &BSFixedString, out: &mut bool) -> bool {
        self.get_graph_variable_impl3(variable_name, out)
    }

    crate::relocation_func! {
        pub fn get_graph_variable_ni_point3(
            &self,
            variable_name: &BSFixedString,
            out: &mut NiPoint3
        ) -> bool => RelocationID::new(32192, 32884)
    }

    #[inline(always)]
    pub fn set_animation_graph_manager(
        &mut self,
        input: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool {
        self.set_animation_graph_manager_impl(input)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_bool(
            &self,
            variable_name: &BSFixedString,
            input: bool
        ) -> bool => RelocationID::new(32141, 32885)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_int(
            &self,
            variable_name: &BSFixedString,
            input: i32
        ) -> bool => RelocationID::new(32142, 32886)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_float(
            &self,
            variable_name: &BSFixedString,
            input: f32
        ) -> bool => RelocationID::new(32143, 32887)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_ni_point3(
            &self,
            variable_name: &BSFixedString,
            input: &mut NiPoint3
        ) -> bool => RelocationID::new(32144, 32888)
    }

    crate::relocation_func! {
        pub fn update_animation_graph_manager(
            &mut self,
            update_data: &BSAnimationUpdateData
        ) -> bool => RelocationID::new(32155, 32899)
    }
}

pub trait IAnimationGraphManagerHolderExt {
    fn notify_animation_graph(&mut self, event_name: &BSFixedString) -> bool;
    fn notify_animation_graph_str(&mut self, event_name: &str) -> bool;
    fn get_animation_graph_manager(
        &self,
        out: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool;
    fn get_graph_variable_float(&self, variable_name: &BSFixedString, out: &mut f32) -> bool;
    fn get_graph_variable_int(&self, variable_name: &BSFixedString, out: &mut i32) -> bool;
    fn get_graph_variable_bool(&self, variable_name: &BSFixedString, out: &mut bool) -> bool;
    fn get_graph_variable_ni_point3(
        &self,
        variable_name: &BSFixedString,
        out: &mut NiPoint3,
    ) -> bool;
    fn set_animation_graph_manager(
        &mut self,
        input: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool;
    fn set_graph_variable_bool(&self, variable_name: &BSFixedString, input: bool) -> bool;
    fn set_graph_variable_int(&self, variable_name: &BSFixedString, input: i32) -> bool;
    fn set_graph_variable_float(&self, variable_name: &BSFixedString, input: f32) -> bool;
    fn set_graph_variable_ni_point3(
        &self,
        variable_name: &BSFixedString,
        input: &mut NiPoint3,
    ) -> bool;
    fn update_animation_graph_manager(&mut self, update_data: &BSAnimationUpdateData) -> bool;
}

impl<T: AsRef<IAnimationGraphManagerHolder> + AsMut<IAnimationGraphManagerHolder>>
    IAnimationGraphManagerHolderExt for T
{
    fn notify_animation_graph(&mut self, event_name: &BSFixedString) -> bool {
        IAnimationGraphManagerHolder::notify_animation_graph(self.as_mut(), event_name)
    }

    fn notify_animation_graph_str(&mut self, event_name: &str) -> bool {
        IAnimationGraphManagerHolder::notify_animation_graph_str(self.as_mut(), event_name)
    }

    fn get_animation_graph_manager(
        &self,
        out: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool {
        IAnimationGraphManagerHolder::get_animation_graph_manager(self.as_ref(), out)
    }

    fn get_graph_variable_float(&self, variable_name: &BSFixedString, out: &mut f32) -> bool {
        IAnimationGraphManagerHolder::get_graph_variable_float(self.as_ref(), variable_name, out)
    }

    fn get_graph_variable_int(&self, variable_name: &BSFixedString, out: &mut i32) -> bool {
        IAnimationGraphManagerHolder::get_graph_variable_int(self.as_ref(), variable_name, out)
    }

    fn get_graph_variable_bool(&self, variable_name: &BSFixedString, out: &mut bool) -> bool {
        IAnimationGraphManagerHolder::get_graph_variable_bool(self.as_ref(), variable_name, out)
    }

    fn get_graph_variable_ni_point3(
        &self,
        variable_name: &BSFixedString,
        out: &mut NiPoint3,
    ) -> bool {
        IAnimationGraphManagerHolder::get_graph_variable_ni_point3(
            self.as_ref(),
            variable_name,
            out,
        )
    }

    fn set_animation_graph_manager(
        &mut self,
        input: &mut BSTSmartPointer<BSAnimationGraphManager>,
    ) -> bool {
        IAnimationGraphManagerHolder::set_animation_graph_manager(self.as_mut(), input)
    }

    fn set_graph_variable_bool(&self, variable_name: &BSFixedString, input: bool) -> bool {
        IAnimationGraphManagerHolder::set_graph_variable_bool(self.as_ref(), variable_name, input)
    }

    fn set_graph_variable_int(&self, variable_name: &BSFixedString, input: i32) -> bool {
        IAnimationGraphManagerHolder::set_graph_variable_int(self.as_ref(), variable_name, input)
    }

    fn set_graph_variable_float(&self, variable_name: &BSFixedString, input: f32) -> bool {
        IAnimationGraphManagerHolder::set_graph_variable_float(self.as_ref(), variable_name, input)
    }

    fn set_graph_variable_ni_point3(
        &self,
        variable_name: &BSFixedString,
        input: &mut NiPoint3,
    ) -> bool {
        IAnimationGraphManagerHolder::set_graph_variable_ni_point3(
            self.as_ref(),
            variable_name,
            input,
        )
    }

    fn update_animation_graph_manager(&mut self, update_data: &BSAnimationUpdateData) -> bool {
        IAnimationGraphManagerHolder::update_animation_graph_manager(self.as_mut(), update_data)
    }
}
