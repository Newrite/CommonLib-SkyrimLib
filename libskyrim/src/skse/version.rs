use core::ffi::c_char;

use crate::version::Version;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionIndependence {
    AddressLibraryPre1629 = 0,
    AddressLibraryPost1629 = 1,
    AddressLibrary = 2,
    SignatureScanning = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructCompatibility {
    Dependent = 0,
    Independent = 1,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct PluginDeclarationVersionNumber(pub u32);

impl PluginDeclarationVersionNumber {
    #[inline(always)]
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self(Version::new(major, minor, patch, build).pack())
    }

    #[inline(always)]
    pub const fn from_version(version: Version) -> Self {
        Self(version.pack())
    }

    #[inline(always)]
    pub const fn get(self) -> Version {
        Version::from_packed(self.0)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginDeclarationString<const N: usize> {
    pub buffer: [c_char; N],
}

impl<const N: usize> Default for PluginDeclarationString<N> {
    #[inline(always)]
    fn default() -> Self {
        Self { buffer: [0; N] }
    }
}

impl<const N: usize> PluginDeclarationString<N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { buffer: [0; N] }
    }

    #[inline(always)]
    pub const fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        assert!(
            bytes.len() < N,
            "string does not fit in PluginDeclarationString"
        );

        let mut buffer = [0; N];
        let mut i = 0;
        while i < bytes.len() {
            buffer[i] = bytes[i] as c_char;
            i += 1;
        }

        Self { buffer }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const c_char {
        self.buffer.as_ptr()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeCompatibility {
    pub flags: u32,
    pub compatible_versions: [PluginDeclarationVersionNumber; 16],
}

const _: () = assert!(core::mem::size_of::<RuntimeCompatibility>() == 0x44);

impl Default for RuntimeCompatibility {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeCompatibility {
    pub const MAX_COMPATIBLE_VERSIONS: usize = 16;
    pub const FLAG_ADDRESS_LIBRARY: u32 = 1 << 0;
    pub const FLAG_SIGNATURE_SCANNING: u32 = 1 << 1;
    pub const FLAG_STRUCTS_POST_629: u32 = 1 << 2;

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            flags: Self::FLAG_ADDRESS_LIBRARY,
            compatible_versions: [PluginDeclarationVersionNumber(0); 16],
        }
    }

    #[inline(always)]
    pub const fn from_version_independence(
        version_independence: VersionIndependence,
        requires_post_629_structs: bool,
    ) -> Self {
        let mut flags = 0;
        if matches!(version_independence, VersionIndependence::AddressLibrary) {
            flags |= Self::FLAG_ADDRESS_LIBRARY;
        }
        if matches!(version_independence, VersionIndependence::SignatureScanning) {
            flags |= Self::FLAG_SIGNATURE_SCANNING;
        }
        if requires_post_629_structs {
            flags |= Self::FLAG_STRUCTS_POST_629;
        }

        Self {
            flags,
            compatible_versions: [PluginDeclarationVersionNumber(0); 16],
        }
    }

    #[inline(always)]
    pub const fn from_compatible_versions(versions: &[Version]) -> Self {
        assert!(versions.len() <= Self::MAX_COMPATIBLE_VERSIONS);

        let mut compatible_versions = [PluginDeclarationVersionNumber(0); 16];
        let mut i = 0;
        while i < versions.len() {
            compatible_versions[i] = PluginDeclarationVersionNumber::from_version(versions[i]);
            i += 1;
        }

        Self {
            flags: 0,
            compatible_versions,
        }
    }

    #[inline(always)]
    pub const fn uses_address_library(&self) -> bool {
        (self.flags & Self::FLAG_ADDRESS_LIBRARY) != 0
    }

    #[inline(always)]
    pub const fn uses_signature_scanning(&self) -> bool {
        (self.flags & Self::FLAG_SIGNATURE_SCANNING) != 0
    }

    #[inline(always)]
    pub const fn targets_629_structs(&self) -> bool {
        (self.flags & Self::FLAG_STRUCTS_POST_629) != 0
    }

    #[inline(always)]
    pub const fn is_version_independent(&self) -> bool {
        self.uses_address_library() || self.uses_signature_scanning()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginDeclarationInfo {
    pub version: PluginDeclarationVersionNumber,
    pub name: PluginDeclarationString<256>,
    pub author: PluginDeclarationString<256>,
    pub support_email: PluginDeclarationString<252>,
    pub struct_compatibility: StructCompatibility,
    pub runtime_compatibility: RuntimeCompatibility,
    pub minimum_skse_version: PluginDeclarationVersionNumber,
}

const _: () = assert!(core::mem::size_of::<PluginDeclarationInfo>() == 0x34C);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, version) == 0x000);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, name) == 0x004);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, author) == 0x104);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, support_email) == 0x204);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, struct_compatibility) == 0x300);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, runtime_compatibility) == 0x304);
const _: () = assert!(core::mem::offset_of!(PluginDeclarationInfo, minimum_skse_version) == 0x348);

