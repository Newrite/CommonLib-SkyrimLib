#![allow(non_camel_case_types)]

use crate::re::{GFxState, GFxStateStateType, GPtr};

/// C++ `RE::GFxStateBag`
#[repr(C)]
pub struct GFxStateBag {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxStateBag>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxStateBag, vtable) == 0x0);

impl GFxStateBag {
    crate::virtual_method! {
        pub const VFUNC_GET_STATE_BAG_IMPL: usize = 0x00;
        pub fn get_state_bag_impl() -> *mut GFxStateBag
    }

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x01;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_SET_STATE: usize = 0x02;
        pub fn set_state(state_type: GFxStateStateType, state: *mut GFxState)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STATE_ADD_REF: usize = 0x03;
        pub fn get_state_add_ref(state_type: GFxStateStateType) -> *mut GFxState
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STATES_ADD_REF: usize = 0x04;
        pub fn get_states_add_ref(state_list: *mut *mut GFxState, state_types: *const GFxStateStateType, count: u32)
    }

    #[inline(always)]
    pub fn get_state(&self, state_type: GFxStateStateType) -> GPtr<GFxState> {
        unsafe { GPtr::from_raw(self.get_state_add_ref(state_type)) }
    }
}

impl AsRef<GFxStateBag> for GFxStateBag {
    #[inline(always)]
    fn as_ref(&self) -> &GFxStateBag {
        self
    }
}

impl AsMut<GFxStateBag> for GFxStateBag {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxStateBag {
        self
    }
}

pub trait GFxStateBagExt: AsRef<GFxStateBag> + AsMut<GFxStateBag> {
    #[inline(always)]
    fn get_state_bag_impl(&self) -> *mut GFxStateBag {
        self.as_ref().get_state_bag_impl()
    }

    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn set_state(&mut self, state_type: GFxStateStateType, state: *mut GFxState) {
        self.as_mut().set_state(state_type, state)
    }

    #[inline(always)]
    fn get_state_add_ref(&self, state_type: GFxStateStateType) -> *mut GFxState {
        self.as_ref().get_state_add_ref(state_type)
    }

    #[inline(always)]
    fn get_states_add_ref(
        &self,
        state_list: *mut *mut GFxState,
        state_types: *const GFxStateStateType,
        count: u32,
    ) {
        self.as_ref()
            .get_states_add_ref(state_list, state_types, count)
    }

    #[inline(always)]
    fn get_state(&self, state_type: GFxStateStateType) -> GPtr<GFxState> {
        self.as_ref().get_state(state_type)
    }
}

impl<T> GFxStateBagExt for T where T: AsRef<GFxStateBag> + AsMut<GFxStateBag> {}
