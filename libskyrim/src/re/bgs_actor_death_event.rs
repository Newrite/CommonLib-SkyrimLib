crate::core_util::abstract_type! { pub type BGSActorDeathEvent; }

// TODO: The vendored CommonLibVR tree only forward-declares `RE::BGSActorDeathEvent` in the
// PlayerCharacter-facing surface we currently have. Replace this opaque stand-in with the real
// event payload once its defining header/source surface is vendored and translated.
