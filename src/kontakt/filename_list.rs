use std::collections::HashMap;

use crate::{read_bytes::ReadBytesExt, Error};

use super::chunkdata::ChunkData;

#[derive(Debug)]
pub struct FNTableImpl {
    pub filenames: HashMap<u32, String>,
}

#[derive(Debug)]
pub struct FileNameListPreK51 {
    pub filenames: HashMap<u32, String>,
}

impl std::convert::TryFrom<&ChunkData> for FileNameListPreK51 {
    type Error = Error;

    fn try_from(chunk: &ChunkData) -> Result<Self, Self::Error> {
        if chunk.id != 0x3d {
            panic!("fixme: error here");
        }
        let reader = std::io::Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

impl std::convert::TryFrom<&ChunkData> for FNTableImpl {
    type Error = Error;

    fn try_from(chunk: &ChunkData) -> Result<Self, Self::Error> {
        if chunk.id != 0x4b {
            panic!("fixme: error here");
        }
        let reader = std::io::Cursor::new(&chunk.data);
        Self::read(reader)
    }
}

impl FNTableImpl {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let version = reader.read_u16_le()?;
        assert!(version == 2);

        let _ = reader.read_u32_le()?;
        let _ = reader.read_u32_le()?;
        let file_count = reader.read_u32_le()?;

        let mut filenames = HashMap::new();
        for i in 0..file_count {
            let segments = reader.read_i32_le()?;

            let mut filename = Vec::new();
            for _ in 0..segments {
                let _segment_type = reader.read_i8()?;
                let segment = reader.read_widestring_utf16()?;
                filename.push(segment);
            }

            filenames.insert(i, filename.join("/"));
        }

        Ok(Self { filenames })
    }
}

impl FileNameListPreK51 {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        let _ = reader.read_u32_le()?;
        let file_count = reader.read_u32_le()?;

        let mut filenames = HashMap::new();
        for i in 0..file_count {
            let segments = reader.read_i32_le()?;

            let mut filename = Vec::new();
            for _ in 0..segments {
                let _segment_type = reader.read_i8()?;
                let segment = reader.read_widestring_utf16()?;
                filename.push(segment);
            }

            filenames.insert(i, filename.join("/"));
        }

        Ok(Self { filenames })
    }
}

struct BFileName;

impl BFileName {
    // K4PatchLib::BFileName::Retrieve
    pub fn read_filename<R: ReadBytesExt>(mut reader: R) -> Result<BFileName, Error> {
        let i = reader.read_i32_le()?;
        if i < 0 {
            reader.read_widestring_utf16()?;
        } else if i > 0 {
        }

        Ok(BFileName)
    }
}

struct BFileNameSegment;

impl BFileNameSegment {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<BFileNameSegment, Error> {
        let i = reader.read_i8()?;
        // if i < 11
        if i < 0xb {
            let _a = reader.read_u16_le()?;
            // if (0x316U >> (uVar5 & 0x1F)) & 1 == 0 {}
            if false {
                match i {
                    5 => {}
                    10 => {}
                    _ => panic!(),
                }
            } else {
                let _s = reader.read_widestring_utf16()?;
            }
        }
        if i < 0 {
            reader.read_widestring_utf16()?;
        } else if i > 0 {
        }

        Ok(BFileNameSegment)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_structured_object() -> Result<(), Error> {
        let file = File::open("tests/patchdata/KontaktV42/filename_list_pre_k5/4.2.2.4504/000")?;
        FileNameListPreK51::read(file)?;

        Ok(())
    }
}
