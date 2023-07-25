use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use super::pak_index::PakIndex;

mod enemies;
mod items;
mod strings;
mod util;

#[derive(Serialize, TypeDef)]
pub struct Ryza3Data {
    pub item_data: Vec<items::ItemData>,
    pub enemy_data: Vec<enemies::Enemy>,
}

impl Ryza3Data {
    pub fn read_all(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        // TODO: consider reading other languages too
        let strings = strings::StringsData::read(pak_index).context("read strings")?;

        info!(
            "Read {} strings by id and {} strings by number",
            strings.id_lookup.len(),
            strings.no_lookup.len()
        );

        // NOTE: itemdata_no appears to be the exact same file
        let item_data = items::ItemData::read(pak_index, &strings).context("read itemdata")?;
        info!("Read data for {} items", item_data.len());

        let enemy_data = enemies::read(pak_index, &strings).context("read enemy data")?;
        info!("Read data for {} enemies", enemy_data.len());

        Ok(Self {
            item_data,
            enemy_data,
        })
    }
}
