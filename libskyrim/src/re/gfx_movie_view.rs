use crate::ffi::{commonlib_gfx_movie_view_add_ref, commonlib_gfx_movie_view_release};
use crate::re::{
    GColor, GFxEvent, GFxMovie, GFxMovieDefMemoryContext, GFxStateBag, GFxValue, GMatrix3D,
    GMemoryHeap, GPoint3F, GPointF, GPtrTarget, GRectF, GStatBag, GViewport,
};
use crate::relocation::RelocationID;

/// C++ `RE::GFxMovieView::ScaleModeType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieViewScaleModeType {
    NoScale = 0,
    ShowAll = 1,
    ExactFit = 2,
    NoBorder = 3,
}

/// C++ `RE::GFxMovieView::AlignType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieViewAlignType {
    Center = 0,
    TopCenter = 1,
    BottomCenter = 2,
    CenterLeft = 3,
    CenterRight = 4,
    TopLeft = 5,
    TopRight = 6,
    BottomLeft = 7,
    BottomRight = 8,
}

/// C++ `RE::GFxMovieView::HEResult`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieViewHEResult {
    NotHandled = 0,
    Handled = 1,
    NoDefaultAction = 2,
    Completed = 3,
}

/// C++ `RE::GFxMovieView::HitTestType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieViewHitTestType {
    Bounds = 0,
    Shapes = 1,
    ButtonEvents = 2,
    ShapesNoInvisible = 3,
}

/// C++ `RE::GFxMovieView`
#[repr(C)]
pub struct GFxMovieView {
    pub base: GFxMovie,         // 00
    pub state_bag: GFxStateBag, // 10
}

const _: () = assert!(core::mem::size_of::<GFxMovieView>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxMovieView, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieView, state_bag) == 0x10);

core_util::inherit!(GFxMovieView : GFxMovie, base);
core_util::inherit!(GFxMovieView => GFxStateBag, state_bag);

