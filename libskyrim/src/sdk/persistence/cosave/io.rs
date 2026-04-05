use alloc::string::String;
use alloc::vec::Vec;

use crate::re::bs_core_types::{FormID, VMHandle};

use super::records::OwnedRecord;
use super::types::{
    LoadContext, LoadError, RecordId, ResolvedFormId, ResolvedVmHandle, SaveError, usize_from_u32,
};

/// Trait for values that can encode themselves into a cosave record payload.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{CosaveEncode, RecordWriter, SaveError};
///
/// struct Pair {
///     first: u32,
///     second: u32,
/// }
///
/// impl CosaveEncode for Pair {
///     fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError> {
///         writer.write_u32(self.first);
///         writer.write_u32(self.second);
///         Ok(())
///     }
/// }
/// ```
pub trait CosaveEncode {
    /// Appends this value to the given record writer.
    fn encode(&self, writer: &mut RecordWriter) -> Result<(), SaveError>;
}

/// Trait for values that can decode themselves from a cosave record payload.
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{CosaveDecode, LoadError, RecordReader};
///
/// struct Pair {
///     first: u32,
///     second: u32,
/// }
///
/// impl CosaveDecode for Pair {
///     fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError> {
///         Ok(Self {
///             first: reader.read_u32()?,
///             second: reader.read_u32()?,
///         })
///     }
/// }
/// ```
pub trait CosaveDecode: Sized {
    /// Decodes a value from the current reader position.
    fn decode(reader: &mut RecordReader<'_>) -> Result<Self, LoadError>;
}

/// Byte-oriented cosave payload writer.
///
/// Use this directly when building a record incrementally, or rely on
/// [`OwnedRecord::from_value`](super::records::OwnedRecord::from_value) when a
/// single typed value already implements [`CosaveEncode`].
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{RecordId, RecordWriter};
///
/// let mut writer = RecordWriter::new();
/// writer.write_u32(42);
/// writer.write_string("hello")?;
///
/// let record = writer.into_record(RecordId::from_bytes(*b"DATA"), 1)?;
/// # Ok::<(), libskyrim::sdk::persistence::cosave::SaveError>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct RecordWriter {
    bytes: Vec<u8>,
}

impl RecordWriter {
    /// Creates an empty record writer.
    #[inline(always)]
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Creates an empty writer with preallocated capacity.
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of encoded bytes currently buffered.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns `true` when no bytes have been written yet.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the buffered payload bytes.
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Consumes the writer and returns the encoded payload bytes.
    #[inline(always)]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Converts the buffered payload into an [`OwnedRecord`].
    #[inline(always)]
    pub fn into_record(self, id: RecordId, version: u32) -> Result<OwnedRecord, SaveError> {
        OwnedRecord::new(id, version, self.into_bytes())
    }

    /// Encodes a typed value into this writer.
    #[inline(always)]
    pub fn write_value<T: CosaveEncode + ?Sized>(&mut self, value: &T) -> Result<(), SaveError> {
        value.encode(self)
    }

