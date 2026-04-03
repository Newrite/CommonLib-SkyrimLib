use alloc::vec;
use alloc::vec::Vec;

use crate::skse::SerializationInterface;

use super::io::{CosaveDecode, CosaveEncode, RecordReader, RecordWriter};
use super::types::{LoadContext, LoadError, RecordHeader, RecordId, SaveError, usize_from_u32};

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
