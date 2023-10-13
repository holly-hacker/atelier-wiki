pub mod data;
pub mod executable;

use std::path::Path;

use anyhow::Context;
use gust_pak::common::GameVersion;
use tracing::{debug, info};

use crate::{
    extract::write_data_to_file,
    utils::{game_slug, PakIndex},
};

pub fn extract(
    game_directory: &Path,
    mut pak_index: PakIndex,
    output_directory: &Path,
) -> anyhow::Result<()> {
    let output_directory = output_directory.join(game_slug(GameVersion::A24));

    debug!("reading executable data");
    let executable_data = executable::Ryza3ExecutableData::read_all(game_directory)
        .context("read executable data")?;

    debug!("reading game data");
    let data =
        data::Ryza3Data::read_all(&mut pak_index, &executable_data).context("read data files")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    info!("Writing files");

    debug!("Writing item data");
    write_data_to_file(&output_directory.join("items.json"), &data.item_data)
        .context("write item data")?;

    debug!("Writing item category data");
    write_data_to_file(
        &output_directory.join("item_categories.json"),
        &data.item_category_data,
    )
    .context("write item category data")?;

    debug!("Writing item effects data");
    write_data_to_file(
        &output_directory.join("item_effects.json"),
        &data.item_effect_data,
    )
    .context("write item data")?;

    debug!("Writing recipe data");
    write_data_to_file(&output_directory.join("recipes.json"), &data.recipe_data)
        .context("write recipe data")?;

    debug!("Writing field map");
    write_data_to_file(&output_directory.join("field_map.json"), &data.field_map)
        .context("write field map")?;

    debug!("Writing field data");
    write_data_to_file(&output_directory.join("field_data.json"), &data.field_data)
        .context("write field data")?;

    debug!("Writing enemy data");
    write_data_to_file(&output_directory.join("enemies.json"), &data.enemy_data)
        .context("write enemy data")?;

    info!("Wrote ryza3 data to {:?}", output_directory);

    Ok(())
}