impl Default for PluginDeclarationInfo {
    #[inline(always)]
    fn default() -> Self {
        Self {
            version: PluginDeclarationVersionNumber::new(1, 0, 0, 0),
            name: PluginDeclarationString::new(),
            author: PluginDeclarationString::new(),
            support_email: PluginDeclarationString::new(),
            struct_compatibility: StructCompatibility::Independent,
            runtime_compatibility: RuntimeCompatibility::new(),
            minimum_skse_version: PluginDeclarationVersionNumber(0),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginDeclaration {
    pub data_version: u32,
    pub data: PluginDeclarationInfo,
}

const _: () = assert!(core::mem::size_of::<PluginDeclaration>() == 0x350);

impl PluginDeclaration {
    pub const VERSION: u32 = 1;

    #[inline(always)]
    pub const fn new(info: PluginDeclarationInfo) -> Self {
        Self {
            data_version: Self::VERSION,
            data: info,
        }
    }

    #[inline(always)]
    pub fn get_version(&self) -> Version {
        self.data.version.get()
    }

    #[inline(always)]
    pub fn get_name_ptr(&self) -> *const c_char {
        self.data.name.as_ptr()
    }

    #[inline(always)]
    pub fn get_author_ptr(&self) -> *const c_char {
        self.data.author.as_ptr()
    }

    #[inline(always)]
    pub fn get_support_email_ptr(&self) -> *const c_char {
        self.data.support_email.as_ptr()
    }

    #[inline(always)]
    pub fn get_struct_compatibility(&self) -> StructCompatibility {
        self.data.struct_compatibility
    }

    #[inline(always)]
    pub fn get_runtime_compatibility(&self) -> &RuntimeCompatibility {
        &self.data.runtime_compatibility
    }

    #[inline(always)]
    pub fn get_minimum_skse_version(&self) -> Version {
        self.data.minimum_skse_version.get()
    }

    #[inline(always)]
    pub fn get_singleton() -> *const Self {
        core::ptr::addr_of!(crate::SKSEPlugin_Version).cast()
    }
}

#[macro_export]
macro_rules! plugin_declaration {
    (
        version: $version:expr,
        name: $name:expr,
        author: $author:expr,
        support_email: $support_email:expr,
        struct_compatibility: $struct_compatibility:expr,
        runtime_compatibility: $runtime_compatibility:expr,
        minimum_skse_version: $minimum_skse_version:expr $(,)?
    ) => {
        #[unsafe(no_mangle)]
        pub static SKSEPlugin_Version: $crate::skse::PluginDeclaration =
            $crate::skse::PluginDeclaration::new($crate::skse::PluginDeclarationInfo {
                version: $crate::skse::PluginDeclarationVersionNumber::from_version($version),
                name: $crate::skse::PluginDeclarationString::from_str($name),
                author: $crate::skse::PluginDeclarationString::from_str($author),
                support_email: $crate::skse::PluginDeclarationString::from_str($support_email),
                struct_compatibility: $struct_compatibility,
                runtime_compatibility: $runtime_compatibility,
                minimum_skse_version: $crate::skse::PluginDeclarationVersionNumber::from_version(
                    $minimum_skse_version,
                ),
            });
    };
}
pub use plugin_declaration;

pub const RUNTIME_SSE_1_1_47: Version = Version::new(1, 1, 47, 0);
pub const RUNTIME_SSE_1_1_51: Version = Version::new(1, 1, 51, 0);
pub const RUNTIME_SSE_1_2_36: Version = Version::new(1, 2, 36, 0);
pub const RUNTIME_SSE_1_2_39: Version = Version::new(1, 2, 39, 0);
pub const RUNTIME_SSE_1_3_5: Version = Version::new(1, 3, 5, 0);
pub const RUNTIME_SSE_1_3_9: Version = Version::new(1, 3, 9, 0);
pub const RUNTIME_SSE_1_4_2: Version = Version::new(1, 4, 2, 0);
pub const RUNTIME_SSE_1_5_3: Version = Version::new(1, 5, 3, 0);
pub const RUNTIME_SSE_1_5_16: Version = Version::new(1, 5, 16, 0);
pub const RUNTIME_SSE_1_5_23: Version = Version::new(1, 5, 23, 0);
pub const RUNTIME_SSE_1_5_39: Version = Version::new(1, 5, 39, 0);
pub const RUNTIME_SSE_1_5_50: Version = Version::new(1, 5, 50, 0);
pub const RUNTIME_SSE_1_5_53: Version = Version::new(1, 5, 53, 0);
pub const RUNTIME_SSE_1_5_62: Version = Version::new(1, 5, 62, 0);
pub const RUNTIME_SSE_1_5_73: Version = Version::new(1, 5, 73, 0);
pub const RUNTIME_SSE_1_5_80: Version = Version::new(1, 5, 80, 0);
pub const RUNTIME_SSE_1_5_97: Version = Version::new(1, 5, 97, 0);
pub const RUNTIME_SSE_1_6_317: Version = Version::new(1, 6, 317, 0);
pub const RUNTIME_SSE_1_6_318: Version = Version::new(1, 6, 318, 0);
pub const RUNTIME_SSE_1_6_323: Version = Version::new(1, 6, 323, 0);
pub const RUNTIME_SSE_1_6_342: Version = Version::new(1, 6, 342, 0);
pub const RUNTIME_SSE_1_6_353: Version = Version::new(1, 6, 353, 0);
pub const RUNTIME_SSE_1_6_629: Version = Version::new(1, 6, 629, 0);
pub const RUNTIME_SSE_1_6_640: Version = Version::new(1, 6, 640, 0);
pub const RUNTIME_SSE_1_6_659: Version = Version::new(1, 6, 659, 0);
pub const RUNTIME_SSE_1_6_678: Version = Version::new(1, 6, 678, 0);
pub const RUNTIME_SSE_1_6_1130: Version = Version::new(1, 6, 1130, 0);
pub const RUNTIME_SSE_1_6_1170: Version = Version::new(1, 6, 1170, 0);
pub const RUNTIME_1_6_1179: Version = Version::new(1, 6, 1179, 0);

pub const RUNTIME_SSE_LATEST_AE: Version = RUNTIME_SSE_1_6_1170;
pub const RUNTIME_SSE_LATEST_SE: Version = RUNTIME_SSE_1_5_97;
pub const RUNTIME_SSE_LATEST: Version = RUNTIME_SSE_LATEST_AE;

pub const RUNTIME_VR_1_4_15: Version = Version::new(1, 4, 15, 0);
pub const RUNTIME_LATEST_VR: Version = RUNTIME_VR_1_4_15;
