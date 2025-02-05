///
/// magic = 0xa4d6e55a || 0xab85ef01 || 0xb36ee55e || 0x10874353 ||  0x74b5a69b || 0x7fa89012
///
/// headerVersion
///     < 256           36 bytes
///     > 256 && < 271  170 bytes
///     > 271           222 bytes
///
use std::io::Cursor;

use time::OffsetDateTime;

use crate::{nks::error::NKSError, read_bytes::ReadBytesExt};

#[derive(Debug, PartialEq)]
pub enum BPatchHeader {
    BPatchHeaderV1(BPatchHeaderV1),
    BPatchHeaderV2(BPatchHeaderV2),
    BPatchHeaderV42(BPatchHeaderV42),
}

impl BPatchHeader {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let header_version = reader.read_u16_le()?;
        Ok(match header_version {
            0..=255 => Self::BPatchHeaderV1(BPatchHeaderV1::read_le(&mut reader)?),
            256..=271 => Self::BPatchHeaderV2(BPatchHeaderV2::read_le(&mut reader)?),
            _ => Self::BPatchHeaderV42(BPatchHeaderV42::read_le(&mut reader)?),
        })
    }
}

/// The header of a Kontakt42 NKS File.
#[derive(Debug, PartialEq, Clone)]
pub struct BPatchHeaderV42 {
    pub patch_type: PatchType,
    /// Patch version (often higher than the Kontakt version that created it)
    pub patch_version: NKIAppVersion,
    pub app_signature: String,
    pub created_at: time::Date,
    /// Only used in V2
    pub u_a: u32,
    pub number_of_zones: u16,
    pub number_of_groups: u16,
    pub number_of_instruments: u16,
    /// Total length of all PCM sample data in bytes (not including RIFF header data)
    /// Should be: numBytesSamplesTotal
    pub pcm_data_len: u32,
    pub is_monolith: bool,
    pub min_supported_version: NKIAppVersion,
    pub u_c: u32,
    pub cat_icon_idx: u32,
    pub instrument_author: String,
    pub instrument_cat1: u8,
    pub instrument_cat2: u8,
    pub instrument_cat3: u8,
    pub instrument_url: String,
    pub u_b: u32,
    /// Unknown bit flags. Known values: 0, 32, 36, 37, 44
    pub flags: u32,
    /// MD5 checksum of the decompressed chunk data
    pub md5_checksum: Vec<u8>,
    /// The final part (patch level) of the authoring app version number.
    /// For example, for Kontakt 5.0.2.5641, the svn revision is 5641.
    pub svn_revision: u32,
    /// CRC32 checksum of the compressed binary data
    pub crc32_fast: [u8; 4],
    /// Length in bytes of the decompressed inner preset chunk (unused in NISound documents)
    pub decompressed_length: u32,
}

/// The header of a Kontakt2 NKS File.
#[derive(Debug, PartialEq)]
pub struct BPatchHeaderV2 {
    pub patch_type: PatchType,
    /// Patch version (often higher than the Kontakt version that created it)
    pub patch_version: NKIAppVersion,
    pub app_signature: String,
    pub created_at: time::Date,
    /// Only used in V2. Seems to be related to filesize.
    pub u_a: u32,
    pub number_of_zones: u16,
    pub number_of_groups: u16,
    pub number_of_instruments: u16,
    /// Total length of all PCM sample data in bytes (not including RIFF header data)
    /// Should be: numBytesSamplesTotal
    pub pcm_data_len: u32,
    pub is_monolith: bool,
    pub min_supported_version: NKIAppVersion,
    pub u_c: u32,
    pub cat_icon_idx: u32,
    pub instrument_author: String,
    pub instrument_url: String,
    pub instrument_cat1: u8,
    pub instrument_cat2: u8,
    pub instrument_cat3: u8,
    pub svn_revision: u32,
    pub patch_level: u32,
    pub u_b: u32,
    pub unknown_offset: u32,
}

/// The header of a Kontakt1 NKS File.
#[derive(Debug, PartialEq)]
pub struct BPatchHeaderV1 {
    pub u_version: u16,
    pub u_a: u32,
    pub u_b: u32,
    pub u_c: u32,
    pub u_d: u32,
    pub created_at: time::Date,
    pub samples_size: u32,
}

