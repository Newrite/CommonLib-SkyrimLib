use core::ops::Index;
use core::ptr;

use crate::re::{FxDelegateHandler, FxResponseArgsBase, GFxMovieView, GFxValue};

/// C++ `RE::FxDelegateArgs`
#[repr(C)]
pub struct FxDelegateArgs {
    pub response_id: GFxValue,           // 00
    pub handler: *mut FxDelegateHandler, // 18
    pub movie_view: *mut GFxMovieView,   // 20
    pub args: *const GFxValue,           // 28
    pub num_args: u32,                   // 30
    pub pad34: u32,                      // 34
}

const _: () = assert!(core::mem::size_of::<FxDelegateArgs>() == 0x38);

impl FxDelegateArgs {
    #[inline(always)]
    pub fn new(
        response_id: GFxValue,
        handler: *mut FxDelegateHandler,
        movie_view: *mut GFxMovieView,
        vals: *const GFxValue,
        num_args: u32,
    ) -> Self {
        Self {
            response_id,
            handler,
            movie_view,
            args: vals,
            num_args,
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn respond<T: AsMut<FxResponseArgsBase>>(&self, params: &mut T) {
        let mut values = ptr::null_mut::<GFxValue>();
        let num_values = params.as_mut().get_values(ptr::from_mut(&mut values));
        if !values.is_null() {
            unsafe {
                *values = self.response_id.clone();
                (*self.movie_view).invoke_no_return(
                    b"respond\0".as_ptr().cast(),
                    values,
                    num_values,
                );
            }
        }
    }

    #[inline(always)]
    pub const fn get_handler(&self) -> *mut FxDelegateHandler {
        self.handler
    }

    #[inline(always)]
    pub const fn get_movie(&self) -> *mut GFxMovieView {
        self.movie_view
    }

    #[inline(always)]
    pub const fn get_arg_count(&self) -> u32 {
        self.num_args
    }

    #[inline(always)]
    pub fn get(&self, pos: usize) -> Option<&GFxValue> {
        if pos < self.num_args as usize {
            unsafe { self.args.add(pos).as_ref() }
        } else {
            None
        }
    }

    // TODO: `FxDelegateArgs.h` also exposes a variadic `Respond(Args&&...)`
    // convenience wrapper backed by `FxResponseArgs<SIZE>` from
    // `FxResponseArgs.h`. End state: translate that separate response-wrapper
    // header and add the matching Rust convenience helper instead of requiring
    // callers to pass an already-materialized `FxResponseArgsBase`.
}

impl Index<usize> for FxDelegateArgs {
    type Output = GFxValue;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.num_args as usize);
        unsafe { &*self.args.add(index) }
    }
}
