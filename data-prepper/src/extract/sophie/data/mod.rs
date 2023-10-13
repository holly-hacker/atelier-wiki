use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod items;

#[derive(Serialize, TypeDef)]
pub struct SophieData {
    pub item_data: Vec<items::Item>,
}

impl SophieData {
    pub fn read_all(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        let item_data = items::Item::read(pak_index).context("read items")?;
        info!("Read data for {} items", item_data.len());

        Ok(Self { item_data })
    }
}