    /// Appends raw bytes without additional length metadata.
    #[inline(always)]
    pub fn write_raw_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    #[inline(always)]
    pub fn write_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    /// Appends one signed byte in little-endian/raw-byte form.
    #[inline(always)]
    pub fn write_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    /// Appends one `u16` in little-endian form.
    #[inline(always)]
    pub fn write_u16(&mut self, value: u16) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one `i16` in little-endian form.
    #[inline(always)]
    pub fn write_i16(&mut self, value: i16) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one `u32` in little-endian form.
    #[inline(always)]
    pub fn write_u32(&mut self, value: u32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one raw saved `FormID`.
    ///
    /// This does not perform any load-order resolution. Pair it with
    /// [`RecordReader::read_form_id`] for raw IDs or
    /// [`RecordReader::read_resolved_form_id`] when load-time resolution is
    /// required.
    #[inline(always)]
    pub fn write_form_id(&mut self, value: FormID) {
        self.write_u32(value);
    }

    /// Appends one `i32` in little-endian form.
    #[inline(always)]
    pub fn write_i32(&mut self, value: i32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one `u64` in little-endian form.
    #[inline(always)]
    pub fn write_u64(&mut self, value: u64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one raw saved `VMHandle`.
    ///
    /// This does not perform any session resolution. Pair it with
    /// [`RecordReader::read_vm_handle`] for raw handles or
    /// [`RecordReader::read_resolved_vm_handle`] when load-time resolution is
    /// required.
    #[inline(always)]
    pub fn write_vm_handle(&mut self, value: VMHandle) {
        self.write_u64(value);
    }

    /// Appends one `i64` in little-endian form.
    #[inline(always)]
    pub fn write_i64(&mut self, value: i64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one `f32` in IEEE little-endian form.
    #[inline(always)]
    pub fn write_f32(&mut self, value: f32) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one `f64` in IEEE little-endian form.
    #[inline(always)]
    pub fn write_f64(&mut self, value: f64) {
        self.write_raw_bytes(&value.to_le_bytes());
    }

    /// Appends one bool encoded as `0` or `1`.
    #[inline(always)]
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(u8::from(value));
    }

    /// Writes a `u32`-encoded length prefix.
    pub fn write_len(&mut self, len: usize) -> Result<(), SaveError> {
        let len = u32::try_from(len).map_err(|_| SaveError::LengthOverflow(len))?;
        self.write_u32(len);
        Ok(())
    }

    /// Writes a UTF-8 string using a `u32` length prefix.
    pub fn write_string(&mut self, value: &str) -> Result<(), SaveError> {
        self.write_len(value.len())?;
        self.write_raw_bytes(value.as_bytes());
        Ok(())
    }
}

/// Bounded view over one decoded cosave record payload.
///
/// The reader owns no storage; it simply walks a borrowed byte slice together
/// with an optional [`LoadContext`] used for `FormID` and `VMHandle`
/// resolution.
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::persistence::cosave::{LoadContext, RecordReader, RecordWriter};
///
/// let mut writer = RecordWriter::new();
/// writer.write_u32(42);
/// writer.write_string("hello")?;
///
/// let mut reader = RecordReader::new(writer.bytes(), LoadContext::empty());
/// let counter = reader.read_u32()?;
/// let text = reader.read_string()?;
/// reader.finish()?;
///
/// assert_eq!(counter, 42);
/// assert_eq!(text, "hello");
/// # Ok::<(), libskyrim::sdk::persistence::cosave::LoadError>(())
/// ```
#[derive(Clone, Copy)]
pub struct RecordReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
    context: LoadContext<'a>,
}

impl<'a> RecordReader<'a> {
    /// Creates a reader over the given payload bytes.
    #[inline(always)]
    pub fn new(bytes: &'a [u8], context: LoadContext<'a>) -> Self {
        Self {
            bytes,
            cursor: 0,
            context,
        }
    }

    /// Returns the load context associated with this reader.
    #[inline(always)]
    pub fn context(&self) -> LoadContext<'a> {
        self.context
    }

    /// Returns the total payload length in bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns the current cursor position.
    #[inline(always)]
    pub fn position(&self) -> usize {
        self.cursor
    }

    /// Returns the number of unread bytes remaining.
    #[inline(always)]
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.cursor)
    }

    /// Returns `true` when the reader has consumed the full payload.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }

    /// Ensures that the full record payload has been consumed.
    ///
    /// This is useful at the end of typed decode flows to reject stale schema
    /// mismatches that would otherwise leave unread trailing bytes.
    pub fn finish(&self) -> Result<(), LoadError> {
        let remaining = self.remaining();
        if remaining == 0 {
            return Ok(());
        }

        Err(LoadError::TrailingBytes { remaining })
    }

    /// Reads exactly `length` raw bytes from the payload.
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

    /// Decodes a typed value from the current cursor position.
    #[inline(always)]
    pub fn read_value<T: CosaveDecode>(&mut self) -> Result<T, LoadError> {
        T::decode(self)
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> Result<u8, LoadError> {
        Ok(self.read_exact(1)?[0])
    }

    /// Reads one signed byte from the payload.
    #[inline(always)]
    pub fn read_i8(&mut self) -> Result<i8, LoadError> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads one little-endian `u16`.
    pub fn read_u16(&mut self) -> Result<u16, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u16>())?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Reads one little-endian `i16`.
    pub fn read_i16(&mut self) -> Result<i16, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i16>())?;
        Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
    }

    /// Reads one little-endian `u32`.
    pub fn read_u32(&mut self) -> Result<u32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u32>())?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Reads one raw saved `FormID` without load-order resolution.
    #[inline(always)]
    pub fn read_form_id(&mut self) -> Result<FormID, LoadError> {
        self.read_u32()
    }

    /// Reads one little-endian `i32`.
    pub fn read_i32(&mut self) -> Result<i32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i32>())?;
        Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Reads one little-endian `u64`.
    pub fn read_u64(&mut self) -> Result<u64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<u64>())?;
        Ok(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Reads one raw saved `VMHandle` without session resolution.
    #[inline(always)]
    pub fn read_vm_handle(&mut self) -> Result<VMHandle, LoadError> {
        self.read_u64()
    }

    /// Reads one little-endian `i64`.
    pub fn read_i64(&mut self) -> Result<i64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<i64>())?;
        Ok(i64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Reads one IEEE little-endian `f32`.
    pub fn read_f32(&mut self) -> Result<f32, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<f32>())?;
        Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Reads one IEEE little-endian `f64`.
    pub fn read_f64(&mut self) -> Result<f64, LoadError> {
        let bytes = self.read_exact(core::mem::size_of::<f64>())?;
        Ok(f64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    /// Reads one serialized bool encoded as `0` or `1`.
    pub fn read_bool(&mut self) -> Result<bool, LoadError> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(LoadError::InvalidBool(value)),
        }
    }

    /// Reads a `u32`-encoded collection/string length prefix.
    #[inline(always)]
    pub fn read_len(&mut self) -> Result<u32, LoadError> {
        self.read_u32()
    }

    /// Reads a UTF-8 string encoded by [`RecordWriter::write_string`].
    pub fn read_string(&mut self) -> Result<String, LoadError> {
        let length = usize_from_u32(self.read_len()?)?;
        let bytes = self.read_exact(length)?;
        let text = core::str::from_utf8(bytes).map_err(LoadError::InvalidUtf8)?;
        Ok(String::from(text))
    }

    /// Reads and resolves a saved `FormID` through the current [`LoadContext`].
    #[inline(always)]
    pub fn read_resolved_form_id(&mut self) -> Result<ResolvedFormId, LoadError> {
        Ok(ResolvedFormId::new(
            self.context().resolve_form_id(self.read_form_id()?)?,
        ))
    }

    /// Reads and resolves a saved `VMHandle` through the current [`LoadContext`].
    #[inline(always)]
    pub fn read_resolved_vm_handle(&mut self) -> Result<ResolvedVmHandle, LoadError> {
        Ok(ResolvedVmHandle::new(
            self.context().resolve_handle(self.read_vm_handle()?)?,
        ))
    }
}
