use std::io::Cursor;

use crate::{
    kontakt::{error::KontaktError, structured_object::StructuredObject, Chunk},
    read_bytes::ReadBytesExt,
    Error,
};

pub const KONTAKT_BPARFX_ID: u16 = 0x25;

#[derive(Debug)]
pub struct BParFX(StructuredObject);

impl BParFX {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(StructuredObject::read(&mut reader)?))
    }
}

impl std::convert::TryFrom<&Chunk> for BParFX {
    type Error = Error;

    fn try_from(chunk: &Chunk) -> Result<Self, Self::Error> {
        if chunk.id != KONTAKT_BPARFX_ID {
            return Err(KontaktError::IncorrectID {
                expected: KONTAKT_BPARFX_ID,
                got: chunk.id,
            }
            .into());
        }
        let reader = Cursor::new(&chunk.data);
        Self::read(reader)
    }
}
