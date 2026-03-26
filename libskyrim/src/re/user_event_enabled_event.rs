crate::core_util::abstract_type! { pub type UserEventEnabledEvent; }

// TODO: The vendored CommonLibVR tree only forward-declares `RE::UserEventEnabledEvent` in the
// headers we currently have. Replace this opaque stand-in with an honest layout once the event's
// defining header/source surface is available locally.
