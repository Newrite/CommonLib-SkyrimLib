use alloc::vec;
use alloc::vec::Vec;

use crate::skse::SerializationInterface;

use super::io::{CosaveDecode, CosaveEncode, RecordReader, RecordWriter};
use super::types::{LoadContext, LoadError, RecordHeader, RecordId, SaveError, usize_from_u32};

/// Owned record ready to be written through `SKSE::SerializationInterface`.
///
/// This is the save-side wrapper: it stores a typed [`RecordHeader`] together
/// with already encoded bytes and can write itself out through [`Self::write_to`].
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{OwnedRecord, RecordId};
///
/// let record = OwnedRecord::from_value(RecordId::from_bytes(*b"CNT1"), 1, &42_u32)?;
/// assert_eq!(record.header().id(), RecordId::from_bytes(*b"CNT1"));
/// # Ok::<(), libskyrim::sdk::persistence::cosave::SaveError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRecord {
    header: RecordHeader,
    bytes: Vec<u8>,
}

impl OwnedRecord {
    /// Creates a save-side record from already encoded payload bytes.
    ///
    /// Use this when payload encoding happened elsewhere and only the final
    /// typed header plus owned bytes still need to be wrapped before emission.
    pub fn new(id: RecordId, version: u32, bytes: Vec<u8>) -> Result<Self, SaveError> {
        let length =
            u32::try_from(bytes.len()).map_err(|_| SaveError::LengthOverflow(bytes.len()))?;
        Ok(Self {
            header: RecordHeader::new(id, version, length),
            bytes,
        })
    }

    /// Encodes a typed value into a new owned record.
    ///
    /// This is the common convenience entrypoint for save code that already has
    /// a `CosaveEncode` implementation for its payload type.
    pub fn from_value<T: CosaveEncode>(
        id: RecordId,
        version: u32,
        value: &T,
    ) -> Result<Self, SaveError> {
        let mut writer = RecordWriter::new();
        value.encode(&mut writer)?;
        writer.into_record(id, version)
    }

    /// Returns the typed cosave header for this record.
    ///
    /// The header includes the record ID, version, and payload length that will
    /// be written to SKSE.
    #[inline(always)]
    pub const fn header(&self) -> RecordHeader {
        self.header
    }

    /// Returns the encoded payload bytes exactly as they will be written.
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Writes this record to the active SKSE cosave stream.
    ///
    /// This opens the record through `SKSE::SerializationInterface`, then emits
    /// the buffered payload bytes. Empty payloads still produce a header-only
    /// record.
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

    /// Converts this save-side record into a load-side wrapper.
    ///
    /// This is mostly useful in tests, migration helpers, or tooling code that
    /// wants to reuse one encoded record on both the save and load sides.
    #[inline(always)]
    pub fn into_loaded(self) -> LoadedRecord {
        LoadedRecord {
            header: self.header,
            bytes: self.bytes,
        }
    }
}

/// Loaded record returned from `SKSE::SerializationInterface`.
///
/// This is the load-side wrapper: it keeps the record header and raw payload
/// bytes together and offers typed decode helpers.
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{LoadContext, LoadedRecord, OwnedRecord, RecordId};
///
/// let loaded = LoadedRecord::from_owned(
///     OwnedRecord::from_value(RecordId::from_bytes(*b"CNT1"), 1, &42_u32)?
/// );
///
/// let counter = loaded.decode::<u32>(LoadContext::empty())?;
/// assert_eq!(counter, 42);
/// # Ok::<(), libskyrim::sdk::persistence::cosave::LoadError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedRecord {
    header: RecordHeader,
    bytes: Vec<u8>,
}

impl LoadedRecord {
    /// Converts an [`OwnedRecord`] into its load-side representation.
    #[inline(always)]
    pub fn from_owned(record: OwnedRecord) -> Self {
        record.into_loaded()
    }

    /// Returns the typed cosave header for this record.
    ///
    /// This is the same header SKSE reported while iterating the current cosave
    /// stream.
    #[inline(always)]
    pub const fn header(&self) -> RecordHeader {
        self.header
    }

    /// Returns the raw record payload bytes.
    ///
    /// Prefer [`decode`](Self::decode) or [`reader`](Self::reader) when the
    /// payload should be interpreted as typed cosave data.
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Consumes the record and returns the raw payload bytes.
    #[inline(always)]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Creates a [`RecordReader`] over this record payload.
    ///
    /// Use this when the caller wants incremental/manual decode instead of one
    /// whole-value [`decode`](Self::decode) pass.
    #[inline(always)]
    pub fn reader<'a>(&'a self, context: LoadContext<'a>) -> RecordReader<'a> {
        RecordReader::new(self.bytes.as_slice(), context)
    }

    /// Decodes the record payload as a typed value.
    ///
    /// This finishes the reader after decoding and therefore rejects trailing
    /// unread bytes by default.
    pub fn decode<T: CosaveDecode>(&self, context: LoadContext<'_>) -> Result<T, LoadError> {
        let mut reader = self.reader(context);
        let value = T::decode(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }
}

/// Reads the next raw cosave record from `SKSE::SerializationInterface`.
///
/// Returns `Ok(None)` when no more records are available from the current
/// serialization stream.
///
/// This is the raw record-pull primitive used by higher-level schema/model
/// layers. It only reads the header plus payload bytes; typed payload decode
/// happens later through [`LoadedRecord::decode`] or [`LoadedRecord::reader`].
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
