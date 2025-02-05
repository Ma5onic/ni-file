use crate::{
    kontakt::{objects::BPatchHeaderV42, KontaktPatch},
    nis::{BNISoundHeader, BNISoundPresetProperties, EncryptionItem, ItemContainer, ItemType},
    Error,
};

use super::PresetChunkItem;

#[derive(Debug)]
pub struct BNISoundPreset(ItemContainer);

impl BNISoundPreset {
    pub fn header(&self) -> Result<BPatchHeaderV42, Error> {
        self.0
            .find_item::<BNISoundHeader>(&ItemType::BNISoundHeader)
            .ok_or(Error::Static("No BNISoundHeader"))?
            .map(|b| b.0)
    }

    pub fn encryption_item(&self) -> Result<EncryptionItem, Error> {
        self.0
            .find_item::<EncryptionItem>(&ItemType::EncryptionItem)
            .ok_or(Error::Static("No EncryptionItem"))?
    }

    pub fn patch(&self) -> Result<KontaktPatch, Error> {
        let preset_chunks: PresetChunkItem = self.encryption_item()?.subtree.item()?.into();

        Ok(KontaktPatch {
            header: self.header()?,
            data: preset_chunks.properties()?.0,
        })
    }

    pub fn properties(&self) -> Result<BNISoundPresetProperties, Error> {
        (&self.0.data).try_into()
    }

    // pub fn sound_info_item(&self) -> Result<SoundInfoItem, Error> {}

    pub fn children(&self) -> &Vec<ItemContainer> {
        &self.0.children
    }
}

impl From<ItemContainer> for BNISoundPreset {
    fn from(ic: ItemContainer) -> Self {
        Self(ic)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_bni_sound_preset_001() -> Result<(), Error> {
        let file = File::open("tests/data/Containers/NIS/files/BNISoundPreset/fx-001.nki")?;
        let preset: BNISoundPreset = ItemContainer::read(file)?.into();
        let _header = preset.header()?;
        Ok(())
    }
}
