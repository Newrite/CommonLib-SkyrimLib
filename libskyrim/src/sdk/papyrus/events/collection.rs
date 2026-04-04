use crate::re::VMHandle;
use crate::skse::SerializationInterface;

use super::types::{PapyrusEventLoadStatus, PapyrusPersistentEventRegistry};

/// Grouped collection of one or more persistent Papyrus event registries.
///
/// This trait is the bridge between per-event registries and runtime callback
/// installers that need to save/load/revert/form-delete a whole registry set.
pub trait PapyrusEventCollection {
    fn save_events(&self, serialization: &SerializationInterface) -> bool;
    fn load_event_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus;
    fn revert_events(&mut self, serialization: Option<&SerializationInterface>);
    fn form_delete_events(&mut self, handle: VMHandle) -> usize;
}

impl PapyrusEventCollection for () {
    #[inline(always)]
    fn save_events(&self, _serialization: &SerializationInterface) -> bool {
        true
    }

    #[inline(always)]
    fn load_event_record(
        &mut self,
        _ty: u32,
        _serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        PapyrusEventLoadStatus::Ignored
    }

    #[inline(always)]
    fn revert_events(&mut self, _serialization: Option<&SerializationInterface>) {}

    #[inline(always)]
    fn form_delete_events(&mut self, _handle: VMHandle) -> usize {
        0
    }
}

impl<T> PapyrusEventCollection for T
where
    T: PapyrusPersistentEventRegistry,
{
    #[inline(always)]
    fn save_events(&self, serialization: &SerializationInterface) -> bool {
        self.save_record(serialization)
    }

    #[inline(always)]
    fn load_event_record(
        &mut self,
        ty: u32,
        serialization: &SerializationInterface,
    ) -> PapyrusEventLoadStatus {
        self.load_record(ty, serialization)
    }

    #[inline(always)]
    fn revert_events(&mut self, serialization: Option<&SerializationInterface>) {
        self.revert(serialization);
    }

    #[inline(always)]
    fn form_delete_events(&mut self, handle: VMHandle) -> usize {
        usize::from(self.form_delete(handle))
    }
}

macro_rules! impl_papyrus_event_collection_tuple {
    ($count:expr; $(($index:tt, $ty:ident)),+ $(,)?) => {
        impl<$($ty),+> PapyrusEventCollection for ($($ty,)+)
        where
            $($ty: PapyrusPersistentEventRegistry,)+
        {
            #[inline(always)]
            fn save_events(&self, serialization: &SerializationInterface) -> bool {
                let registries: [&dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&self.$index),+
                ];
                save_registries(serialization, &registries)
            }

            #[inline(always)]
            fn load_event_record(
                &mut self,
                ty: u32,
                serialization: &SerializationInterface,
            ) -> PapyrusEventLoadStatus {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                load_record(ty, serialization, &mut registries)
            }

            #[inline(always)]
            fn revert_events(&mut self, serialization: Option<&SerializationInterface>) {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                revert_registries(serialization, &mut registries);
            }

            #[inline(always)]
            fn form_delete_events(&mut self, handle: VMHandle) -> usize {
                let mut registries: [&mut dyn PapyrusPersistentEventRegistry; $count] = [
                    $(&mut self.$index),+
                ];
                form_delete_registries(handle, &mut registries)
            }
        }
    };
}

impl_papyrus_event_collection_tuple!(2; (0, A0), (1, A1));
impl_papyrus_event_collection_tuple!(3; (0, A0), (1, A1), (2, A2));
impl_papyrus_event_collection_tuple!(4; (0, A0), (1, A1), (2, A2), (3, A3));
impl_papyrus_event_collection_tuple!(5; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4));
impl_papyrus_event_collection_tuple!(6; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5));
impl_papyrus_event_collection_tuple!(7; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5), (6, A6));
impl_papyrus_event_collection_tuple!(8; (0, A0), (1, A1), (2, A2), (3, A3), (4, A4), (5, A5), (6, A6), (7, A7));

/// Save all persistent Papyrus event registries.
///
/// Returns `false` if any registry is missing record metadata or fails to save.
#[inline(always)]
pub fn save_registries(
    serialization: &SerializationInterface,
    registries: &[&dyn PapyrusPersistentEventRegistry],
) -> bool {
    registries
        .iter()
        .copied()
        .all(|registry| registry.save_record(serialization))
}

/// Load a single Papyrus event record into the first registry that owns `ty`.
#[inline(always)]
pub fn load_record(
    ty: u32,
    serialization: &SerializationInterface,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) -> PapyrusEventLoadStatus {
    for registry in registries {
        let status = registry.load_record(ty, serialization);
        if status != PapyrusEventLoadStatus::Ignored {
            return status;
        }
    }

    PapyrusEventLoadStatus::Ignored
}

/// Revert all Papyrus event registries.
#[inline(always)]
pub fn revert_registries(
    serialization: Option<&SerializationInterface>,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) {
    for registry in registries {
        registry.revert(serialization);
    }
}

/// Route a form-delete handle through all Papyrus event registries.
#[inline(always)]
pub fn form_delete_registries(
    handle: VMHandle,
    registries: &mut [&mut dyn PapyrusPersistentEventRegistry],
) -> usize {
    let mut removed = 0;
    for registry in registries {
        if registry.form_delete(handle) {
            removed += 1;
        }
    }
    removed
}