impl GFxMovieView {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_SET_VIEWPORT: usize = 0x19; pub fn set_viewport(view_desc: &GViewport) }
    crate::virtual_method! { pub const VFUNC_GET_VIEWPORT: usize = 0x1A; pub fn get_viewport(view_desc: *mut GViewport) }
    crate::virtual_method! { pub const VFUNC_SET_VIEW_SCALE_MODE: usize = 0x1B; pub fn set_view_scale_mode(mode: GFxMovieViewScaleModeType) }
    crate::virtual_method! { pub const VFUNC_GET_VIEW_SCALE_MODE: usize = 0x1C; pub fn get_view_scale_mode() -> GFxMovieViewScaleModeType }
    crate::virtual_method! { pub const VFUNC_SET_VIEW_ALIGNMENT: usize = 0x1D; pub fn set_view_alignment(alignment: GFxMovieViewAlignType) }
    crate::virtual_method! { pub const VFUNC_GET_VIEW_ALIGNMENT: usize = 0x1E; pub fn get_view_alignment() -> GFxMovieViewAlignType }
    crate::virtual_method! { pub const VFUNC_GET_VISIBLE_FRAME_RECT: usize = 0x1F; pub fn get_visible_frame_rect() -> GRectF }
    crate::virtual_method! { pub const VFUNC_SET_PERSPECTIVE_3D: usize = 0x20; pub fn set_perspective_3d(proj_mat: &GMatrix3D) }
    crate::virtual_method! { pub const VFUNC_SET_VIEW_3D: usize = 0x21; pub fn set_view_3d(view_mat: &GMatrix3D) }
    crate::virtual_method! { pub const VFUNC_GET_SAFE_RECT: usize = 0x22; pub fn get_safe_rect() -> GRectF }
    crate::virtual_method! { pub const VFUNC_SET_SAFE_RECT: usize = 0x23; pub fn set_safe_rect(rect: &GRectF) }
    crate::virtual_method! { pub const VFUNC_RESTART: usize = 0x24; pub fn restart() }
    crate::virtual_method! { pub const VFUNC_ADVANCE: usize = 0x25; pub fn advance(delta_t: f32, frame_catch_up_count: u32) -> f32 }
    crate::virtual_method! { pub const VFUNC_DISPLAY: usize = 0x26; pub fn display() }
    crate::virtual_method! { pub const VFUNC_DISPLAY_PRE_PASS: usize = 0x27; pub fn display_pre_pass() }
    crate::virtual_method! { pub const VFUNC_SET_PAUSE: usize = 0x28; pub fn set_pause(pause: bool) }
    crate::virtual_method! { pub const VFUNC_IS_PAUSED: usize = 0x29; pub fn is_paused() -> bool }
    crate::virtual_method! { pub const VFUNC_SET_BACKGROUND_COLOR: usize = 0x2A; pub fn set_background_color(bg_color: GColor) }
    crate::virtual_method! { pub const VFUNC_SET_BACKGROUND_ALPHA: usize = 0x2B; pub fn set_background_alpha(alpha: f32) }
    crate::virtual_method! { pub const VFUNC_GET_BACKGROUND_ALPHA: usize = 0x2C; pub fn get_background_alpha() -> f32 }
    crate::virtual_method! { pub const VFUNC_HANDLE_EVENT: usize = 0x2D; pub fn handle_event(event: &GFxEvent) -> GFxMovieViewHEResult }
    crate::virtual_method! { pub const VFUNC_GET_MOUSE_STATE: usize = 0x2E; pub fn get_mouse_state(mouse_index: u32, x: *mut f32, y: *mut f32, buttons: *mut u32) }
    crate::virtual_method! { pub const VFUNC_NOTIFY_MOUSE_STATE: usize = 0x2F; pub fn notify_mouse_state(x: f32, y: f32, buttons: u32, mouse_index: u32) }
    crate::virtual_method! { pub const VFUNC_HIT_TEST: usize = 0x30; pub fn hit_test(x: f32, y: f32, test_cond: GFxMovieViewHitTestType, controller_idx: u32) -> bool }
    crate::virtual_method! { pub const VFUNC_HIT_TEST_3D: usize = 0x31; pub fn hit_test_3d(pt_out: *mut GPoint3F, x: f32, y: f32, controller_idx: u32) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_EXTERNAL_INTERFACE_RET_VAL: usize = 0x32; pub fn set_external_interface_ret_val(value: &GFxValue) }
    crate::virtual_method! { pub const VFUNC_GET_USER_DATA: usize = 0x33; pub fn get_user_data() -> *mut core::ffi::c_void }
    crate::virtual_method! { pub const VFUNC_SET_USER_DATA: usize = 0x34; pub fn set_user_data(data: *mut core::ffi::c_void) }
    crate::virtual_method! { pub const VFUNC_ATTACH_DISPLAY_CALLBACK: usize = 0x35; pub fn attach_display_callback(path_to_object: *const i8, callback: extern "C" fn(*mut core::ffi::c_void), user: *mut core::ffi::c_void) -> bool }
    crate::virtual_method! { pub const VFUNC_IS_MOVIE_FOCUSED: usize = 0x36; pub fn is_movie_focused() -> bool }
    crate::virtual_method! { pub const VFUNC_GET_DIRTY_FLAG: usize = 0x37; pub fn get_dirty_flag(do_reset: bool) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_MOUSE_CURSOR_COUNT: usize = 0x38; pub fn set_mouse_cursor_count(count: u32) }
    crate::virtual_method! { pub const VFUNC_GET_MOUSE_CURSOR_COUNT: usize = 0x39; pub fn get_mouse_cursor_count() -> u32 }
    crate::virtual_method! { pub const VFUNC_SET_CONTROLLER_COUNT: usize = 0x3A; pub fn set_controller_count(count: u32) }
    crate::virtual_method! { pub const VFUNC_GET_CONTROLLER_COUNT: usize = 0x3B; pub fn get_controller_count() -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_STATS: usize = 0x3C; pub fn get_stats(bag: *mut GStatBag, reset: bool) }
    crate::virtual_method! { pub const VFUNC_GET_HEAP: usize = 0x3D; pub fn get_heap() -> *mut GMemoryHeap }
    crate::virtual_method! { pub const VFUNC_FORCE_COLLECT_GARBAGE: usize = 0x3E; pub fn force_collect_garbage() }
    crate::virtual_method! { pub const VFUNC_TRANSLATE_TO_SCREEN_POINT: usize = 0x3F; pub fn translate_to_screen_point(point: &GPointF, user_matrix: *mut core::ffi::c_void) -> GPointF }
    crate::virtual_method! { pub const VFUNC_TRANSLATE_TO_SCREEN_RECT: usize = 0x40; pub fn translate_to_screen_rect(rect: &GRectF, user_matrix: *mut core::ffi::c_void) -> GRectF }
    crate::virtual_method! { pub const VFUNC_TRANSLATE_LOCAL_TO_SCREEN: usize = 0x41; pub fn translate_local_to_screen(path_to_character: *const i8, point: &GPointF, pres_point: *mut GPointF, user_matrix: *mut core::ffi::c_void) -> bool }
    crate::virtual_method! { pub const VFUNC_SET_CONTROLLER_FOCUS_GROUP: usize = 0x42; pub fn set_controller_focus_group(controller_idx: u32, focus_group_index: u32) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_CONTROLLER_FOCUS_GROUP: usize = 0x43; pub fn get_controller_focus_group(controller_idx: u32) -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_MEMORY_CONTEXT: usize = 0x44; pub fn get_memory_context() -> *mut GFxMovieDefMemoryContext }
    crate::virtual_method! { pub const VFUNC_RELEASE: usize = 0x45; pub fn release() }

    #[inline(always)]
    pub fn set_viewport_parts(
        &mut self,
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        flags: crate::re::GViewportFlag,
    ) {
        self.set_viewport(&GViewport::with_viewport(
            buffer_width,
            buffer_height,
            left,
            top,
            width,
            height,
            flags,
        ));
    }

    crate::relocation_func! {
        pub fn invoke_no_return(&self, method_name: *const i8, args: *const GFxValue, num_args: u32) => RelocationID::new(80547, 82665)
    }
}

