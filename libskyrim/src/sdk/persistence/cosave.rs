//! Typed cosave helpers layered over the low-level SKSE serialization ABI.
//!
//! This module provides a length-bounded reader/writer, strongly typed record
//! identifiers, and reusable codecs for common plugin data containers.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

use crate::re::bs_core_types::{FormID, VMHandle};
use crate::skse::SerializationInterface;

/// Compile-time checked four-character record ID literal.
#[macro_export]
macro_rules! record_id {
    ($value:literal) => {
        $crate::sdk::persistence::cosave::RecordId::from_raw(
            $crate::sdk::persistence::cosave::fourcc_from_str($value),
        )
    };
}

/// Compile-time checked four-character plugin unique ID literal.
#[macro_export]
macro_rules! unique_id {
    ($value:literal) => {
        $crate::sdk::persistence::cosave::UniqueId::from_raw(
            $crate::sdk::persistence::cosave::fourcc_from_str($value),
        )
    };
}

pub use crate::{record_id, unique_id};

#[inline(always)]
pub const fn fourcc_from_str(value: &str) -> u32 {
    let bytes = value.as_bytes();
    if bytes.len() != 4 {
        panic!("fourcc literals must contain exactly 4 bytes");
    }

    let mut i = 0;
    while i < 4 {
        if bytes[i] > 0x7F {
            panic!("fourcc literals must be ASCII");
        }
        i += 1;
    }

    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedVecError {
    len: usize,
    max: usize,
}

impl BoundedVecError {
    #[inline(always)]
    pub const fn new(len: usize, max: usize) -> Self {
        Self { len, max }
    }

    #[inline(always)]
    pub const fn len(self) -> usize {
        self.len
    }

    #[inline(always)]
    pub const fn max(self) -> usize {
        self.max
    }
}

impl fmt::Display for BoundedVecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bounded vector length {} exceeds maximum {}",
            self.len, self.max
        )
    }
}

impl core::error::Error for BoundedVecError {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoundedVec<T, const N: u32> {
    inner: Vec<T>,
}

impl<T, const N: u32> BoundedVec<T, N> {
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Result<Self, BoundedVecError> {
        if capacity > Self::max_len() {
            return Err(BoundedVecError::new(capacity, Self::max_len()));
        }

        Ok(Self {
            inner: Vec::with_capacity(capacity),
        })
    }

    #[inline(always)]
    pub const fn max_len_u32() -> u32 {
        N
    }

    #[inline(always)]
    pub const fn max_len() -> usize {
        N as usize
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), BoundedVecError> {
        if self.inner.len() >= Self::max_len() {
            return Err(BoundedVecError::new(self.inner.len() + 1, Self::max_len()));
        }

        self.inner.push(value);
        Ok(())
    }

    #[inline(always)]
    pub fn try_extend<I>(&mut self, iter: I) -> Result<(), BoundedVecError>
    where
        I: IntoIterator<Item = T>,
    {
        for value in iter {
            self.push(value)?;
        }

        Ok(())
    }

    #[inline(always)]
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    #[inline(always)]
    pub fn try_from_vec(vec: Vec<T>) -> Result<Self, BoundedVecError> {
        if vec.len() > Self::max_len() {
            return Err(BoundedVecError::new(vec.len(), Self::max_len()));
        }

        Ok(Self { inner: vec })
    }
}

impl<T: fmt::Debug, const N: u32> fmt::Debug for BoundedVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BoundedVec").field(&self.inner).finish()
    }
}

impl<T, const N: u32> Deref for BoundedVec<T, N> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.inner.as_slice()
    }
}

impl<T, const N: u32> DerefMut for BoundedVec<T, N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut_slice()
    }
}

impl<T, const N: u32> AsRef<[T]> for BoundedVec<T, N> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.inner.as_slice()
    }
}

impl<T, const N: u32> AsMut<[T]> for BoundedVec<T, N> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }
}

impl<T, const N: u32> TryFrom<Vec<T>> for BoundedVec<T, N> {
    type Error = BoundedVecError;

