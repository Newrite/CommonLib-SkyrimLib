use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

use crate::re::bs_core_types::{FormID, VMHandle};
use crate::skse::SerializationInterface;

/// Converts a four-character ASCII literal into a little-endian `u32`.
///
/// This is the building block behind [`record_id!`](crate::record_id) and
/// [`unique_id!`](crate::unique_id).
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

/// Error returned when a [`BoundedVec`] exceeds its configured maximum length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedVecError {
    len: usize,
    max: usize,
}

impl BoundedVecError {
    /// Creates a bounded-vector overflow error.
    #[inline(always)]
    pub const fn new(len: usize, max: usize) -> Self {
        Self { len, max }
    }

    /// Returns the offending length.
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.len
    }

    /// Returns the configured maximum length.
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

/// Length-bounded vector used for schema values with explicit caps.
///
/// This is useful for cosave payloads that want the SDK to enforce a maximum
/// collection length both while constructing values and while decoding them.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::BoundedVec;
///
/// let mut tags = BoundedVec::<u32, 2>::new();
/// tags.push(10)?;
/// tags.push(20)?;
/// assert_eq!(tags.as_slice(), &[10, 20]);
/// # Ok::<(), libskyrim::sdk::persistence::cosave::BoundedVecError>(())
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoundedVec<T, const N: u32> {
    inner: Vec<T>,
}

impl<T, const N: u32> BoundedVec<T, N> {
    /// Creates an empty bounded vector.
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Creates an empty bounded vector with preallocated capacity.
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Result<Self, BoundedVecError> {
        if capacity > Self::max_len() {
            return Err(BoundedVecError::new(capacity, Self::max_len()));
        }

        Ok(Self {
            inner: Vec::with_capacity(capacity),
        })
    }

    /// Returns the configured maximum length as `u32`.
    #[inline(always)]
    pub const fn max_len_u32() -> u32 {
        N
    }

    /// Returns the configured maximum length as `usize`.
    #[inline(always)]
    pub const fn max_len() -> usize {
        N as usize
    }

    /// Returns the current number of stored elements.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` when the collection contains no elements.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes all elements while preserving allocated capacity.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns the stored elements as an immutable slice.
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Returns the stored elements as a mutable slice.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    /// Appends one element if doing so stays within the configured bound.
    #[inline(always)]
    pub fn push(&mut self, value: T) -> Result<(), BoundedVecError> {
        if self.inner.len() >= Self::max_len() {
            return Err(BoundedVecError::new(self.inner.len() + 1, Self::max_len()));
        }

        self.inner.push(value);
        Ok(())
    }

    /// Extends the collection while preserving the configured upper bound.
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

    /// Consumes the bounded collection and returns the underlying `Vec`.
    #[inline(always)]
    pub fn into_vec(self) -> Vec<T> {
        self.inner
    }

    /// Converts a plain vector into a bounded one if it fits.
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

/// Strongly typed four-character cosave record identifier.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::RecordId;
///
/// let id = RecordId::from_bytes(*b"CNT1");
/// assert_eq!(id.to_bytes(), *b"CNT1");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RecordId(u32);

impl RecordId {
    /// Creates a record ID from a raw little-endian `u32`.
    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a record ID from ASCII bytes.
    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Returns the raw little-endian numeric representation.
    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the four ASCII bytes in little-endian order.
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

/// Strongly typed plugin unique ID used with SKSE serialization registration.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::UniqueId;
///
/// let uid = UniqueId::from_bytes(*b"EXMP");
/// assert_eq!(uid.to_bytes(), *b"EXMP");
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UniqueId(u32);

impl UniqueId {
    /// Creates a unique ID from a raw little-endian `u32`.
    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Creates a unique ID from ASCII bytes.
    #[inline(always)]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(bytes))
    }

    /// Returns the raw little-endian numeric representation.
    #[inline(always)]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the four ASCII bytes in little-endian order.
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

/// Header metadata for one cosave record.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{RecordHeader, RecordId};
///
/// let header = RecordHeader::new(RecordId::from_bytes(*b"CNT1"), 2, 16);
/// assert_eq!(header.version(), 2);
/// assert_eq!(header.length(), 16);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader {
    id: RecordId,
    version: u32,
    length: u32,
}

impl RecordHeader {
    /// Creates one typed record header.
    #[inline(always)]
    pub const fn new(id: RecordId, version: u32, length: u32) -> Self {
        Self {
            id,
            version,
            length,
        }
    }

