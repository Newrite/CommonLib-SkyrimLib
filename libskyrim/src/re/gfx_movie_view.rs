use crate::ffi::{commonlib_gfx_movie_view_add_ref, commonlib_gfx_movie_view_release};
use crate::re::GPtrTarget;

crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::GFxMovieView`.
    ///
    /// `UI`/`IMenu` currently only need this as a `GPtr<GFxMovieView>` pointee.
    pub type GFxMovieView;
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
