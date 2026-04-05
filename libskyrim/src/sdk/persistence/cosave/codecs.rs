//! Standard `CosaveEncode` / `CosaveDecode` implementations for common payload
//! shapes.
//!
//! This module is intentionally boring: it mirrors the byte layout implied by
//! [`RecordWriter`](super::RecordWriter) and [`RecordReader`](super::RecordReader)
//! so most plugin payloads can derive or compose codecs without custom binary
//! glue.
//!
//! Patterns to reach for:
//!
//! - primitive numbers and `bool` map directly onto the matching `write_*` /
//!   `read_*` helpers;
//! - `Option<T>` encodes as presence-bool plus nested value;
//! - `Vec<T>` and [`BoundedVec<T, N>`](super::BoundedVec) encode as `u32`
//!   length plus ordered elements;
//! - [`ResolvedFormId`](super::ResolvedFormId) and
//!   [`ResolvedVmHandle`](super::ResolvedVmHandle) are the right wrappers when
//!   on-disk identifiers must be re-resolved through [`super::LoadContext`];
//! - ordered maps use `BTreeMap` so save and load order stays deterministic.
//!
//! If a payload shape does not match these defaults, implement
//! [`CosaveEncode`](super::CosaveEncode) / [`CosaveDecode`](super::CosaveDecode)
//! manually and use the contextual helpers on [`SaveError`](super::SaveError)
//! and [`LoadError`](super::LoadError) to keep diagnostics readable.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::io::{CosaveDecode, CosaveEncode, RecordReader, RecordWriter};
use super::types::{
    BoundedVec, LoadError, ResolvedFormId, ResolvedVmHandle, SaveError, usize_from_u32,
};

/// Numeric primitive codecs mirrored directly to the writer/reader helpers.
macro_rules! impl_number_codec {
    ($ty:ty, $write:ident, $read:ident) => {
        impl CosaveEncode for $ty {
            #[inline(always)]
            fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
                writer.$write(*self);
                Ok(())
            }
        }

        impl CosaveDecode for $ty {
            #[inline(always)]
            fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
                reader.$read()
            }
        }
    };
}

impl_number_codec!(u8, write_u8, read_u8);
impl_number_codec!(i8, write_i8, read_i8);
impl_number_codec!(u16, write_u16, read_u16);
impl_number_codec!(i16, write_i16, read_i16);
impl_number_codec!(u32, write_u32, read_u32);
impl_number_codec!(i32, write_i32, read_i32);
impl_number_codec!(u64, write_u64, read_u64);
impl_number_codec!(i64, write_i64, read_i64);
impl_number_codec!(f32, write_f32, read_f32);
impl_number_codec!(f64, write_f64, read_f64);

/// Bool is encoded as a single `u8` using `0` and `1`.
impl CosaveEncode for bool {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_bool(*self);
        Ok(())
    }
}

impl CosaveDecode for bool {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_bool()
    }
}

/// Option values are encoded as `bool` presence plus the nested value.
impl<T: CosaveEncode> CosaveEncode for Option<T> {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        match self {
            Some(value) => {
                writer.write_bool(true);
                value.encode(writer)?;
            }
            None => writer.write_bool(false),
        }
        Ok(())
    }
}

impl<T: CosaveDecode> CosaveDecode for Option<T> {
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        if reader.read_bool()? {
            Ok(Some(T::decode(reader)?))
        } else {
            Ok(None)
        }
    }
}

/// Strings are encoded as length-prefixed UTF-8.
impl CosaveEncode for str {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_string(self)
    }
}

impl CosaveEncode for String {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        self.as_str().encode(writer)
    }
}

impl CosaveDecode for String {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_string()
    }
}

/// Fixed-size arrays are encoded element-by-element without a leading length.
impl<T: CosaveEncode, const N: usize> CosaveEncode for [T; N] {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        for (index, value) in self.iter().enumerate() {
            value
                .encode(writer)
                .map_err(|source| SaveError::sequence_element(index, source))?;
        }
        Ok(())
    }
}

