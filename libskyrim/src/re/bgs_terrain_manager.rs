use crate::re::bgs_terrain_node::BGSTerrainNode;
use crate::re::bst_array::BSTArray;
use crate::re::tes_world_space::TESWorldSpace;

/// C++ `RE::BGSTerrainManager`
#[repr(C)]
pub struct BGSTerrainManager {
    pub unk00: u64,                                       // 00
    pub world_space: *mut TESWorldSpace,                  // 08
    pub root_node: *mut BGSTerrainNode,                   // 10
    pub min_cell_x: i16,                                  // 18
    pub min_cell_y: i16,                                  // 1A
    pub max_level: u32,                                   // 1C
    pub min_level: u32,                                   // 20
    pub root_level: u32,                                  // 24
    pub segmented_block_level: u32,                       // 28
    pub tree_level: u32,                                  // 2C
    pub unk30: bool,                                      // 30
    pub unk31: u8,                                        // 31
    pub lod_trees_hidden: bool,                           // 32
    pub unk33: u8,                                        // 33
    pub needs_immediate_update: bool,                     // 34
    pub unk35: u8,                                        // 35
    pub has_lod: bool,                                    // 36
    pub unk37: u8,                                        // 37
    pub unk38: u64,                                       // 38
    pub unk40: u64,                                       // 40
    pub unk48: u64,                                       // 48
    pub unk50: u64,                                       // 50
    pub unk58: u64,                                       // 58
    pub update_nodes: BSTArray<*mut BGSTerrainNode>,      // 60
    pub next_update_node: u32,                            // 78
    pub unk7c: u32,                                       // 7C
    pub immediate_updates: BSTArray<*mut BGSTerrainNode>, // 80
    pub unk98: u64,                                       // 98
    pub unk_a0: u64,                                      // A0
    pub unk_a8: u64,                                      // A8
    pub unk_b0: u64,                                      // B0
    pub unk_b8: u64,                                      // B8
    pub unk_c0: u64,                                      // C0
    pub unk_c8: u64,                                      // C8
}

const _: () = assert!(core::mem::size_of::<BGSTerrainManager>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(BGSTerrainManager, world_space) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSTerrainManager, root_node) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSTerrainManager, update_nodes) == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSTerrainManager, immediate_updates) == 0x80);
