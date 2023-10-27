pub mod extract_maps;

use std::path::Path;

use anyhow::Context;
pub use extract_maps::MapInfoList;
use tracing::info;

use crate::extract_images::{Args, Category};
use crate::utils::images::{extract_sprites_with_texture_atlas, ExtractSpritesOptions};
use crate::utils::PakIndex;

const PATH_MAPS: &str = "maps";

pub fn extract_images(
    args: &Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    category: Option<Category>,
) -> anyhow::Result<()> {
    if category.is_none() || category == Some(Category::Monsters) {
        info!("Extracting monster portraits");
        let options = ExtractSpritesOptions {
            pattern: r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t",
            subdirectory: "enemies",
            sprite_dimensions: (512, 512),
            texture_atlas_dimensions: (64, 64),
            ..Default::default()
        };
        extract_sprites_with_texture_atlas(args, pak_index, output_directory, options)
            .context("extract monster portraits")?;
    }

    if category.is_none() || category == Some(Category::Items) {
        info!("Extracting item icons");
        let options = ExtractSpritesOptions {
            pattern: r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t",
            subdirectory: "items",
            sprite_dimensions: (512, 512),
            texture_atlas_dimensions: (64, 64),
            ..Default::default()
        };
        extract_sprites_with_texture_atlas(args, pak_index, output_directory, options)
            .context("extract item icons")?;
    }

    if category.is_none() || category == Some(Category::Maps) {
        info!("Extracting map textures");
        extract_maps::extract_map_textures(args, pak_index, output_directory)
            .context("extract map textures")?;
    }

    Ok(())
}
