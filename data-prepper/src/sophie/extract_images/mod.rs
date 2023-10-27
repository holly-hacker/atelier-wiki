use std::path::Path;

use anyhow::Context;
use tracing::info;

use crate::extract_images::{Args, Category};
use crate::utils::images::extract_prefixed_with_texture_atlas;
use crate::utils::PakIndex;

const PATH_ITEMS: &str = "items";

pub fn extract_images(
    args: &Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    category: Option<Category>,
) -> anyhow::Result<()> {
    if category.is_none() || category == Some(Category::Items) {
        info!("Extracting item icons");
        const ITEM_PATTERN: &str = r"\Data\Win32\ui_JP\a17_item_l_*.g1t";
        extract_prefixed_with_texture_atlas(
            args,
            pak_index,
            ITEM_PATTERN,
            output_directory,
            PATH_ITEMS,
        )
        .context("extract item icons")?;
    }

    Ok(())
}
