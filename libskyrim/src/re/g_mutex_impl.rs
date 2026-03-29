crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::GMutexImpl`.
    ///
    /// `GMutex.h` only forward-declares the pointee and stores it behind a raw
    /// pointer field.
    pub type GMutexImpl;
}
