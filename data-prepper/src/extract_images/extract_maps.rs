use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use tracing::{debug, info};

use crate::{
    extract_images::rgba8_image::Rgba8Image,
    utils::{match_pattern_str, PakIndex},
};

use super::upload_manager::UploadManager;

const MAP_PATTERN_MINIMAP: &str = r"\data\x64\res_cmn\ui\neo\neo_minimap_ta_*.g1t";
const MAP_PATTERN_FULL: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_minimap_all_*.g1t";
const NEO_TILE_SIZE: usize = 2048;
const WEB_TILE_SIZE: usize = 256;

pub fn extract_map_textures(
    args: &super::Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    _upload_manager: &mut UploadManager,
) -> anyhow::Result<()> {
    let image_output_folder = output_directory.join(super::PATH_MAPS);
    if !args.dont_write_images {
        debug!("Creating image output directory");
        std::fs::create_dir_all(&image_output_folder).context("create image output directory")?;
    }

    let mut minimap_tiles = BTreeMap::new();

    pak_index
        .iter_entries()
        .filter_map(|e| {
            match_pattern_str(MAP_PATTERN_MINIMAP, e.get_file_name())
                .map(|num| (e, num.to_string()))
        })
        .filter_map(|(f, num)| {
            let file_name = f.get_file_name().to_string();
            let (map, index) = num.split_once('_')?;
            let (map, index) = (map.parse::<usize>().ok()?, index.parse::<usize>().ok()?);
            Some((map, (file_name, index)))
        })
        .for_each(|(map, tuple)| {
            minimap_tiles
                .entry(map)
                .or_insert_with(Vec::new)
                .push(tuple);
        });

    for (map_idx, tiles) in minimap_tiles {
        let span = tracing::debug_span!("map", num = map_idx);
        let _enter = span.enter();

        extract_map_texture(pak_index, map_idx, tiles, &image_output_folder)
            .with_context(|| format!("extract map texture for map {map_idx}"))?;
    }

    Ok(())
}

