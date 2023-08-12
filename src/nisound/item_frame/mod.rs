pub mod app_id;
pub mod domain_id;
pub mod item_frame_header;
pub mod item_id;

pub use item_frame_header::ItemFrameHeader;

use crate::{prelude::*, read_bytes::ReadBytesExt};
use item_id::ItemID;
use std::io::{Cursor, Read};

#[derive(Clone, Debug)]
pub struct ItemFrame {
    pub header: ItemFrameHeader,
    pub inner: Option<Box<ItemFrame>>,
    pub data: Vec<u8>,
}

impl ItemFrame {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        let header = ItemFrameHeader::read(&mut reader)?;
        let length = header.length as usize - 20;

        match header.item_id {
            ItemID::Item => {
                let data = reader.read_bytes(length)?;

                Ok(Self {
                    header,
                    inner: None,
                    data,
                })
            }
            _ => {
                let mut buf = Cursor::new(reader.read_bytes(length)?);
                let inner = ItemFrame::read(&mut buf)?;
                let mut data = Vec::new();
                buf.read_to_end(&mut data)?;

                Ok(Self {
                    header,
                    inner: Some(Box::new(inner)),
                    data,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_frame_read_000() -> Result<()> {
        let file = std::io::Cursor::new(include_bytes!(
            "../../../tests/patchdata/NISD/ItemFrame/RepositoryRoot-000"
        ));
        let item = ItemFrame::read(file)?;
        assert_eq!(item.data.len(), 58);

        assert_eq!(item.header.item_id, ItemID::RepositoryRoot);
        assert_eq!(item.inner.unwrap().header.item_id, ItemID::Authorization);

        Ok(())
    }

    #[test]
    fn test_item_frame_read_001() -> Result<()> {
        let file = std::io::Cursor::new(include_bytes!(
            "../../../tests/patchdata/NISD/ItemFrame/RepositoryRoot-001"
        ));
        let item = ItemFrame::read(file)?;
        assert_eq!(item.data.len(), 58);

        assert_eq!(item.header.item_id, ItemID::RepositoryRoot);
        assert_eq!(item.inner.unwrap().header.item_id, ItemID::Authorization);

        Ok(())
    }
}
