pub mod data;
mod extract_images;

use std::path::Path;

use anyhow::Context;
pub use extract_images::extract_images;
use gust_pak::common::GameVersion;
use tracing::{debug, info};

use crate::extract::write_data_to_file;
use crate::utils::{game_slug, PakIndex};

pub fn extract(mut pak_index: PakIndex, output_directory: &Path) -> anyhow::Result<()> {
    let output_directory = output_directory.join(game_slug(GameVersion::A17));

    debug!("reading game data");
    let data = data::SophieData::read_all(&mut pak_index).context("read data files")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    info!("Writing files");

    debug!("Writing item data");
    write_data_to_file(&output_directory.join("items.json"), &data.item_data)
        .context("write item data")?;

    debug!("Writing present data");
    write_data_to_file(&output_directory.join("presents.json"), &data.present_info)
        .context("write present data")?;

    debug!("Writing rumor data");
    write_data_to_file(&output_directory.join("rumors.json"), &data.rumors)
        .context("write rumor data")?;

    debug!("Writing doll data");
    write_data_to_file(&output_directory.join("dolls.json"), &data.doll_making_data)
        .context("write doll data")?;
    info!("Wrote sophie data to {:?}", output_directory);

    Ok(())
}
