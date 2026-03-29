// TODO: SOURCE - replace this opaque stand-in with a real `NavMeshInfoMap`
// translation from `RE/N/NavMeshInfoMap.h` once its missing base layers
// (`BSNavmeshInfoMap` and `PrecomputedNavmeshInfoPathMap`) are translated;
// `TES::RUNTIME_DATA2` currently stores only a raw pointer at offset `0x168`.
core_util::abstract_type! {
    pub type NavMeshInfoMap;
}
