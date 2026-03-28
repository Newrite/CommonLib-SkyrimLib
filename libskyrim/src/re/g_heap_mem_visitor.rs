crate::core_util::abstract_type! {
    /// Source-backed forward declaration of `RE::GHeapMemVisitor`.
    ///
    /// `GMemoryHeap` and `GSysAllocPaged` currently only use this as a raw
    /// pointer parameter in visitor entrypoints.
    pub type GHeapMemVisitor;
}