impl GPtrTarget for GFxMovieView {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            commonlib_gfx_movie_view_add_ref(core::ptr::from_ref(self).cast_mut().cast());
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            commonlib_gfx_movie_view_release(core::ptr::from_ref(self).cast_mut().cast());
        }
    }
}

impl AsRef<GFxMovieView> for GFxMovieView {
    #[inline(always)]
    fn as_ref(&self) -> &GFxMovieView {
        self
    }
}

impl AsMut<GFxMovieView> for GFxMovieView {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxMovieView {
        self
    }
}

pub trait GFxMovieViewExt: AsRef<GFxMovieView> + AsMut<GFxMovieView> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn set_viewport(&mut self, view_desc: &GViewport) {
        self.as_mut().set_viewport(view_desc)
    }

    #[inline(always)]
    fn get_viewport(&self, view_desc: *mut GViewport) {
        self.as_ref().get_viewport(view_desc)
    }

    #[inline(always)]
    fn set_view_scale_mode(&mut self, mode: GFxMovieViewScaleModeType) {
        self.as_mut().set_view_scale_mode(mode)
    }

    #[inline(always)]
    fn get_view_scale_mode(&self) -> GFxMovieViewScaleModeType {
        self.as_ref().get_view_scale_mode()
    }

    #[inline(always)]
    fn set_view_alignment(&mut self, alignment: GFxMovieViewAlignType) {
        self.as_mut().set_view_alignment(alignment)
    }

    #[inline(always)]
    fn get_view_alignment(&self) -> GFxMovieViewAlignType {
        self.as_ref().get_view_alignment()
    }

    #[inline(always)]
    fn get_visible_frame_rect(&self) -> GRectF {
        self.as_ref().get_visible_frame_rect()
    }

    #[inline(always)]
    fn set_perspective_3d(&mut self, proj_mat: &GMatrix3D) {
        self.as_mut().set_perspective_3d(proj_mat)
    }

    #[inline(always)]
    fn set_view_3d(&mut self, view_mat: &GMatrix3D) {
        self.as_mut().set_view_3d(view_mat)
    }

    #[inline(always)]
    fn get_safe_rect(&self) -> GRectF {
        self.as_ref().get_safe_rect()
    }

    #[inline(always)]
    fn set_safe_rect(&mut self, rect: &GRectF) {
        self.as_mut().set_safe_rect(rect)
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.as_mut().restart()
    }

    #[inline(always)]
    fn advance(&mut self, delta_t: f32, frame_catch_up_count: u32) -> f32 {
        self.as_mut().advance(delta_t, frame_catch_up_count)
    }

    #[inline(always)]
    fn display(&mut self) {
        self.as_mut().display()
    }

    #[inline(always)]
    fn display_pre_pass(&mut self) {
        self.as_mut().display_pre_pass()
    }

    #[inline(always)]
    fn set_pause(&mut self, pause: bool) {
        self.as_mut().set_pause(pause)
    }

    #[inline(always)]
    fn is_paused(&self) -> bool {
        self.as_ref().is_paused()
    }

    #[inline(always)]
    fn set_background_color(&mut self, bg_color: GColor) {
        self.as_mut().set_background_color(bg_color)
    }

    #[inline(always)]
    fn set_background_alpha(&mut self, alpha: f32) {
        self.as_mut().set_background_alpha(alpha)
    }

    #[inline(always)]
    fn get_background_alpha(&self) -> f32 {
        self.as_ref().get_background_alpha()
    }

    #[inline(always)]
    fn handle_event(&mut self, event: &GFxEvent) -> GFxMovieViewHEResult {
        self.as_mut().handle_event(event)
    }

    #[inline(always)]
    fn get_mouse_state(&self, mouse_index: u32, x: *mut f32, y: *mut f32, buttons: *mut u32) {
        self.as_ref().get_mouse_state(mouse_index, x, y, buttons)
    }

    #[inline(always)]
    fn notify_mouse_state(&mut self, x: f32, y: f32, buttons: u32, mouse_index: u32) {
        self.as_mut().notify_mouse_state(x, y, buttons, mouse_index)
    }

    #[inline(always)]
    fn hit_test(
        &mut self,
        x: f32,
        y: f32,
        test_cond: GFxMovieViewHitTestType,
        controller_idx: u32,
    ) -> bool {
        self.as_mut().hit_test(x, y, test_cond, controller_idx)
    }

    #[inline(always)]
    fn hit_test_3d(&mut self, pt_out: *mut GPoint3F, x: f32, y: f32, controller_idx: u32) -> bool {
        self.as_mut().hit_test_3d(pt_out, x, y, controller_idx)
    }

    #[inline(always)]
    fn set_external_interface_ret_val(&mut self, value: &GFxValue) {
        self.as_mut().set_external_interface_ret_val(value)
    }

    #[inline(always)]
    fn get_user_data(&self) -> *mut core::ffi::c_void {
        self.as_ref().get_user_data()
    }

    #[inline(always)]
    fn set_user_data(&mut self, data: *mut core::ffi::c_void) {
        self.as_mut().set_user_data(data)
    }

    #[inline(always)]
    fn attach_display_callback(
        &mut self,
        path_to_object: *const i8,
        callback: extern "C" fn(*mut core::ffi::c_void),
        user: *mut core::ffi::c_void,
    ) -> bool {
        self.as_mut()
            .attach_display_callback(path_to_object, callback, user)
    }

    #[inline(always)]
    fn is_movie_focused(&self) -> bool {
        self.as_ref().is_movie_focused()
    }

    #[inline(always)]
    fn get_dirty_flag(&mut self, do_reset: bool) -> bool {
        self.as_mut().get_dirty_flag(do_reset)
    }

    #[inline(always)]
    fn set_mouse_cursor_count(&mut self, count: u32) {
        self.as_mut().set_mouse_cursor_count(count)
    }

    #[inline(always)]
    fn get_mouse_cursor_count(&self) -> u32 {
        self.as_ref().get_mouse_cursor_count()
    }

    #[inline(always)]
    fn set_controller_count(&mut self, count: u32) {
        self.as_mut().set_controller_count(count)
    }

    #[inline(always)]
    fn get_controller_count(&self) -> u32 {
        self.as_ref().get_controller_count()
    }

    #[inline(always)]
    fn get_stats(&mut self, bag: *mut GStatBag, reset: bool) {
        self.as_mut().get_stats(bag, reset)
    }

    #[inline(always)]
    fn get_heap(&self) -> *mut GMemoryHeap {
        self.as_ref().get_heap()
    }

    #[inline(always)]
    fn force_collect_garbage(&mut self) {
        self.as_mut().force_collect_garbage()
    }

    #[inline(always)]
    fn translate_to_screen_point(
        &mut self,
        point: &GPointF,
        user_matrix: *mut core::ffi::c_void,
    ) -> GPointF {
        self.as_mut().translate_to_screen_point(point, user_matrix)
    }

    #[inline(always)]
    fn translate_to_screen_rect(
        &mut self,
        rect: &GRectF,
        user_matrix: *mut core::ffi::c_void,
    ) -> GRectF {
        self.as_mut().translate_to_screen_rect(rect, user_matrix)
    }

    #[inline(always)]
    fn translate_local_to_screen(
        &mut self,
        path_to_character: *const i8,
        point: &GPointF,
        pres_point: *mut GPointF,
        user_matrix: *mut core::ffi::c_void,
    ) -> bool {
        self.as_mut()
            .translate_local_to_screen(path_to_character, point, pres_point, user_matrix)
    }

    #[inline(always)]
    fn set_controller_focus_group(&mut self, controller_idx: u32, focus_group_index: u32) -> bool {
        self.as_mut()
            .set_controller_focus_group(controller_idx, focus_group_index)
    }

    #[inline(always)]
    fn get_controller_focus_group(&self, controller_idx: u32) -> u32 {
        self.as_ref().get_controller_focus_group(controller_idx)
    }

    #[inline(always)]
    fn get_memory_context(&self) -> *mut GFxMovieDefMemoryContext {
        self.as_ref().get_memory_context()
    }

    #[inline(always)]
    fn release(&mut self) {
        self.as_mut().release()
    }

    #[inline(always)]
    fn set_viewport_parts(
        &mut self,
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        flags: crate::re::GViewportFlag,
    ) {
        self.as_mut().set_viewport_parts(
            buffer_width,
            buffer_height,
            left,
            top,
            width,
            height,
            flags,
        )
    }

    #[inline(always)]
    fn invoke_no_return(&mut self, method_name: *const i8, args: *const GFxValue, num_args: u32) {
        self.as_mut().invoke_no_return(method_name, args, num_args)
    }
}

impl<T> GFxMovieViewExt for T where T: AsRef<GFxMovieView> + AsMut<GFxMovieView> {}