    /// Returns the typed record identifier.
    #[inline(always)]
    pub const fn id(self) -> RecordId {
        self.id
    }

    /// Returns the schema/version number stored in the header.
    #[inline(always)]
    pub const fn version(self) -> u32 {
        self.version
    }

    /// Returns the payload byte length stored in the header.
    #[inline(always)]
    pub const fn length(self) -> u32 {
        self.length
    }
}

/// `FormID` that has already been resolved for the current load order.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::ResolvedFormId;
///
/// let form_id = ResolvedFormId::new(0x1234_5678);
/// assert_eq!(form_id.get(), 0x1234_5678);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ResolvedFormId(FormID);

impl ResolvedFormId {
    /// Wraps one already-resolved runtime `FormID`.
    #[inline(always)]
    pub const fn new(form_id: FormID) -> Self {
        Self(form_id)
    }

    /// Returns the wrapped resolved `FormID`.
    #[inline(always)]
    pub const fn get(self) -> FormID {
        self.0
    }

    /// Consumes the wrapper and returns the resolved `FormID`.
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

/// `VMHandle` that has already been resolved for the current runtime session.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::ResolvedVmHandle;
///
/// let handle = ResolvedVmHandle::new(0x1234);
/// assert_eq!(handle.get(), 0x1234);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ResolvedVmHandle(VMHandle);

impl ResolvedVmHandle {
    /// Wraps one already-resolved runtime `VMHandle`.
    #[inline(always)]
    pub const fn new(handle: VMHandle) -> Self {
        Self(handle)
    }

    /// Returns the wrapped resolved `VMHandle`.
    #[inline(always)]
    pub const fn get(self) -> VMHandle {
        self.0
    }

    /// Consumes the wrapper and returns the resolved `VMHandle`.
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

/// Save-side error for typed cosave encoding and record emission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveError {
    /// One encoded payload or length prefix exceeded `u32::MAX`.
    LengthOverflow(usize),
    /// One bounded payload shape exceeded its declared maximum length.
    BoundExceeded { len: usize, max: usize },
    /// SKSE failed to open the requested record header for writing.
    OpenRecordFailed { id: RecordId, version: u32 },
    /// SKSE failed while writing payload bytes for one opened record.
    WriteRecordFailed { id: RecordId, version: u32 },
    /// Nested field encode failure enriched with the field name.
    Field {
        field: &'static str,
        source: Box<SaveError>,
    },
    /// Nested sequence-element encode failure enriched with the element index.
    SequenceElement {
        index: usize,
        source: Box<SaveError>,
    },
    /// Nested map-key encode failure enriched with the entry index.
    MapKey {
        entry: usize,
        source: Box<SaveError>,
    },
    /// Nested map-value encode failure enriched with the entry index.
    MapValue {
        entry: usize,
        source: Box<SaveError>,
    },
}

impl SaveError {
    /// Wraps a nested field-encoding failure with field context.
    #[inline(always)]
    pub fn field(field: &'static str, source: SaveError) -> Self {
        Self::Field {
            field,
            source: Box::new(source),
        }
    }

    /// Wraps a nested sequence-element encoding failure with element context.
    #[inline(always)]
    pub fn sequence_element(index: usize, source: SaveError) -> Self {
        Self::SequenceElement {
            index,
            source: Box::new(source),
        }
    }

    /// Wraps a nested map-key encoding failure with entry context.
    #[inline(always)]
    pub fn map_key(entry: usize, source: SaveError) -> Self {
        Self::MapKey {
            entry,
            source: Box::new(source),
        }
    }

