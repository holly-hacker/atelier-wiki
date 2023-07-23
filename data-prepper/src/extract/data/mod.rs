use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use super::pak_index::PakIndex;

mod items;
mod strings;
mod util;

#[derive(Serialize, TypeDef)]
pub struct Ryza3Data {
    pub item_data: Vec<items::ItemData>,
}

impl Ryza3Data {
    pub fn read_all(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        // TODO: consider reading other languages too
        let strings = util::read_xml(
            pak_index,
            r"\saves\text_en\strcombineall.xml",
            strings::StringsData::read,
        )
        .context("read strings")?;

        info!(
            "Read {} strings by id and {} strings by number",
            strings.id_lookup.len(),
            strings.no_lookup.len()
        );

        // NOTE: itemdata_no appears to be the exact same file
        let item_data = util::read_xml(pak_index, r"\saves\item\itemdata.xml", |doc| {
            items::ItemData::read(doc, &strings)
        })
        .context("read itemdata")?;

        info!("Read data for {} items", item_data.len());

        Ok(Self { item_data })
    }
}
