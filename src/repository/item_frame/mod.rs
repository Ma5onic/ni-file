pub mod app_id;
pub mod domain_id;
pub mod item_frame_header;
pub mod item_id;

// frame data
pub mod authorization;
pub mod bni_sound_preset;
pub mod encryption_item;
pub mod preset;
pub mod repository_root;
pub mod sound_info;
pub mod sound_info_item;
pub mod subtree_item;

use std::convert::TryFrom;

pub use item_frame_header::ItemFrameHeader;

use crate::{prelude::*, read_bytes::ReadBytesExt};

use super::item_frame_stack::ItemFrameStack;

#[derive(Clone, Debug)]
pub struct ItemFrame {
    pub header: ItemFrameHeader,
    pub inner: ItemFrameStack,
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<&ItemFrameStack> for ItemFrame {
    type Error = NIFileError;

    fn try_from(stack: &ItemFrameStack) -> Result<Self> {
        ItemFrame::read(stack.0.as_slice())
    }
}

impl ItemFrame {
    pub fn read<R: ReadBytesExt>(mut reader: R) -> Result<Self> {
        log::debug!("ItemFrame::read");

        let buf = reader.read_sized_data()?;
        let mut buf = buf.as_slice();
        let header = ItemFrameHeader::read(&mut buf)?;
        let inner = ItemFrameStack::read(&mut buf)?;

        Ok(Self {
            header,
            inner,
            data: buf.to_vec(),
        })
    }

    pub fn inner(&self) -> Result<ItemFrame> {
        ItemFrame::try_from(&self.inner)
    }
}
