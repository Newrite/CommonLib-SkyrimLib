/// C++ `RE::BSScript::TypeInfo::RawType`
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawType {
    None = 0,
    Object = 1,
    String = 2,
    Int = 3,
    Float = 4,
    Bool = 5,

    NoneArray = 10,
    ObjectArray = 11,
    StringArray = 12,
    IntArray = 13,
    FloatArray = 14,
    BoolArray = 15,

    ArraysEnd = 16,
}

impl TryFrom<usize> for RawType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Object),
            2 => Ok(Self::String),
            3 => Ok(Self::Int),
            4 => Ok(Self::Float),
            5 => Ok(Self::Bool),
            10 => Ok(Self::NoneArray),
            11 => Ok(Self::ObjectArray),
            12 => Ok(Self::StringArray),
            13 => Ok(Self::IntArray),
            14 => Ok(Self::FloatArray),
            15 => Ok(Self::BoolArray),
            16 => Ok(Self::ArraysEnd),
            _ => Err(()),
        }
    }
}

core_util::impl_enumset_type!(RawType => usize);

/// C++ `RE::BSScript::TypeInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeInfo {
    pub raw_type: core_util::EnumSet<RawType, usize>, // 00
}

const _: () = assert!(core::mem::size_of::<TypeInfo>() == 0x8);

impl TypeInfo {
    #[inline(always)]
    pub const fn new(raw_type: core_util::EnumSet<RawType, usize>) -> Self {
        Self { raw_type }
    }

    #[inline(always)]
    pub fn get_raw_type(self) -> Option<RawType> {
        self.raw_type.get()
    }

    #[inline(always)]
    pub fn is_array(self) -> bool {
        matches!(
            self.get_raw_type(),
            Some(
                RawType::NoneArray
                    | RawType::ObjectArray
                    | RawType::StringArray
                    | RawType::IntArray
                    | RawType::FloatArray
                    | RawType::BoolArray
            )
        )
    }
}
