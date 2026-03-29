crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::GASRefCountCollector`.
    ///
    /// The vendored CommonLibVR tree only forward-declares this type from
    /// `GFxMovieDef.h`; there is no standalone header/source pair here to
    /// support a fuller source-backed layout translation yet.
    // TODO: `GFxMovieDef.h` only forward-declares `GASRefCountCollector`, so
    // the Rust translation must stay opaque for now. Replace this with a real
    // intrusive pointee translation once the vendored tree includes the actual
    // header/source that defines its layout and refcount surface.
    pub type GASRefCountCollector;
}
