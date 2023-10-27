pub mod extract_maps;

use std::path::Path;

use anyhow::Context;
pub use extract_maps::MapInfoList;
use tracing::info;

use crate::extract_images::{Args, Category};
use crate::utils::images::extract_prefixed_with_texture_atlas;
use crate::utils::PakIndex;

const PATH_ITEMS: &str = "items";
const PATH_ENEMIES: &str = "enemies";
const PATH_MAPS: &str = "maps";

pub fn extract_images(
    args: &Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    category: Option<Category>,
) -> anyhow::Result<()> {
    if category.is_none() || category == Some(Category::Monsters) {
        info!("Extracting monster portraits");
        const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
        extract_prefixed_with_texture_atlas(
            args,
            pak_index,
            MONSTER_PATTERN,
            output_directory,
            PATH_ENEMIES,
        )
        .context("extract monster portraits")?;
    }

    if category.is_none() || category == Some(Category::Items) {
        info!("Extracting item icons");
        const ITEM_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t";
        extract_prefixed_with_texture_atlas(
            args,
            pak_index,
            ITEM_PATTERN,
            output_directory,
            PATH_ITEMS,
        )
        .context("extract item icons")?;
    }

    if category.is_none() || category == Some(Category::Maps) {
        info!("Extracting map textures");
        extract_maps::extract_map_textures(args, pak_index, output_directory)
            .context("extract map textures")?;
    }

    Ok(())
}
