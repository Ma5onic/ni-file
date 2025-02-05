use crate::{read_bytes::ReadBytesExt, Error, NIFileError};

/// The header chunk of an [`Item`](crate::nisound::Item).
/// 40 bytes
///
/// | Offset | Length | Type      | Meaning                     | Default    | Other                                    |
/// |--------|--------|-----------|-----------------------------|------------|------------------------------------------|
/// | 0      | 8      | uint64_t  | size                        |  |                                          |
/// | 8      | 4      | uint32_t  | version                     | 1 |                                          |
/// | 12     | 4      | uint32_t  | domainID                     |  |                                          |
/// | 16     | 4      | uint32_t  | headerFlags                     |  |                                          |
/// | 20     | 4      | ItemUuid* | itemUuid                     |  |                                          |
///
#[derive(Debug, Clone)]
pub struct ItemHeader {
    /// Size in bytes of the entire [`Item`](super::Item).
    pub length: u64,
    /// Integer that resolves to a [`DomainID`](super::DomainID).
    pub magic: Vec<u8>, // (+0xC, uint, 'hsin')
    pub header_flags: u32, // (0x10, uint)
    /// # Boost-compatible GUID
    ///
    /// https://www.boost.org/doc/libs/1\_43\_0/libs/uuid/index.html
    pub uuid: Vec<u8>, // (0x14, 16 bytes, randomly generated)
}

impl ItemHeader {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let length = reader.read_u64_le()?;
        let version = reader.read_u32_le()?;
        let magic = reader.read_bytes(4)?;
        let header_flags = reader.read_u32_le()?;
        let _unknown = reader.read_u32_le()?;
        let uuid = reader.read_bytes(16)?;

        if magic != b"hsin" {
            let magic = crate::utils::format_hex(&magic);
            return Err(NIFileError::Generic(format!(
                "Error reading ItemHeader magic: expected hsin 0x6873696E, got 0x{magic}"
            )));
        };

        if version != 1 {
            return Err(NIFileError::Generic("version must be 1".into()));
        };

        Ok(Self {
            length,
            magic,
            header_flags,
            uuid,
        })
    }

    /// Returns the only known header flag.
    pub fn deferred_flag(&self) -> u32 {
        self.header_flags & 0x00000001
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_item_frame_read() -> Result<(), Error> {
        let file = File::open("tests/patchdata/NISD/ItemHeader/ItemHeader-RepositoryRoot-000")?;
        let item = ItemHeader::read(file)?;
        assert_eq!(item.magic, b"hsin");

        Ok(())
    }
}
