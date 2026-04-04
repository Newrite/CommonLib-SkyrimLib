use alloc::string::String;

use crate::re::{ActiveEffect, BGSRefAlias, TESForm, TESObjectREFR, VMHandle};
use crate::sdk::core::GamePtr;
use crate::skse::{RegistrationEventArgs, RegistrationSetUnique, SerializationInterface};

use super::types::{PapyrusEventLoadStatus, PapyrusEventRecord, PapyrusPersistentEventRegistry};

/// SDK-facing wrapper over `skse::RegistrationSetUnique`.
pub struct PapyrusTargetedEventRegistry<Args = ()> {
    inner: RegistrationSetUnique<Args>,
    record: Option<PapyrusEventRecord>,
}

impl<Args> PapyrusTargetedEventRegistry<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            inner: RegistrationSetUnique::new(event_name),
            record: None,
        }
    }

    #[inline(always)]
    pub fn persistent(event_name: impl Into<String>, record: PapyrusEventRecord) -> Self {
        Self::new(event_name).with_record(record)
    }

    #[inline(always)]
    pub fn with_record(mut self, record: PapyrusEventRecord) -> Self {
        self.record = Some(record);
        self
    }

    #[inline(always)]
    pub fn inner(&self) -> &RegistrationSetUnique<Args> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut RegistrationSetUnique<Args> {
        &mut self.inner
    }

    #[inline(always)]
    pub fn event_name(&self) -> &str {
        self.inner.event_name()
    }

    #[inline(always)]
    pub const fn record(&self) -> Option<PapyrusEventRecord> {
        self.record
    }

    #[inline(always)]
    pub fn set_record(&mut self, record: PapyrusEventRecord) {
        self.record = Some(record);
    }

    #[inline(always)]
    pub fn clear_record(&mut self) {
        self.record = None;
    }

    #[inline(always)]
    pub fn register_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.register_form(form.into())
    }

    #[inline(always)]
    pub fn register_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.register_alias(alias.into())
    }

    #[inline(always)]
    pub fn register_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.register_alias(alias)
    }

    #[inline(always)]
    pub fn register_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.register_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_form(&mut self, form: impl Into<GamePtr<TESForm>>) -> bool {
        self.inner.unregister_form(form.into())
    }

    #[inline(always)]
    pub fn unregister_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.unregister_alias(alias.into())
    }

    #[inline(always)]
    pub fn unregister_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.unregister_alias(alias)
    }

    #[inline(always)]
    pub fn unregister_active_effect(&mut self, effect: impl Into<GamePtr<ActiveEffect>>) -> bool {
        self.inner.unregister_active_effect(effect.into())
    }

    #[inline(always)]
    pub fn unregister_handle(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }

    #[inline(always)]
    pub fn unregister_target(&mut self, target_id: u32) -> bool {
        self.inner.unregister_unique_id(target_id)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn for_each_target_handle(&self, target_id: u32, f: impl FnMut(VMHandle)) {
        self.inner.for_each_handle(target_id, f);
    }

    #[inline(always)]
    pub fn save_record(&self, serialization: &SerializationInterface) -> bool {
        match self.record {
            Some(record) => self
                .inner
                .save_record(serialization, record.ty, record.version),
            None => false,
        }
    }

    #[inline(always)]
    pub fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        let Some(record) = self.record else {
            return PapyrusEventLoadStatus::Ignored;
        };
        if record.ty != ty {
            return PapyrusEventLoadStatus::Ignored;
        }

        if self.inner.load(serialization) {
            PapyrusEventLoadStatus::Loaded
        } else {
            PapyrusEventLoadStatus::Failed
        }
    }

    #[inline(always)]
    pub fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        self.inner.revert(serialization);
    }

    #[inline(always)]
    pub fn form_delete(&mut self, handle: VMHandle) -> bool {
        self.inner.unregister_handle(handle)
    }
}

impl<Args> PapyrusTargetedEventRegistry<Args>
where
    Args: RegistrationEventArgs,
{
    #[inline(always)]
    pub fn send_to(&self, target: impl Into<GamePtr<TESObjectREFR>>, args: Args) {
        self.inner.send_event(target.into(), args);
    }
}

impl<Args> PapyrusTargetedEventRegistry<Args>
where
    Args: RegistrationEventArgs + Send + 'static,
{
    #[inline(always)]
    pub fn queue_send_to(&'static self, target: impl Into<GamePtr<TESObjectREFR>>, args: Args) {
        self.inner.queue_event(target.into(), args);
    }
}

impl<Args> PapyrusPersistentEventRegistry for PapyrusTargetedEventRegistry<Args> {
    #[inline(always)]
    fn event_name(&self) -> &str {
        PapyrusTargetedEventRegistry::event_name(self)
    }

    #[inline(always)]
    fn record(&self) -> Option<PapyrusEventRecord> {
        PapyrusTargetedEventRegistry::record(self)
    }

    #[inline(always)]
    fn save_record(&self, serialization: &SerializationInterface) -> bool {
        PapyrusTargetedEventRegistry::save_record(self, serialization)
    }

    #[inline(always)]
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusTargetedEventRegistry::load_record(self, ty, serialization)
    }

    #[inline(always)]
    fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        PapyrusTargetedEventRegistry::revert(self, serialization);
    }

    #[inline(always)]
    fn form_delete(&mut self, handle: VMHandle) -> bool {
        PapyrusTargetedEventRegistry::form_delete(self, handle)
    }
}

/// Compatibility alias for callers who think in terms of
/// `RegistrationSetUnique`.
pub type PapyrusUniqueEventRegistry<Args = ()> = PapyrusTargetedEventRegistry<Args>;
