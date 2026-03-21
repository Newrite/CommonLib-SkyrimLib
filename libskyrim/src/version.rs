use core::fmt;

/// Идентификаторы платформ/магазинов (SKSE использует 4-е число версии для этого)
pub const RUNTIME_TYPE_BETHESDA: u16 = 0;
pub const RUNTIME_TYPE_VR: u16       = 1;
pub const RUNTIME_TYPE_GOG: u16      = 1; // VR и GOG делят один ID, нужно отличать по версии
pub const RUNTIME_TYPE_EPIC: u16     = 2;

/// Единый класс версии (заменяет и SkseVersion, и REL::Version)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self(
            ((major as u32 & 0xFF) << 24) |
                ((minor as u32 & 0xFF) << 16) |
                ((patch as u32 & 0xFFF) << 4) |
                (build as u32 & 0xF)
        )
    }

    pub const fn from_packed(packed: u32) -> Self {
        Self(packed)
    }

    pub const fn pack(&self) -> u32 { self.0 }

    pub const fn major(&self) -> u16 { (self.0 >> 24) as u16 }
    pub const fn minor(&self) -> u16 { ((self.0 >> 16) & 0xFF) as u16 }
    pub const fn patch(&self) -> u16 { ((self.0 >> 4) & 0xFFF) as u16 }
    pub const fn build(&self) -> u16 { (self.0 & 0xF) as u16 }

    pub fn save_folder(&self) -> &'static str {
        // Если minor == 4, это Skyrim VR (VR и GOG делят build = 1, так что проверяем minor)
        if self.minor() == 4 {
            return "Skyrim VR";
        }

        // Для остальных проверяем 4-е число (build), где SKSE хранит маркер магазина
        match self.build() {
            RUNTIME_TYPE_EPIC => "Skyrim Special Edition EPIC",
            RUNTIME_TYPE_GOG  => "Skyrim Special Edition GOG",
            _                 => "Skyrim Special Edition", // По умолчанию Steam (Bethesda)
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.major(), self.minor(), self.patch(), self.build())
    }
}

// ─── КОНСТАНТЫ ВЕРСИЙ (из SKSE Version.h) ────────────────────────────────────
pub const RUNTIME_SSE_1_5_39: Version = Version::new(1, 5, 39, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_5_97: Version = Version::new(1, 5, 97, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_318: Version = Version::new(1, 6, 318, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_353: Version = Version::new(1, 6, 353, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_629: Version = Version::new(1, 6, 629, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_640: Version = Version::new(1, 6, 640, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_659_GOG: Version = Version::new(1, 6, 659, RUNTIME_TYPE_GOG);
pub const RUNTIME_SSE_1_6_678_EPIC: Version = Version::new(1, 6, 678, RUNTIME_TYPE_EPIC);
pub const RUNTIME_SSE_1_6_1130: Version = Version::new(1, 6, 1130, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_1170: Version = Version::new(1, 6, 1170, RUNTIME_TYPE_BETHESDA);
pub const RUNTIME_SSE_1_6_1179_GOG: Version = Version::new(1, 6, 1179, RUNTIME_TYPE_GOG);

pub const RUNTIME_VR_1_4_15: Version = Version::new(1, 4, 15, RUNTIME_TYPE_VR);

pub const RUNTIME_LATEST_SE: Version = RUNTIME_SSE_1_5_97;
pub const RUNTIME_LATEST_AE: Version = RUNTIME_SSE_1_6_1170;
pub const RUNTIME_LATEST_VR: Version = RUNTIME_VR_1_4_15;