impl<T: CosaveDecode, const N: usize> CosaveDecode for [T; N] {
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        let mut values = Vec::with_capacity(N);
        for index in 0..N {
            values.push(
                T::decode(reader).map_err(|source| LoadError::sequence_element(index, source))?,
            );
        }

        match values.try_into() {
            Ok(array) => Ok(array),
            Err(_) => unreachable!("decoded fixed-size array should contain exactly {N} items"),
        }
    }
}

/// Resolved form IDs encode as their already-resolved runtime value.
impl CosaveEncode for ResolvedFormId {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_form_id(self.get());
        Ok(())
    }
}

impl CosaveDecode for ResolvedFormId {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_resolved_form_id()
    }
}

/// Resolved VM handles encode as their already-resolved runtime value.
impl CosaveEncode for ResolvedVmHandle {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_vm_handle(self.get());
        Ok(())
    }
}

impl CosaveDecode for ResolvedVmHandle {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_resolved_vm_handle()
    }
}

/// Vectors are encoded as `u32` length plus each element in order.
impl<T: CosaveEncode> CosaveEncode for Vec<T> {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_len(self.len())?;
        for (index, value) in self.iter().enumerate() {
            value
                .encode(writer)
                .map_err(|source| SaveError::sequence_element(index, source))?;
        }
        Ok(())
    }
}

impl<T: CosaveDecode> CosaveDecode for Vec<T> {
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        let length = reader.read_len()?;
        let capacity = usize_from_u32(length)?;
        let mut values = Vec::with_capacity(capacity);
        for index in 0..capacity {
            values.push(
                T::decode(reader).map_err(|source| LoadError::sequence_element(index, source))?,
            );
        }
        Ok(values)
    }
}

/// Bounded vectors preserve their maximum length during decode.
impl<T: CosaveEncode, const N: u32> CosaveEncode for BoundedVec<T, N> {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        if self.len() > Self::max_len() {
            return Err(SaveError::BoundExceeded {
                len: self.len(),
                max: Self::max_len(),
            });
        }

        writer.write_len(self.len())?;
        for (index, value) in self.as_slice().iter().enumerate() {
            value
                .encode(writer)
                .map_err(|source| SaveError::sequence_element(index, source))?;
        }
        Ok(())
    }
}

impl<T: CosaveDecode, const N: u32> CosaveDecode for BoundedVec<T, N> {
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        let length = reader.read_len()?;
        if length > N {
            return Err(LoadError::VectorTooLong {
                len: length,
                max: N,
            });
        }

        let capacity = usize_from_u32(length)?;
        let mut values = Vec::with_capacity(capacity);
        for index in 0..capacity {
            values.push(
                T::decode(reader).map_err(|source| LoadError::sequence_element(index, source))?,
            );
        }

        Self::try_from_vec(values).map_err(|error| LoadError::VectorTooLong {
            len: error.len() as u32,
            max: error.max() as u32,
        })
    }
}

/// Ordered maps are encoded as a length plus key/value pairs in iteration order.
impl<K, V> CosaveEncode for BTreeMap<K, V>
where
    K: Ord + CosaveEncode,
    V: CosaveEncode,
{
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_len(self.len())?;
        for (entry, (key, value)) in self.iter().enumerate() {
            key.encode(writer)
                .map_err(|source| SaveError::map_key(entry, source))?;
            value
                .encode(writer)
                .map_err(|source| SaveError::map_value(entry, source))?;
        }
        Ok(())
    }
}

impl<K, V> CosaveDecode for BTreeMap<K, V>
where
    K: Ord + CosaveDecode,
    V: CosaveDecode,
{
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        let length = reader.read_len()?;
        let mut map = BTreeMap::new();
        for entry in 0..usize_from_u32(length)? {
            let key = K::decode(reader).map_err(|source| LoadError::map_key(entry, source))?;
            let value = V::decode(reader).map_err(|source| LoadError::map_value(entry, source))?;
            if map.insert(key, value).is_some() {
                return Err(LoadError::DuplicateMapKey);
            }
        }
        Ok(map)
    }
}