    #[inline(always)]
    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_vec(value)
    }
}

impl<T, const N: u32> From<BoundedVec<T, N>> for Vec<T> {
    #[inline(always)]
    fn from(value: BoundedVec<T, N>) -> Self {
        value.inner
    }
}

impl<T, const N: u32> IntoIterator for BoundedVec<T, N> {
    type IntoIter = alloc::vec::IntoIter<T>;
    type Item = T;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T, const N: u32> IntoIterator for &'a BoundedVec<T, N> {
    type IntoIter = core::slice::Iter<'a, T>;
    type Item = &'a T;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, T, const N: u32> IntoIterator for &'a mut BoundedVec<T, N> {
    type IntoIter = core::slice::IterMut<'a, T>;
    type Item = &'a mut T;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RecordId(u32);

impl RecordId {
    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_fourcc(f, self.0)
    }
}

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_fourcc(f, self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UniqueId(u32);

impl UniqueId {
    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub const fn to_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_fourcc(f, self.0)
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_fourcc(f, self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader {
    id: RecordId,
    version: u32,
    length: u32,
}

impl RecordHeader {
    #[inline(always)]
    pub const fn new(id: RecordId, version: u32, length: u32) -> Self {
        Self {
            id,
            version,
            length,
        }
    }

    #[inline(always)]
    pub const fn id(self) -> RecordId {
        self.id
    }

    #[inline(always)]
    pub const fn version(self) -> u32 {
        self.version
    }

    #[inline(always)]
    pub const fn length(self) -> u32 {
        self.length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ResolvedFormId(FormID);

impl ResolvedFormId {
    #[inline(always)]
    pub const fn new(form_id: FormID) -> Self {
        Self(form_id)
    }

    #[inline(always)]
    pub const fn get(self) -> FormID {
        self.0
    }

    #[inline(always)]
    pub const fn into_inner(self) -> FormID {
        self.0
    }
}

impl From<FormID> for ResolvedFormId {
    #[inline(always)]
    fn from(value: FormID) -> Self {
        Self::new(value)
    }
}

impl From<ResolvedFormId> for FormID {
    #[inline(always)]
    fn from(value: ResolvedFormId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ResolvedVmHandle(VMHandle);

impl ResolvedVmHandle {
    #[inline(always)]
    pub const fn new(handle: VMHandle) -> Self {
        Self(handle)
    }

    #[inline(always)]
    pub const fn get(self) -> VMHandle {
        self.0
    }

    #[inline(always)]
    pub const fn into_inner(self) -> VMHandle {
        self.0
    }
}

impl From<VMHandle> for ResolvedVmHandle {
    #[inline(always)]
    fn from(value: VMHandle) -> Self {
        Self::new(value)
    }
}

impl From<ResolvedVmHandle> for VMHandle {
    #[inline(always)]
    fn from(value: ResolvedVmHandle) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
    LengthOverflow(usize),
    BoundExceeded {
        len: usize,
        max: usize,
    },
    OpenRecordFailed {
        id: RecordId,
        version: u32,
    },
    WriteRecordFailed {
        id: RecordId,
        version: u32,
    },
    Field {
        field: &'static str,
        source: Box<SaveError>,
    },
    SequenceElement {
        index: usize,
        source: Box<SaveError>,
    },
    MapKey {
        entry: usize,
        source: Box<SaveError>,
    },
    MapValue {
        entry: usize,
        source: Box<SaveError>,
    },
}

impl SaveError {
    #[inline(always)]
    pub fn field(field: &'static str, source: SaveError) -> Self {
        Self::Field {
            field,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn sequence_element(index: usize, source: SaveError) -> Self {
        Self::SequenceElement {
            index,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn map_key(entry: usize, source: SaveError) -> Self {
        Self::MapKey {
            entry,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn map_value(entry: usize, source: SaveError) -> Self {
        Self::MapValue {
            entry,
            source: Box::new(source),
        }
    }
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthOverflow(len) => write!(f, "serialized length {} exceeds u32::MAX", len),
            Self::BoundExceeded { len, max } => {
                write!(f, "bounded value length {} exceeds maximum {}", len, max)
            }
            Self::OpenRecordFailed { id, version } => {
                write!(f, "failed to open cosave record {} version {}", id, version)
            }
            Self::WriteRecordFailed { id, version } => {
                write!(
                    f,
                    "failed to write cosave record {} version {}",
                    id, version
                )
            }
            Self::Field { field, source } => {
                write!(f, "failed to encode field `{}`: {}", field, source)
            }
            Self::SequenceElement { index, source } => {
                write!(f, "failed to encode sequence element {}: {}", index, source)
            }
            Self::MapKey { entry, source } => {
                write!(f, "failed to encode map key at entry {}: {}", entry, source)
            }
            Self::MapValue { entry, source } => {
                write!(
                    f,
                    "failed to encode map value at entry {}: {}",
                    entry, source
                )
            }
        }
    }
}

impl core::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Field { source, .. }
            | Self::SequenceElement { source, .. }
            | Self::MapKey { source, .. }
            | Self::MapValue { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    LengthOverflow(u32),
    ReadRecordFailed {
        id: RecordId,
        expected: u32,
        actual: u32,
    },
    UnexpectedEof {
        requested: usize,
        remaining: usize,
    },
    TrailingBytes {
        remaining: usize,
    },
    InvalidUtf8(core::str::Utf8Error),
    InvalidBool(u8),
    VectorTooLong {
        len: u32,
        max: u32,
    },
    DuplicateMapKey,
    MissingSerializationContext(&'static str),
    ResolveFormIdFailed(FormID),
    ResolveHandleFailed(VMHandle),
    Field {
        field: &'static str,
        source: Box<LoadError>,
    },
    SequenceElement {
        index: usize,
        source: Box<LoadError>,
    },
    MapKey {
        entry: usize,
        source: Box<LoadError>,
    },
    MapValue {
        entry: usize,
        source: Box<LoadError>,
    },
}

impl LoadError {
    #[inline(always)]
    pub fn field(field: &'static str, source: LoadError) -> Self {
        Self::Field {
            field,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn sequence_element(index: usize, source: LoadError) -> Self {
        Self::SequenceElement {
            index,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn map_key(entry: usize, source: LoadError) -> Self {
        Self::MapKey {
            entry,
            source: Box::new(source),
        }
    }

    #[inline(always)]
    pub fn map_value(entry: usize, source: LoadError) -> Self {
        Self::MapValue {
            entry,
            source: Box::new(source),
        }
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthOverflow(len) => write!(f, "serialized length {} does not fit usize", len),
            Self::ReadRecordFailed {
                id,
                expected,
                actual,
            } => write!(
                f,
                "failed to read cosave record {}: expected {} bytes, got {}",
                id, expected, actual
            ),
            Self::UnexpectedEof {
                requested,
                remaining,
            } => write!(
                f,
                "unexpected end of cosave record: requested {} bytes with {} remaining",
                requested, remaining
            ),
            Self::TrailingBytes { remaining } => {
                write!(f, "cosave record decode left {} unread bytes", remaining)
            }
            Self::InvalidUtf8(error) => write!(f, "invalid UTF-8 in cosave string: {}", error),
            Self::InvalidBool(value) => write!(f, "invalid serialized bool value {}", value),
            Self::VectorTooLong { len, max } => write!(
                f,
                "serialized collection length {} exceeds maximum {}",
                len, max
            ),
            Self::DuplicateMapKey => write!(f, "duplicate key encountered while decoding map"),
            Self::MissingSerializationContext(operation) => write!(
                f,
                "serialization interface is required for {} but no load context was provided",
                operation
            ),
            Self::ResolveFormIdFailed(form_id) => {
                write!(f, "failed to resolve saved FormID 0x{:08X}", form_id)
            }
            Self::ResolveHandleFailed(handle) => {
                write!(f, "failed to resolve saved VMHandle 0x{:016X}", handle)
            }
            Self::Field { field, source } => {
                write!(f, "failed to decode field `{}`: {}", field, source)
            }
            Self::SequenceElement { index, source } => {
                write!(f, "failed to decode sequence element {}: {}", index, source)
            }
            Self::MapKey { entry, source } => {
                write!(f, "failed to decode map key at entry {}: {}", entry, source)
            }
            Self::MapValue { entry, source } => {
                write!(
                    f,
                    "failed to decode map value at entry {}: {}",
                    entry, source
                )
            }
        }
    }
}

impl core::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::InvalidUtf8(error) => Some(error),
            Self::Field { source, .. }
            | Self::SequenceElement { source, .. }
            | Self::MapKey { source, .. }
            | Self::MapValue { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct LoadContext<'a> {
    serialization: Option<&'a SerializationInterface>,
}

impl<'a> LoadContext<'a> {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            serialization: None,
        }
    }

    #[inline(always)]
    pub const fn new(serialization: &'a SerializationInterface) -> Self {
        Self {
            serialization: Some(serialization),
        }
    }

    #[inline(always)]
    pub const fn serialization(self) -> Option<&'a SerializationInterface> {
        self.serialization
    }

    pub fn resolve_form_id(self, old_form_id: FormID) -> Result<FormID, LoadError> {
        let Some(serialization) = self.serialization else {
            return Err(LoadError::MissingSerializationContext("FormID resolution"));
        };

        let mut resolved = 0 as FormID;
        if !serialization.resolve_form_id(old_form_id, &mut resolved) {
            return Err(LoadError::ResolveFormIdFailed(old_form_id));
        }

        Ok(resolved)
    }

    pub fn resolve_handle(self, old_handle: VMHandle) -> Result<VMHandle, LoadError> {
        let Some(serialization) = self.serialization else {
            return Err(LoadError::MissingSerializationContext(
                "VMHandle resolution",
            ));
        };

        let mut resolved = 0 as VMHandle;
        if !serialization.resolve_handle(old_handle, &mut resolved) {
            return Err(LoadError::ResolveHandleFailed(old_handle));
        }

        Ok(resolved)
    }
}

pub trait CosaveEncode {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError>;
}

pub trait CosaveDecode: Sized {
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError>;
}

#[derive(Debug, Clone, Default)]
pub struct RecordWriter {
    bytes: Vec<u8>,
}

impl RecordWriter {
    #[inline(always)]
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    #[inline(always)]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    #[inline(always)]
    pub fn into_record(self, id: RecordId, version: u32) -> Result<OwnedRecord, SaveError> {
        OwnedRecord::new(id, version, self.into_bytes())
    }

    #[inline(always)]
    pub fn write_value<T: CosaveEncode + ?Sized>(&mut self, value: &T) -> Result<(), SaveError> {
        value.encode(self)
    }

    #[inline(always)]
    pub fn write_raw_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    #[inline(always)]
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    #[inline(always)]
    pub fn write_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    #[inline(always)]
    pub fn write_u16(&mut self, value: u16) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_i16(&mut self, value: i16) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_u32(&mut self, value: u32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_form_id(&mut self, value: FormID) {
        self.write_u32(value);
    }

    #[inline(always)]
    pub fn write_i32(&mut self, value: i32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_u64(&mut self, value: u64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_vm_handle(&mut self, value: VMHandle) {
        self.write_u64(value);
    }

    #[inline(always)]
    pub fn write_i64(&mut self, value: i64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_f32(&mut self, value: f32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_f64(&mut self, value: f64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    #[inline(always)]
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(u8::from(value));
    }

    pub fn write_len(&mut self, len: usize) -> Result<(), SaveError> {
        let len = u32::try_from(len).map_err(|_| SaveError::LengthOverflow(len))?;
        self.write_u32(len);
        Ok(())
    }

    pub fn write_string(&mut self, value: &str) -> Result<(), SaveError> {
        self.write_len(value.len())?;
        self.write_raw_bytes(value.as_bytes());
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct RecordReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    context: LoadContext<'a>,
}

impl<'a> RecordReader<'a> {
    #[inline(always)]
    pub fn new(bytes: &'a [u8], context: LoadContext<'a>) -> Self {
        Self {
            bytes,
            cursor: 0,
            context,
        }
    }

    #[inline(always)]
    pub fn context(&self) -> LoadContext<'a> {
        self.context
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline(always)]
    pub fn position(&self) -> usize {
        self.cursor
    }

    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.cursor)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    pub fn finish(&self) -> Result<(), LoadError> {
        let remaining = self.remaining();
        if remaining == 0 {
            return Ok(());
        }

        Err(LoadError::TrailingBytes { remaining })
    }

    pub fn read_exact(&mut self, length: usize) -> Result<&'a [u8], LoadError> {
        let remaining = self.remaining();
        if length > remaining {
            return Err(LoadError::UnexpectedEof {
                requested: length,
                remaining,
            });
        }

        let start = self.cursor;
        let end = start + length;
        self.cursor = end;
        Ok(&self.bytes[start..end])
    }

    #[inline(always)]
    pub fn read_value<T: CosaveDecode>(&mut self) -> Result<T, LoadError> {
        T::decode(self)
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, LoadError> {
        Ok(self.read_exact(1)?[0])
    }

    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, LoadError> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u16>())?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_i16(&mut self) -> Result<i16, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i16>())?;
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_u32(&mut self) -> Result<u32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u32>())?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    #[inline(always)]
    pub fn read_form_id(&mut self) -> Result<FormID, LoadError> {
        self.read_u32()
    }

    pub fn read_i32(&mut self) -> Result<i32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i32>())?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_u64(&mut self) -> Result<u64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u64>())?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    #[inline(always)]
    pub fn read_vm_handle(&mut self) -> Result<VMHandle, LoadError> {
        self.read_u64()
    }

    pub fn read_i64(&mut self) -> Result<i64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i64>())?;
        Ok(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn read_f32(&mut self) -> Result<f32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<f32>())?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub fn read_f64(&mut self) -> Result<f64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<f64>())?;
        Ok(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    pub fn read_bool(&mut self) -> Result<bool, LoadError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(LoadError::InvalidBool(value)),
        }
    }

    #[inline(always)]
    pub fn read_len(&mut self) -> Result<u32, LoadError> {
        self.read_u32()
    }

    pub fn read_string(&mut self) -> Result<String, LoadError> {
        let length = usize_from_u32(self.read_len()?)?;
        let bytes = self.read_exact(length)?;
        let text = core::str::from_utf8(bytes).map_err(LoadError::InvalidUtf8)?;
        Ok(String::from(text))
    }

    #[inline(always)]
    pub fn read_resolved_form_id(&mut self) -> Result<ResolvedFormId, LoadError> {
        Ok(ResolvedFormId::new(
            self.context().resolve_form_id(self.read_form_id()?)?,
        ))
    }

    #[inline(always)]
    pub fn read_resolved_vm_handle(&mut self) -> Result<ResolvedVmHandle, LoadError> {
        Ok(ResolvedVmHandle::new(
            self.context().resolve_handle(self.read_vm_handle()?)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRecord {
    header: RecordHeader,
    bytes: Vec<u8>,
}

impl OwnedRecord {
    pub fn new(id: RecordId, version: u32, bytes: Vec<u8>) -> Result<Self, SaveError> {
        let length =
            u32::try_from(bytes.len()).map_err(|_| SaveError::LengthOverflow(bytes.len()))?;
        Ok(Self {
            header: RecordHeader::new(id, version, length),
            bytes,
        })
    }

    pub fn from_value<T: CosaveEncode>(
        id: RecordId,
        version: u32,
        value: &T,
    ) -> Result<Self, SaveError> {
        let mut writer = RecordWriter::new();
        value.encode(&mut writer)?;
        writer.into_record(id, version)
    }

    #[inline(always)]
    pub const fn header(&self) -> RecordHeader {
        self.header
    }

    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn write_to(&self, serialization: &SerializationInterface) -> Result<(), SaveError> {
        let header = self.header;
        if !serialization.open_record(header.id().raw(), header.version()) {
            return Err(SaveError::OpenRecordFailed {
                id: header.id(),
                version: header.version(),
            });
        }

        if self.bytes.is_empty() {
            return Ok(());
        }

        if !serialization.write_record_data(self.bytes.as_ptr().cast(), header.length()) {
            return Err(SaveError::WriteRecordFailed {
                id: header.id(),
                version: header.version(),
            });
        }

        Ok(())
    }

    #[inline(always)]
    pub fn into_loaded(self) -> LoadedRecord {
        LoadedRecord {
            header: self.header,
            bytes: self.bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedRecord {
    header: RecordHeader,
    bytes: Vec<u8>,
}

impl LoadedRecord {
    #[inline(always)]
    pub fn from_owned(record: OwnedRecord) -> Self {
        record.into_loaded()
    }

    #[inline(always)]
    pub const fn header(&self) -> RecordHeader {
        self.header
    }

    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    #[inline(always)]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    #[inline(always)]
    pub fn reader<'a>(&'a self, context: LoadContext<'a>) -> RecordReader<'a> {
        RecordReader::new(self.bytes.as_slice(), context)
    }

    pub fn decode<T: CosaveDecode>(&self, context: LoadContext<'_>) -> Result<T, LoadError> {
        let mut reader = self.reader(context);
        let value = T::decode(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }
}

pub fn read_next_record(
    serialization: &SerializationInterface,
) -> Result<Option<LoadedRecord>, LoadError> {
    let mut ty = 0_u32;
    let mut version = 0_u32;
    let mut length = 0_u32;
    if !serialization.get_next_record_info(&mut ty, &mut version, &mut length) {
        return Ok(None);
    }

    let header = RecordHeader::new(RecordId::from_raw(ty), version, length);
    let byte_len = usize_from_u32(length)?;
    let mut bytes = vec![0_u8; byte_len];

    if byte_len != 0 {
        let actual = serialization.read_record_data(bytes.as_mut_ptr().cast(), length);
        if actual != length {
            return Err(LoadError::ReadRecordFailed {
                id: header.id(),
                expected: length,
                actual,
            });
        }
    }

    Ok(Some(LoadedRecord { header, bytes }))
}

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

impl CosaveEncode for ResolvedFormId {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_form_id(self.0);
        Ok(())
    }
}

impl CosaveDecode for ResolvedFormId {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_resolved_form_id()
    }
}

impl CosaveEncode for ResolvedVmHandle {
    #[inline(always)]
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        writer.write_vm_handle(self.0);
        Ok(())
    }
}

impl CosaveDecode for ResolvedVmHandle {
    #[inline(always)]
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
        reader.read_resolved_vm_handle()
    }
}

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

impl<T: CosaveEncode, const N: u32> CosaveEncode for BoundedVec<T, N> {
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
        if self.len() > Self::max_len() {
            return Err(SaveError::BoundExceeded {
                len: self.len(),
                max: Self::max_len(),
            });
        }

        writer.write_len(self.len())?;
        for (index, value) in self.inner.iter().enumerate() {
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

        let mut values = Vec::with_capacity(usize_from_u32(length)?);
        for index in 0..usize_from_u32(length)? {
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

#[inline(always)]
const fn usize_from_u32(value: u32) -> Result<usize, LoadError> {
    #[allow(clippy::cast_possible_truncation)]
    let converted = value as usize;
    if converted as u32 != value {
        return Err(LoadError::LengthOverflow(value));
    }
    Ok(converted)
}

fn format_fourcc(f: &mut fmt::Formatter<'_>, raw: u32) -> fmt::Result {
    let bytes = raw.to_le_bytes();
    if bytes
        .iter()
        .copied()
        .all(|byte| byte.is_ascii_graphic() || byte == b' ')
    {
        write!(
            f,
            "'{}{}{}{}' (0x{:08X})",
            bytes[0] as char, bytes[1] as char, bytes[2] as char, bytes[3] as char, raw
        )
    } else {
        write!(f, "0x{:08X}", raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PLUGIN_HANDLE_RAW: u32 = 1;

    unsafe extern "system" fn test_set_unique_id(_plugin: crate::skse::PluginHandle, _uid: u32) {}

    unsafe extern "system" fn test_set_event_callback(
        _plugin: crate::skse::PluginHandle,
        _callback: Option<crate::skse::SerializationEventCallback>,
    ) {
    }

    unsafe extern "system" fn test_set_form_delete_callback(
        _plugin: crate::skse::PluginHandle,
        _callback: Option<crate::skse::FormDeleteCallback>,
    ) {
    }

    unsafe extern "system" fn test_write_record(
        _ty: u32,
        _version: u32,
        _buf: *const core::ffi::c_void,
        _length: u32,
    ) -> bool {
        false
    }

    unsafe extern "system" fn test_open_record(_ty: u32, _version: u32) -> bool {
        false
    }

    unsafe extern "system" fn test_write_record_data(
        _buf: *const core::ffi::c_void,
        _length: u32,
    ) -> bool {
        false
    }

    unsafe extern "system" fn test_get_next_record_info(
        _ty: *mut u32,
        _version: *mut u32,
        _length: *mut u32,
    ) -> bool {
        false
    }

    unsafe extern "system" fn test_read_record_data(
        _buf: *mut core::ffi::c_void,
        _length: u32,
    ) -> u32 {
        0
    }

    unsafe extern "system" fn test_resolve_handle(
        old_handle: VMHandle,
        new_handle: *mut VMHandle,
    ) -> bool {
        if old_handle == 0 {
            return false;
        }

        unsafe {
            *new_handle = old_handle.wrapping_add(TEST_PLUGIN_HANDLE_RAW as u64);
        }
        true
    }

    unsafe extern "system" fn test_resolve_form_id(
        old_form_id: FormID,
        new_form_id: *mut FormID,
    ) -> bool {
        if old_form_id == 0 {
            return false;
        }

        unsafe {
            *new_form_id = old_form_id.wrapping_add(TEST_PLUGIN_HANDLE_RAW);
        }
        true
    }

    fn test_serialization_interface() -> SerializationInterface {
        SerializationInterface {
            interface_version: 4,
            set_unique_id: test_set_unique_id,
            set_revert_callback: test_set_event_callback,
            set_save_callback: test_set_event_callback,
            set_load_callback: test_set_event_callback,
            set_form_delete_callback: test_set_form_delete_callback,
            write_record: test_write_record,
            open_record: test_open_record,
            write_record_data: test_write_record_data,
            get_next_record_info: test_get_next_record_info,
            read_record_data: test_read_record_data,
            resolve_handle: test_resolve_handle,
            resolve_form_id: test_resolve_form_id,
        }
    }

    #[test]
    fn vec_round_trip() {
        let value = vec![1_u32, 2, 3, 5, 8];
        let mut writer = RecordWriter::new();
        value.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = Vec::<u32>::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn bounded_vec_round_trip() {
        let mut value = BoundedVec::<u32, 4>::new();
        value.push(10).unwrap();
        value.push(20).unwrap();

        let mut writer = RecordWriter::new();
        value.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = BoundedVec::<u32, 4>::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded.as_slice(), value.as_slice());
    }

    #[test]
    fn bounded_vec_rejects_oversized_payloads() {
        let mut writer = RecordWriter::new();
        writer.write_u32(3);
        writer.write_u32(1);
        writer.write_u32(2);
        writer.write_u32(3);

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let error = BoundedVec::<u32, 2>::decode(&mut reader).unwrap_err();
        assert_eq!(error, LoadError::VectorTooLong { len: 3, max: 2 });
    }

    #[test]
    fn btree_map_round_trip() {
        let mut value = BTreeMap::new();
        value.insert(String::from("health"), 5_u32);
        value.insert(String::from("stamina"), 7_u32);

        let mut writer = RecordWriter::new();
        value.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = BTreeMap::<String, u32>::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded, value);
    }

    #[test]
    fn fourcc_literals_are_const_checked() {
        const UNIQUE: UniqueId = unique_id!("TFNG");
        const RECORD: RecordId = record_id!("CDAD");

        assert_eq!(UNIQUE.to_bytes(), *b"TFNG");
        assert_eq!(RECORD.to_bytes(), *b"CDAD");
    }

    #[test]
    fn resolved_form_id_decodes_through_context() {
        let interface = test_serialization_interface();
        let context = LoadContext::new(&interface);

        let mut writer = RecordWriter::new();
        writer.write_form_id(0x1000_0042);

        let mut reader = RecordReader::new(writer.bytes(), context);
        let decoded = ResolvedFormId::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded.get(), 0x1000_0043);
    }

    #[test]
    fn resolved_vm_handle_decodes_through_context() {
        let interface = test_serialization_interface();
        let context = LoadContext::new(&interface);

        let mut writer = RecordWriter::new();
        writer.write_vm_handle(0x0000_0000_0000_1000);

        let mut reader = RecordReader::new(writer.bytes(), context);
        let decoded = ResolvedVmHandle::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded.get(), 0x0000_0000_0000_1001);
    }

    #[test]
    fn option_codec_round_trips_some_and_none() {
        let some = Some(String::from("flask"));
        let none: Option<u32> = None;

        let mut writer = RecordWriter::new();
        some.encode(&mut writer).unwrap();
        none.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded_some = Option::<String>::decode(&mut reader).unwrap();
        let decoded_none = Option::<u32>::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded_some, some);
        assert_eq!(decoded_none, none);
    }

    #[test]
    fn fixed_array_codec_round_trips() {
        let values = [3_u32, 5, 8, 13];

        let mut writer = RecordWriter::new();
        values.encode(&mut writer).unwrap();

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let decoded = <[u32; 4]>::decode(&mut reader).unwrap();
        reader.finish().unwrap();

        assert_eq!(decoded, values);
    }

    #[test]
    fn vec_decode_reports_failing_element_index() {
        let mut writer = RecordWriter::new();
        writer.write_len(2).unwrap();
        writer.write_bool(true);
        writer.write_u8(2);

        let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
        let error = Vec::<bool>::decode(&mut reader).unwrap_err();
        assert_eq!(
            error,
            LoadError::SequenceElement {
                index: 1,
                source: Box::new(LoadError::InvalidBool(2)),
            }
        );
    }

    #[test]
    fn btreemap_decode_reports_failing_key_and_value_entries() {
        let mut key_writer = RecordWriter::new();
        key_writer.write_len(1).unwrap();
        key_writer.write_u8(2);
        key_writer.write_u32(10);

        let mut key_reader = RecordReader::new(key_writer.bytes(), LoadContext::empty());
        let key_error = BTreeMap::<bool, u32>::decode(&mut key_reader).unwrap_err();
        assert_eq!(
            key_error,
            LoadError::MapKey {
                entry: 0,
                source: Box::new(LoadError::InvalidBool(2)),
            }
        );

        let mut value_writer = RecordWriter::new();
        value_writer.write_len(1).unwrap();
        value_writer.write_bool(true);
        value_writer.write_u8(2);

        let mut value_reader = RecordReader::new(value_writer.bytes(), LoadContext::empty());
        let value_error = BTreeMap::<bool, bool>::decode(&mut value_reader).unwrap_err();
        assert_eq!(
            value_error,
            LoadError::MapValue {
                entry: 0,
                source: Box::new(LoadError::InvalidBool(2)),
            }
        );
    }
}
