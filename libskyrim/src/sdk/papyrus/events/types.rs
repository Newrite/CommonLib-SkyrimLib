use crate::re::VMHandle;
use crate::skse::SerializationInterface;

/// Persistent record metadata for a Papyrus event registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PapyrusEventRecord {
    pub ty: u32,
    pub version: u32,
}

impl PapyrusEventRecord {
    #[inline(always)]
    pub const fn new(ty: u32, version: u32) -> Self {
        Self { ty, version }
    }
}

/// Result of trying to load a serialized Papyrus event record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapyrusEventLoadStatus {
    Loaded,
    Ignored,
    Failed,
}

/// Object-safe persistence surface shared by SDK Papyrus event registries.
///
/// Each registry should own a distinct `PapyrusEventRecord::ty` when used with
/// grouped `save_registries(...)` / `load_record(...)` helpers.
pub trait PapyrusPersistentEventRegistry {
    fn event_name(&self) -> &str;
    fn record(&self) -> Option<PapyrusEventRecord>;
    fn save_record(&self, serialization: &SerializationInterface) -> bool;
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus;
    fn revert(&mut self, serialization: Option<&SerializationInterface>);
    fn form_delete(&mut self, handle: VMHandle) -> bool;
}