fn extract_map_texture(
    pak_index: &mut PakIndex,
    map_idx: usize,
    mut tiles: Vec<(String, usize)>,
    output_directory: &Path,
) -> anyhow::Result<()> {
    // TODO: not really needed, just for debugging purposes
    tiles.sort_by_key(|(_, idx)| *idx);

    let max_tile = tiles.iter().map(|(_, idx)| idx).max().unwrap();

    if max_tile + 1 != tiles.len() {
        info!(
            "Skipping map {map_idx} because it should have {} tiles but found {} (likely partial map)",
            max_tile + 1,
            tiles.len()
        );
        return Ok(());
    }

    debug!("Map has {} tiles", tiles.len());

    // calculate the width and height in terms of tiles
    let (tiles_x, tiles_y) = {
        let full_map = pak_index
            .get_file(&MAP_PATTERN_FULL.replace('*', &format!("{map_idx:02}")))
            .context("get full image")?
            .context("find full image")?;

        let untiled_map = gust_g1t::GustG1t::read(full_map).context("read header")?;
        let (w, h) = (
            untiled_map.textures[0].width,
            untiled_map.textures[0].height,
        );

        let aspect_ratio = w as f64 / h as f64;
        let tiles_x_squared = tiles.len() as f64 * aspect_ratio;
        let tiles_x = tiles_x_squared.sqrt().ceil() as usize;
        let tiles_y = tiles.len() / tiles_x;
        debug!("Untiled map has resolution {w}x{h} with {} tiles, so there should be {tiles_x} by {tiles_y} tiles.", tiles.len());

        (tiles_x, tiles_y)
    };

    // this check exists because map 6 in ryza3 is missing tiles, but we can't easily detect that
    // it's not a perfect check but it seems to work for now
    if tiles_x * tiles_y != tiles.len() {
        info!(
            "Skipping map {map_idx} because it has {} tiles ({tiles_x}x{tiles_y}) but should have {} tiles",
            tiles.len(),
            tiles_x * tiles_y
        );
        return Ok(());
    }

    // create the full-size image in memory
    // this takes up quite a bit of RAM (up to ~2.5gb), but it's the easiest (and likely fastest) way to do this
    // NOTE: last tile in each direction may be a different size, which is fine. we fill the void with transparency
    let full_size_width = tiles_x * NEO_TILE_SIZE;
    let full_size_height = tiles_y * NEO_TILE_SIZE;
    let mut full_size_image =
        Rgba8Image::new_empty(full_size_width as u32, full_size_height as u32);
    for (tile_name, input_tile_idx) in tiles {
        let (input_tile_idx_x, input_tile_idx_y) =
            (input_tile_idx % tiles_x, input_tile_idx / tiles_x);

        let input_tile_image = {
            let mut tile = pak_index
                .get_file(&tile_name)
                .context("get tile")?
                .context("find tile")?;

            let tile_g1t = gust_g1t::GustG1t::read(&mut tile)
                .with_context(|| format!("read tile header {input_tile_idx}"))?;

            let decoded_tile = tile_g1t
                .read_image(&tile_g1t.textures[0], tile)
                .context("decode tile")?;

            // IMPORTANT: the tile is not always 2048x2048! we can't just blindly pass NEO_TILE_SIZE here
            Rgba8Image::new(tile_g1t.textures[0].width, decoded_tile)
                .context("create tile image")?
        };

        full_size_image.blit(
            (input_tile_idx_x * NEO_TILE_SIZE) as u32,
            (input_tile_idx_y * NEO_TILE_SIZE) as u32,
            &input_tile_image,
        )?;
    }

    // remove mut
    let full_size_image = full_size_image;

    // leaflet expects square images of equal size which is not guaranteed for the full map, so we need to pad the image
    // with transparency to make it square
    let padded_image_dimension = full_size_image
        .width()
        .max(full_size_image.height())
        .next_power_of_two();

    // calculate the zoom levels
    // we want zoom level 0 to be a single image for the full map (scaled to 256x256), and we want the deepest zoom
    // level to be unscaled 256x256 images
    let zoom_levels = (padded_image_dimension as f64 / WEB_TILE_SIZE as f64)
        .log2()
        .ceil() as usize;

    debug!(zoom_levels);

    for zoom_level in 0..zoom_levels {
        let span = tracing::debug_span!("zoom_level", zoom = zoom_level);
        let _enter = span.enter();

        let pixels_per_tile = padded_image_dimension >> zoom_level;
        let tiles_width_padded = 1 << zoom_level;
        let scale_factor = pixels_per_tile / WEB_TILE_SIZE as u32;

        for tile_y in 0..tiles_width_padded {
            for tile_x in 0..tiles_width_padded {
                let start_x = tile_x * pixels_per_tile;
                let start_y = tile_y * pixels_per_tile;

                if start_x >= full_size_image.width() || start_y >= full_size_image.height() {
                    // image would be out of bounds. we could write a transparent image here but that's pointless
                    debug!(start_x, start_y, "Skipping tile, out of bounds");
                    continue;
                }

                // TODO: take a slice/borrow instead of a copy? we may double the peak memory usage here (to 5gb!)
                let unscaled_tile = full_size_image.copy_chunk(
                    start_x,
                    start_y,
                    pixels_per_tile.min(full_size_image.width() - (start_x)),
                    pixels_per_tile.min(full_size_image.height() - (start_y)),
                ).with_context(|| format!("copy chunk from tile index {tile_x},{tile_y} at zoom level {zoom_level}"))?;
                let scaled_tile = unscaled_tile.scale_down((scale_factor, scale_factor));
                // TODO: maybe pad to 256x256? check if leaflet pads non-square images by themselves

                let path = format!(
                    "{map_idx}/{zoom_level}/{y}_{x}.webp",
                    y = tile_y,
                    x = tile_x,
                );
                debug!(path, "Image decoded");

                let path = output_directory.join(path);

                std::fs::create_dir_all(Path::new(&path).parent().unwrap())
                    .context("create image output directory")?;

                let encoded = scaled_tile.encode_webp().context("encode image")?;
                std::fs::write(&path, encoded).context("write image")?;
            }
        }
    }

    Ok(())
}
