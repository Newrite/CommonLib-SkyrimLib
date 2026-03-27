use alloc::string::{String, ToString};

use crate::re::object_type_info::ObjectTypeInfo;

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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeInfo {
    pub raw_type: core_util::EnumSet<RawType, usize>, // 00
}

const _: () = assert!(core::mem::size_of::<TypeInfo>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TypeInfo, raw_type) == 0x00);

impl TypeInfo {
    #[inline(always)]
    pub const fn new(raw_type: RawType) -> Self {
        Self {
            raw_type: core_util::EnumSet::from_underlying(raw_type as usize),
        }
    }

    #[inline(always)]
    pub const fn from_underlying(raw_type: usize) -> Self {
        Self {
            raw_type: core_util::EnumSet::from_underlying(raw_type),
        }
    }

    #[inline(always)]
    pub const fn get_raw_type(&self) -> core_util::EnumSet<RawType, usize> {
        self.raw_type
    }

    #[inline(always)]
    pub fn get_type_info(&self) -> *mut ObjectTypeInfo {
        debug_assert!(self.is_object() || self.is_object_array());
        let raw = self.raw_type.underlying();
        if self.is_object() {
            (raw & !(RawType::Object as usize)) as *mut ObjectTypeInfo
        } else {
            (raw & !(RawType::ObjectArray as usize)) as *mut ObjectTypeInfo
        }
    }

    #[inline(always)]
    pub fn get_unmangled_raw_type(&self) -> RawType {
        let raw = self.raw_type.underlying();
        if raw < RawType::ArraysEnd as usize {
            self.raw_type.get().unwrap_or(RawType::None)
        } else if self.raw_type.all(RawType::Object) {
            RawType::ObjectArray
        } else {
            RawType::Object
        }
    }

    #[inline(always)]
    pub fn is_array(&self) -> bool {
        self.is_literal_array() || self.is_object_array()
    }

    #[inline(always)]
    pub fn is_bool(&self) -> bool {
        self.raw_type == RawType::Bool
    }

    #[inline(always)]
    pub fn is_float(&self) -> bool {
        self.raw_type == RawType::Float
    }

    #[inline(always)]
    pub fn is_int(&self) -> bool {
        self.raw_type == RawType::Int
    }

    #[inline(always)]
    pub fn is_literal_array(&self) -> bool {
        matches!(
            self.raw_type.get(),
            Some(
                RawType::StringArray | RawType::IntArray | RawType::FloatArray | RawType::BoolArray
            )
        )
    }

    #[inline(always)]
    pub fn is_none_array(&self) -> bool {
        self.raw_type == RawType::NoneArray
    }

    #[inline(always)]
    pub fn is_none_object(&self) -> bool {
        self.raw_type == RawType::None
    }

    #[inline(always)]
    pub fn is_object(&self) -> bool {
        self.get_unmangled_raw_type() == RawType::Object
    }

    #[inline(always)]
    pub fn is_object_array(&self) -> bool {
        self.raw_type.underlying() >= RawType::ArraysEnd as usize
            && self.raw_type.all(RawType::Object)
    }

    #[inline(always)]
    pub fn is_string(&self) -> bool {
        self.raw_type == RawType::String
    }

    #[inline(always)]
    pub fn type_as_string(&self) -> String {
        match self.get_unmangled_raw_type() {
            RawType::None => "none".into(),
            RawType::String => "string".into(),
            RawType::Int => "int".into(),
            RawType::Float => "float".into(),
            RawType::Bool => "bool".into(),
            RawType::Object => {
                let info = self.get_type_info();
                if info.is_null() {
                    "none".into()
                } else {
                    unsafe { (*info).name.to_string() }
                }
            }
            RawType::NoneArray => "none".into(),
            RawType::ObjectArray => {
                let info = self.get_type_info();
                if info.is_null() {
                    "none".into()
                } else {
                    let mut s = unsafe { (*info).name.to_string() };
                    s.push_str("[]");
                    s
                }
            }
            RawType::StringArray => "string[]".into(),
            RawType::IntArray => "int[]".into(),
            RawType::FloatArray => "float[]".into(),
            RawType::BoolArray => "bool[]".into(),
            RawType::ArraysEnd => String::new(),
        }
    }

    #[inline(always)]
    pub fn set_type(&mut self, raw_type: RawType) {
        self.raw_type = raw_type.into();
    }
}