impl BPatchHeaderV1 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let u_version = reader.read_u16_le()?; // version? usually 2
        let u_a = reader.read_u32_le()?; // ?
        let u_b = reader.read_u32_le()?; // ?
        let u_c = reader.read_u32_le()?; // ?

        let timestamp = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = timestamp.date();

        let samples_size = reader.read_u32_le()?; // total size of all samples

        let u_d = reader.read_u32_le()?; // mostly 0, found 1

        Ok(Self {
            u_version,
            u_a,
            u_b,
            u_c,
            u_d,
            created_at,
            samples_size,
        })
    }
}

impl BPatchHeaderV2 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let data = reader.read_bytes(160)?; // 170 - 10
        let mut reader = Cursor::new(data);

        let header_magic = reader.read_u32_le()?;
        assert_eq!(header_magic, u32::swap_bytes(0x722A013E));

        let patch_type: PatchType = reader.read_u16_le()?.into();
        let patch_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let mut app_signature = reader.read_bytes(4)?;
        app_signature.reverse();
        let app_signature: String = app_signature.into_iter().map(|c| c as char).collect();

        let datetime = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = datetime.date();

        let u_a = reader.read_u32_le()?;

        let number_of_zones = reader.read_u16_le()?;
        let number_of_groups = reader.read_u16_le()?;
        let number_of_instruments = reader.read_u16_le()?;

        let pcm_data_len = reader.read_u32_le()?;
        let is_monolith = reader.read_u32_le()? == 1;

        let min_supported_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let u_c = reader.read_u32_le()?;

        let cat_icon_idx = reader.read_u32_le()?;

        let instrument_author = String::from_utf8(reader.read_bytes(8)?)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.utf8_error()))?
            .trim_matches(char::from(0))
            .to_string();

        let instrument_cat1 = reader.read_u8()?;
        let instrument_cat2 = reader.read_u8()?;
        let instrument_cat3 = reader.read_u8()?;

        let instrument_url = String::from_utf8(reader.read_bytes(85)?)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.utf8_error()))?
            .trim_matches(char::from(0))
            .to_string();

        let u_b = reader.read_u32_le()?;
        let patch_level = reader.read_u32_le()?;
        let svn_revision = reader.read_u32_le()?;
        // let unknown_offset = reader.read_u32_le()?;
        let unknown_offset = 0;

        Ok(Self {
            patch_type,
            patch_version,
            app_signature,
            created_at,
            u_a,
            u_c,
            pcm_data_len,
            number_of_zones,
            number_of_groups,
            number_of_instruments,
            is_monolith,
            min_supported_version,
            cat_icon_idx,
            instrument_author,
            instrument_url,
            instrument_cat1,
            instrument_cat2,
            instrument_cat3,
            svn_revision,
            u_b,
            patch_level,
            unknown_offset,
        })
    }
}

