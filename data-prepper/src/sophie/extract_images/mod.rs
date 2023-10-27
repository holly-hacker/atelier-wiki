use std::path::Path;

use anyhow::Context;
use tracing::info;

use crate::extract_images::{Args, Category};
use crate::utils::images::{extract_sprites_with_texture_atlas, ExtractSpritesOptions};
use crate::utils::PakIndex;

pub fn extract_images(
    args: &Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    category: Option<Category>,
) -> anyhow::Result<()> {
    if category.is_none() || category == Some(Category::Items) {
        info!("Extracting item icons");
        let options = ExtractSpritesOptions {
            pattern: r"\Data\Win32\ui_JP\a17_item_l_*.g1t",
            subdirectory: "items",
            sprite_dimensions: (512, 512),
            sprite_trimmed_dimensions: Some((288, 288)),
            // TODO: do we want 64 instead? we can only do integer scaling for now
            texture_atlas_dimensions: (72, 72),
        };
        extract_sprites_with_texture_atlas(args, pak_index, output_directory, options)
            .context("extract item icons")?;
    }

    Ok(())
}
