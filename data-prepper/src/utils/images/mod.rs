use std::path::Path;

use anyhow::{bail, Context};
use tracing::debug;

use self::rgba8_image::Rgba8Image;
use self::texture_atlas::UniformTextureAtlas;
use super::{match_pattern, PakIndex};
use crate::extract_images::Args;

pub mod rgba8_image;
pub mod texture_atlas;

pub struct ExtractSpritesOptions {
    /// The pattern used to find file names
    pub pattern: &'static str,
    /// The subdirectory in the output directory to put the individual images in. This is also the
    /// name of the texture atlas file.
    pub subdirectory: &'static str,
    /// The size of each individual input image.
    pub sprite_dimensions: (u32, u32),
    /// The size of each item in the texture atlas
    pub texture_atlas_dimensions: (u32, u32),
}

pub fn extract_sprites_with_texture_atlas(
    args: &Args,
    pak_index: &mut PakIndex,
    output_directory: &Path,
    options: ExtractSpritesOptions,
) -> anyhow::Result<()> {
    let image_output_folder = output_directory.join(options.subdirectory);
    if !args.dont_write_images {
        debug!("Creating image output directory");
        std::fs::create_dir_all(&image_output_folder).context("create image output directory")?;
    }

    let mut entries: Vec<_> = pak_index
        .iter_entries()
        .filter_map(|e| {
            match_pattern::<usize>(options.pattern, e.get_file_name()).map(|num| (e, num))
        })
        .map(|(f, num)| (f.get_file_name().to_string(), num))
        .collect();

    entries.sort_by_key(|(_, num)| *num);

    // create texture atlas
    let mut texture_atlas = UniformTextureAtlas::new_with_scaling(
        options.sprite_dimensions,
        options.texture_atlas_dimensions,
        entries.len(),
    )
    .context("create texture atlas")?;

    for (entry, num) in entries {
        let mut file = pak_index
            .get_file(&entry)
            .with_context(|| format!("read {entry}"))?
            .with_context(|| format!("cannot find entry {entry}"))?;

        debug!(?entry, "reading g1t header");
        let g1t = gust_g1t::GustG1t::read(&mut file).context("read g1t")?;
        let texture = &g1t.textures[0];

        if (texture.width, texture.height) != options.sprite_dimensions {
            bail!(
                "Texture {entry} has invalid size {}x{}, expected {}x{}",
                texture.width,
                texture.height,
                options.sprite_dimensions.0,
                options.sprite_dimensions.1,
            );
        }

        debug!(?entry, "reading image");
        let image_bytes = g1t.read_image(texture, &mut file).context("read image")?;
        let image = Rgba8Image::new(texture.width, image_bytes).context("image buffer to image")?;
        debug_assert_eq!(image.height(), texture.height);

        debug!(?entry, "adding image to texture atlas");
        texture_atlas
            .add_image(&image, num.to_string())
            .context("add image to texture atlas")?;

        save_image(args, image, &image_output_folder, &format!("{}.png", num))
            .with_context(|| format!("save image {num}"))?;
    }

    // save the texture atlas info
    let atlas_directory = output_directory.join("texture-atlasses");
    std::fs::create_dir_all(&atlas_directory).context("create atlas directory")?;
    crate::extract::write_data_to_file(
        &atlas_directory.join(format!("{}.json", options.subdirectory)),
        &texture_atlas.create_info(),
    )
    .context("write texture atlas info")?;

    // save the texture atlas image
    save_image(
        args,
        texture_atlas.into_image(),
        &image_output_folder,
        "packed.webp",
    )
    .context("save texture atlas")?;

    Ok(())
}

fn save_image(
    args: &Args,
    image: Rgba8Image,
    output_folder: &Path,
    file_name: &str,
) -> anyhow::Result<()> {
    let file_path = output_folder.join(file_name);

    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    debug!(?file_path, "encoding image to {ext}...");
    let image_bytes = match ext {
        "png" => match args.compression {
            Some(compression) => image
                .encode_oxipng(compression)
                .context("encode using oxipng"),
            None => image.encode_png().context("encode png"),
        }?,
        "webp" => image.encode_webp().context("encode webp")?,
        _ => bail!("Unknown image extension {}", ext),
    };

    if !args.dont_write_images {
        debug!(?file_path, "saving image...");
        std::fs::write(&file_path, image_bytes).context("write to image file")?;
        debug!(?file_path, "saved image");
    }

    Ok(())
}
