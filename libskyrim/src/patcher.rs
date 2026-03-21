//!
//! @file patcher.rs
//! @author Andrew Spaulding (Kasplat) / Refactored for CommonLib-NG
//! @brief Locates and applies pre-defined patches to game functions and objects.
//! @bug No known bugs.
//!
//! This file includes the patcher implementation, which reads in arrays of descriptor
//! from the skyrim and patches modules, and then applies them to the game. A descriptor
//! is either the location of a game function/object or a modification to a game function.
//!

use core::cell::UnsafeCell;
use core::ptr::NonNull;

use crate::log::skse_message;
use crate::version;
use crate::ffi;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Signature definitions
////////////////////////////////////////////////////////////////////////////////////////////////////

///
/// Used to match code to pre-defined signatures.
///
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Opcode {
    Code(u8),
    Any
}

/// Identifies a distinct string of binary code within the skyrim binary.
#[derive(Copy, Clone, Debug)]
pub struct Signature(&'static [Opcode]);

/// Generates a new signature out of hex digits and question marks.
#[macro_export]
macro_rules! signature {
    ( $($sig:tt),+; $size:literal ) => {{
        let psize = [ $($crate::patcher::signature!(@munch $sig)),* ].len();
        $crate::patcher::core::assert!($size == psize, "Patch size is incorrect.");
        $crate::patcher::Signature::new(&[ $($crate::patcher::signature!(@munch $sig)),* ])
    }};

    ( @munch $op:literal ) => { $crate::patcher::Opcode::Code($op) };
    ( @munch ? )           => { $crate::patcher::Opcode::Any       };
}
pub use signature;

// Для макроса внутри самого модуля
pub use core;

impl Signature {
    pub const fn new(sig: &'static [Opcode]) -> Self {
        Self(sig)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the signature matches a given slice of memory bytes
    pub fn check_slice(&self, mem: &[u8]) -> bool {
        if self.len() != mem.len() {
            return false;
        }
        for (i, op) in self.0.iter().enumerate() {
            if let Opcode::Code(b) = op {
                if *b != mem[i] {
                    return false;
                }
            }
        }
        true
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Patcher definitions
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Contains a version independent address ID for the specified skyrim versions.
pub enum GameLocation {
    Base { se_id: usize, ae_id: usize, offset: usize },
}

impl GameLocation {
    /// Gets the absolute memory address by resolving the ID through CommonLib-NG
    pub fn get_address(&self) -> Result<usize, ()> {
        let is_se = version::current_runtime() <= version::RUNTIME_VERSION_1_5_97;

        match *self {
            Self::Base { se_id, ae_id, offset } => {
                let id = if is_se { se_id } else { ae_id };

                // Fetch the base address of the ID via FFI (CommonLib REL::ID)
                let base_addr = unsafe { ffi::commonlib_id_to_address(id) };

                if base_addr == 0 {
                    return Err(());
                }

                Ok(base_addr + offset)
            }
        }
    }
}

/// Encodes the type of hook which is being used by a patch.
#[derive(Clone)]
pub enum Hook {
    None,

    /// Jumps to the entry point and records the original function in the trampoline.
    DirectJump {
        entry: *const u8,
        trampoline: NonNull<UnsafeCell<usize>>
    },

    /// Calls the entry point.
    DirectCall(*const u8),

    /// Pure byte patch without a branch/call (e.g. replacing a small instruction with NOPs)
    WriteBytes(&'static [u8]),
}

/// An object in Skyrim's code to be located by the patcher, and modified if necessary.
pub enum DescriptorObject {
    Function(NonNull<UnsafeCell<usize>>),
    Global(NonNull<UnsafeCell<usize>>),
    Patch {
        enabled: fn() -> bool,
        conflicts: Option<&'static str>,
        hook: Hook,
        sig: Signature,
    }
}

/// Describes a named location in the games code to be found and used by the patcher.
pub struct Descriptor {
    pub name: &'static str,
    pub loc: GameLocation,
    pub object: DescriptorObject
}

///
/// Contains an address retrieved by the patcher.
///
#[repr(transparent)]
pub struct GameRef<T>(UnsafeCell<usize>, core::marker::PhantomData<T>);

impl<T> GameRef<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(0), core::marker::PhantomData)
    }

    pub const fn inner(&self) -> NonNull<UnsafeCell<usize>> {
        unsafe { NonNull::new_unchecked(&self.0 as *const _ as *mut _) }
    }

    pub fn get(&self) -> T {
        unsafe {
            let addr = *self.0.get();
            core::mem::transmute_copy::<usize, T>(&addr)
        }
    }
}

unsafe impl Sync for Hook {}
unsafe impl Sync for Descriptor {}
unsafe impl<T> Sync for GameRef<T> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Patcher implementation
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Locates any game functions/objects, and applies any code patches via CommonLib Trampoline.
pub fn apply<const NUM_PATCHES: usize>(
    patches: [&Descriptor; NUM_PATCHES]
) -> Result<(), ()> {
    skse_message!("--------------------- Skyrim Patcher (CommonLib-NG) ---------------------");

    let mut failed = false;

    for descriptor in patches.iter() {
        // Check if the patch is enabled by settings
        if let DescriptorObject::Patch { enabled, .. } = descriptor.object {
            if !enabled() {
                skse_message!("[SKIPPED] {} is disabled", descriptor.name);
                continue;
            }
        }

        // 1. Resolve Address
        let addr = match descriptor.loc.get_address() {
            Ok(a) => a,
            Err(_) => {
                skse_message!("[FAILURE] Could not resolve ID for: {}", descriptor.name);
                failed = true;
                continue;
            }
        };

        // 2. Signature Check
        if let DescriptorObject::Patch { sig, .. } = &descriptor.object {
            if sig.len() > 0 {
                let mem_slice = unsafe { core::slice::from_raw_parts(addr as *const u8, sig.len()) };
                if !sig.check_slice(mem_slice) {
                    skse_message!("[FAILURE] Signature mismatch for: {} at {:#x}", descriptor.name, addr);
                    failed = true;
                    continue;
                }
            }
        }

        // 3. Apply Patch via FFI (CommonLib)
        unsafe {
            match &descriptor.object {
                DescriptorObject::Patch { hook, sig, .. } => {
                    match hook {
                        Hook::DirectJump { entry, trampoline } => {
                            // CommonLib Trampoline write_branch5
                            let orig = ffi::commonlib_write_branch5(addr, *entry as usize);
                            *(trampoline.as_ref().get()) = orig;

                            // Fill remaining signature bytes with NOPs (0x90)
                            if sig.len() > 5 {
                                let nops = alloc::vec![0x90u8; sig.len() - 5];
                                ffi::commonlib_safe_write(addr + 5, nops.as_ptr(), nops.len());
                            }
                        },
                        Hook::DirectCall(entry) => {
                            // CommonLib Trampoline write_call5
                            ffi::commonlib_write_call5(addr, *entry as usize);

                            // Fill remaining signature bytes with NOPs (0x90)
                            if sig.len() > 5 {
                                let nops = alloc::vec![0x90u8; sig.len() - 5];
                                ffi::commonlib_safe_write(addr + 5, nops.as_ptr(), nops.len());
                            }
                        },
                        Hook::WriteBytes(bytes) => {
                            // Direct memory write via CommonLib safe_write
                            ffi::commonlib_safe_write(addr, bytes.as_ptr(), bytes.len());
                        },
                        Hook::None => {}
                    }
                },

                // Global Variable or Function location capture
                DescriptorObject::Function(result) | DescriptorObject::Global(result) => {
                    *(result.as_ref().get()) = addr;
                }
            }
        }

        skse_message!("[SUCCESS] {} resolved at {:#x}", descriptor.name, addr);
    }

    if failed {
        skse_message!("[FAILURE] Could not apply all patches!");
        skse_message!("-------------------------------------------------------------------------");
        return Err(());
    }

    skse_message!("[SUCCESS] All patches successfully applied.");
    skse_message!("-------------------------------------------------------------------------");
    Ok(())
}

/// Flattens multiple arrays of patches into a single array.
pub fn flatten_patch_groups<const N: usize>(
    groups: &[&'static [Descriptor]]
) -> [&'static Descriptor; N] {
    let mut res = core::mem::MaybeUninit::<[&Descriptor; N]>::uninit();
    let mut i = 0;

    for g in groups.iter() {
        for d in g.iter() {
            unsafe { (*res.as_mut_ptr())[i] = d; }
            i += 1;
        }
    }

    assert!(i == N, "flatten_patch_groups: Size mismatch!");
    unsafe { res.assume_init() }
}
