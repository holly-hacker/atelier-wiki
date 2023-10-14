use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod items;
mod presents;
mod rumors;

#[derive(Serialize, TypeDef)]
pub struct SophieData {
    pub item_data: Vec<items::Item>,
    pub present_info: presents::PresentInfo,
    pub rumors: Vec<rumors::Rumor>,
}

impl SophieData {
    pub fn read_all(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        let item_data = items::Item::read(pak_index).context("read items")?;
        info!("Read data for {} items", item_data.len());

        let present_info = presents::PresentInfo::read(pak_index).context("read presents")?;
        info!(
            "Read present info for {} friends",
            present_info.friend_present_info.len()
        );

        let rumors = rumors::Rumor::read(pak_index).context("read rumors")?;
        info!("Read data for {} items", rumors.len());

        Ok(Self {
            item_data,
            present_info,
            rumors,
        })
    }
}