impl BPatchHeaderV42 {
    pub fn read_le<R: ReadBytesExt>(mut reader: R) -> Result<Self, NKSError> {
        let data = reader.read_bytes(212)?; // 222 - 10
        let mut reader = Cursor::new(data);

        let magic: u32 = reader.read_le()?;
        assert_eq!(
            magic, 0xEA37631A,
            "Invalid BPatchHeaderV42 magic number: expected 0x1a6337ea got 0x{magic:x}"
        );

        let patch_type: PatchType = reader.read_u16_le()?.into();
        let patch_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let mut app_signature = reader.read_bytes(4)?;
        app_signature.reverse();
        let app_signature: String = app_signature.into_iter().map(|c| c as char).collect();

        let datetime = OffsetDateTime::from_unix_timestamp(reader.read_u32_le()? as i64).unwrap();
        let created_at: time::Date = datetime.date();

        let u_a = reader.read_u32_le()?;
        assert_eq!(u_a, 0, "u_a should be 0");

        let number_of_zones = reader.read_u16_le()?;
        let number_of_groups = reader.read_u16_le()?;
        let number_of_instruments = reader.read_u16_le()?;

        let pcm_data_len = reader.read_u32_le()?;
        let is_monolith = reader.read_u32_le()? == 1;

        let min_supported_version = NKIAppVersion {
            minor_3: reader.read_u8()?,
            minor_2: reader.read_u8()?,
            minor_1: reader.read_u8()?,
            major: reader.read_u8()?,
        };

        let u_c = reader.read_u32_le()?;

        let cat_icon_idx = reader.read_u32_le()?;

        let instrument_author = String::from_utf8(reader.read_bytes(8)?)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.utf8_error()))?
            .trim_matches(char::from(0))
            .to_string();

        let instrument_cat1 = reader.read_u8()?;
        let instrument_cat2 = reader.read_u8()?;
        let instrument_cat3 = reader.read_u8()?;

        let instrument_url = String::from_utf8(reader.read_bytes(85)?)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.utf8_error()))?
            .trim_matches(char::from(0))
            .to_string();

        let u_b = reader.read_u32_le()?;

        // NOTE: most likely some kind of bitflag field, as values
        // in the wild are 0x00 (0) or 0x01 (32)
        let flags = reader.read_u32_le()?;

        // TODO: read as le bytes
        let md5_checksum = reader.read_bytes(16)?;
        let svn_revision = reader.read_u32_le()?;

        let crc32_fast = reader.read_u32_le()?.to_be_bytes();
        let decompressed_length = reader.read_u32_le()?;

        // seems all zero bytes
        let _padding = reader.read_bytes(32)?;

        Ok(Self {
            patch_type,
            patch_version,
            app_signature,
            created_at,
            u_a,
            number_of_zones,
            number_of_groups,
            number_of_instruments,
            pcm_data_len,
            is_monolith,
            min_supported_version,
            /// Almost always 0, 1 in Rise and Hit Library
            u_c,
            cat_icon_idx,
            instrument_author,
            instrument_cat1,
            instrument_cat2,
            instrument_cat3,
            instrument_url,
            u_b,
            flags,
            md5_checksum,
            svn_revision,
            crc32_fast,
            decompressed_length,
        })
    }
}

#[derive(PartialEq, Clone)]
pub struct NKIAppVersion {
    pub major: u8,
    pub minor_1: u8,
    pub minor_2: u8,
    pub minor_3: u8,
}

impl std::fmt::Debug for NKIAppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor_1, self.minor_2, self.minor_3
        )
    }
}

impl std::fmt::Display for NKIAppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}.{}.{}.{}",
            self.major, self.minor_2, self.minor_2, self.minor_3,
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatchType {
    NKM,
    NKI,
    NKB,
    NKP,
    NKG,
    NKZ,
    Unknown(u16),
}

impl PatchType {
    /// Get a meaningful string for a PatchType (FileTypeProxy).
    pub fn description(&self) -> String {
        match self {
            PatchType::NKB => "Bank",
            PatchType::NKG => "Group",
            PatchType::NKI => "Instrument",
            PatchType::NKM => "Multi",
            PatchType::NKP => "Preset",
            PatchType::NKZ => todo!(),
            PatchType::Unknown(_) => "?",
        }
        .into()
    }
}

impl From<u16> for PatchType {
    fn from(value: u16) -> Self {
        use PatchType::*;
        match value {
            0 => NKM,
            1 => NKI,
            2 => NKB,
            3 => NKP,
            4 => NKG,
            5 => NKZ,
            _ => Unknown(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_header_v1_read() -> Result<(), NKSError> {
        let file = File::open("tests/data/Objects/Kontakt/BPatchHeaderV1/BPatchHeaderV1-000")?;
        println!("{:?}", BPatchHeader::read_le(file)?);
        Ok(())
    }

    #[test]
    fn test_header_v2_read() -> Result<(), NKSError> {
        let file = File::open("tests/data/Objects/Kontakt/BPatchHeaderV2/BPatchHeaderV2-000")?;
        println!("{:?}", BPatchHeader::read_le(file)?);
        Ok(())
    }

    #[test]
    fn test_header_v42_read() -> Result<(), NKSError> {
        let file = File::open("tests/data/Objects/Kontakt/BPatchHeaderV42/BPatchHeaderV42-000")?;
        println!("{:?}", BPatchHeader::read_le(file)?);
        Ok(())
    }
}
