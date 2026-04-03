use alloc::string::String;
use alloc::vec::Vec;

use crate::re::bs_core_types::{FormID, VMHandle};

use super::records::OwnedRecord;
use super::types::{
    LoadContext, LoadError, RecordId, ResolvedFormId, ResolvedVmHandle, SaveError, usize_from_u32,
};

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
