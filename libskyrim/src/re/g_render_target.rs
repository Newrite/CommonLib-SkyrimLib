crate::core_util::abstract_type! {
    /// Source-backed forward declaration of `RE::GRenderTarget`.
    ///
    /// Current GFx renderer translations only need pointer identity.
    pub type GRenderTarget;
}

// TODO: `GRenderer.h` only forward-declares `GRenderTarget`, and this
// translation batch only needs pointer compatibility for renderer virtual
// methods. Replace this stub with the matching header/source translation once a
// consumer needs `GRenderTarget` layout, RTTI, vtable slots, or owner helpers.