    /// Wraps a nested map-value encoding failure with entry context.
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

/// Load-side error for typed cosave decoding and identifier resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// One serialized length did not fit the current platform `usize`.
    LengthOverflow(u32),
    /// SKSE reported fewer bytes than promised for the current record.
    ReadRecordFailed {
        id: RecordId,
        expected: u32,
        actual: u32,
    },
    /// The payload ended before the requested number of bytes could be read.
    UnexpectedEof { requested: usize, remaining: usize },
    /// Typed decode completed before consuming the full payload.
    TrailingBytes { remaining: usize },
    /// A serialized string was not valid UTF-8.
    InvalidUtf8(core::str::Utf8Error),
    /// A serialized bool was not encoded as `0` or `1`.
    InvalidBool(u8),
    /// A serialized collection length exceeded its configured maximum.
    VectorTooLong { len: u32, max: u32 },
    /// A map decode path encountered the same key more than once.
    DuplicateMapKey,
    /// The payload requested identifier resolution without a backing load context.
    MissingSerializationContext(&'static str),
    /// A saved `FormID` could not be resolved for the current load order.
    ResolveFormIdFailed(FormID),
    /// A saved `VMHandle` could not be resolved for the current runtime session.
    ResolveHandleFailed(VMHandle),
    /// Nested field decode failure enriched with the field name.
    Field {
        field: &'static str,
        source: Box<LoadError>,
    },
    /// Nested sequence-element decode failure enriched with the element index.
    SequenceElement {
        index: usize,
        source: Box<LoadError>,
    },
    /// Nested map-key decode failure enriched with the entry index.
    MapKey {
        entry: usize,
        source: Box<LoadError>,
    },
    /// Nested map-value decode failure enriched with the entry index.
    MapValue {
        entry: usize,
        source: Box<LoadError>,
    },
}

impl LoadError {
    /// Wraps a nested field-decoding failure with field context.
    #[inline(always)]
    pub fn field(field: &'static str, source: LoadError) -> Self {
        Self::Field {
            field,
            source: Box::new(source),
        }
    }

    /// Wraps a nested sequence-element decoding failure with element context.
    #[inline(always)]
    pub fn sequence_element(index: usize, source: LoadError) -> Self {
        Self::SequenceElement {
            index,
            source: Box::new(source),
        }
    }

    /// Wraps a nested map-key decoding failure with entry context.
    #[inline(always)]
    pub fn map_key(entry: usize, source: LoadError) -> Self {
        Self::MapKey {
            entry,
            source: Box::new(source),
        }
    }

    /// Wraps a nested map-value decoding failure with entry context.
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

/// Load-time helper for `FormID` and `VMHandle` resolution.
///
/// Pass [`LoadContext::new`] when decoding payloads loaded through
/// `SKSE::SerializationInterface`; use [`LoadContext::empty`] when a record does
/// not contain any resolvable identifiers.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::LoadContext;
///
/// fn decode_plain_payload(bytes: &[u8]) -> u32 {
///     let mut reader = libskyrim::sdk::persistence::cosave::RecordReader::new(
///         bytes,
///         LoadContext::empty(),
///     );
///     let value = reader.read_u32().unwrap();
///     reader.finish().unwrap();
///     value
/// }
/// ```
#[derive(Clone, Copy, Default)]
pub struct LoadContext<'a> {
    serialization: Option<&'a SerializationInterface>,
}

impl<'a> LoadContext<'a> {
    /// Creates an empty context with no resolution support.
    ///
    /// Use this for payloads that do not contain saved `FormID`s or
    /// `VMHandle`s requiring runtime resolution.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            serialization: None,
        }
    }

    /// Creates a context backed by `SKSE::SerializationInterface`.
    ///
    /// Use this for load flows that decode [`ResolvedFormId`] or
    /// [`ResolvedVmHandle`].
    #[inline(always)]
    pub const fn new(serialization: &'a SerializationInterface) -> Self {
        Self {
            serialization: Some(serialization),
        }
    }

    /// Returns the backing SKSE serialization interface, if any.
    #[inline(always)]
    pub const fn serialization(self) -> Option<&'a SerializationInterface> {
        self.serialization
    }

    /// Resolves a saved `FormID` for the current runtime/load order.
    ///
    /// Most typed decode code does not call this directly; instead it decodes
    /// [`ResolvedFormId`], which routes through the same resolution path.
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

    /// Resolves a saved `VMHandle` for the current runtime session.
    ///
    /// Most typed decode code does not call this directly; instead it decodes
    /// [`ResolvedVmHandle`], which routes through the same resolution path.
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

#[inline(always)]
pub(super) const fn usize_from_u32(value: u32) -> Result<usize, LoadError> {
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
