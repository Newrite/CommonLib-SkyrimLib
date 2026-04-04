use alloc::string::String;

use crate::re::{ActiveEffect, BGSBaseAlias, BGSRefAlias, TESForm, VMHandle};
use crate::sdk::core::GamePtr;
use crate::skse::{RegistrationEventArgs, RegistrationSet, SerializationInterface};

use super::types::{PapyrusEventLoadStatus, PapyrusEventRecord, PapyrusPersistentEventRegistry};

/// SDK-facing wrapper over `skse::RegistrationSet`.
pub struct PapyrusEventRegistry<Args = ()> {
    inner: RegistrationSet<Args>,
    record: Option<PapyrusEventRecord>,
}

impl<Args> PapyrusEventRegistry<Args> {
    #[inline(always)]
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            inner: RegistrationSet::new(event_name),
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
    pub fn inner(&self) -> &RegistrationSet<Args> {
        &self.inner
    }

    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut RegistrationSet<Args> {
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
    pub fn register_alias(&mut self, alias: impl Into<GamePtr<BGSBaseAlias>>) -> bool {
        self.inner.register_alias(alias.into())
    }

    #[inline(always)]
    pub fn register_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.register_alias(alias.into().base())
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
    pub fn unregister_alias(&mut self, alias: impl Into<GamePtr<BGSBaseAlias>>) -> bool {
        self.inner.unregister_alias(alias.into())
    }

    #[inline(always)]
    pub fn unregister_ref_alias(&mut self, alias: impl Into<GamePtr<BGSRefAlias>>) -> bool {
        self.inner.unregister_alias(alias.into().base())
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
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn for_each_handle(&self, f: impl FnMut(VMHandle)) {
        self.inner.for_each_handle(f);
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

impl<Args> PapyrusEventRegistry<Args>
where
    Args: RegistrationEventArgs,
{
    #[inline(always)]
    pub fn send(&self, args: Args) {
        self.inner.send_event(args);
    }
}

impl<Args> PapyrusEventRegistry<Args>
where
    Args: RegistrationEventArgs + Send + 'static,
{
    #[inline(always)]
    pub fn queue_send(&'static self, args: Args) {
        self.inner.queue_event(args);
    }
}

impl<Args> PapyrusPersistentEventRegistry for PapyrusEventRegistry<Args> {
    #[inline(always)]
    fn event_name(&self) -> &str {
        PapyrusEventRegistry::event_name(self)
    }

    #[inline(always)]
    fn record(&self) -> Option<PapyrusEventRecord> {
        PapyrusEventRegistry::record(self)
    }

    #[inline(always)]
    fn save_record(&self, serialization: &SerializationInterface) -> bool {
        PapyrusEventRegistry::save_record(self, serialization)
    }

    #[inline(always)]
    fn load_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusEventRegistry::load_record(self, ty, serialization)
    }

    #[inline(always)]
    fn revert(&mut self, serialization: Option<&SerializationInterface>) {
        PapyrusEventRegistry::revert(self, serialization);
    }

    #[inline(always)]
    fn form_delete(&mut self, handle: VMHandle) -> bool {
        PapyrusEventRegistry::form_delete(self, handle)
    }
}
