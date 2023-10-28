pub mod extract_maps;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
pub use extract_maps::MapInfoList;
use tracing::info;

use crate::extract_images::{Args, Category};
use crate::shared::xml::UiSpritesheetInfo;
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

    if category.is_none() || category == Some(Category::Misc) {
        info!("Extracting misc textures");

        let icons_directory = output_directory.join("icons");
        std::fs::create_dir_all(&icons_directory).context("create icons directory")?;

        let mut texture_cache = HashMap::new();

        let icons =
            UiSpritesheetInfo::read(pak_index, r"\saves\ui_cmn\gen_styles\uis_gen_a24_icons.xml")
                .context("read ui_spritesheet.xml")?;

        std::fs::write(
            icons_directory.join("fld_cut_tree.png"),
            icons
                .get_image_indexed(pak_index, &mut texture_cache, "gen_a24_icons07_fld", 2)?
                .encode_png()?,
        )?;

        std::fs::write(
            icons_directory.join("fld_fishing.png"),
            icons
                .get_image(pak_index, &mut texture_cache, "gen_a24_icons14_ic_fishing")?
                .encode_png()?,
        )?;

        std::fs::write(
            icons_directory.join("fld_monster.png"),
            icons
                .get_image_indexed(pak_index, &mut texture_cache, "gen_a24_icons14_ic_mon", 1)?
                .encode_png()?,
        )?;
    }

    Ok(())
}
