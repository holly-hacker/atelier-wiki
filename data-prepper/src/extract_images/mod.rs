mod rgba8_image;
mod texture_atlas;
mod upload_manager;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use argh::FromArgs;
use tracing::{debug, info};

use crate::{
    config::Config,
    extract_images::{
        rgba8_image::Rgba8Image, texture_atlas::UniformTextureAtlas, upload_manager::UploadManager,
    },
    utils::{extract_game_version, match_pattern, PakIndex},
};

const PATH_ITEMS: &str = "items";
const PATH_ENEMIES: &str = "enemies";

/// Extract and prepare the game data from the game install directory
#[derive(FromArgs)]
#[argh(subcommand, name = "extract-images")]
pub struct Args {
    /// the game install directory
    #[argh(option, short = 'i')]
    game_directory: PathBuf,

    /// the output directory
    #[argh(option, short = 'o')]
    output_directory: Option<PathBuf>,

    /// level of oxipng compression to use. if not present, use standard png encoder
    #[argh(option, short = 'c')]
    compression: Option<u8>,

    /// upload the extracted files to S3-compatible object storage
    #[argh(switch)]
    upload: bool,
}

impl Args {
    pub fn handle(self, config: Option<Config>) -> anyhow::Result<()> {
        let mut upload_manager = UploadManager::new();
        if self.upload {
            let Some(config) = &config else {
                bail!("Cannot upload without a valid config");
            };

            let Some(upload_config) = &config.upload else {
                bail!("Cannot upload without a valid config (missing [upload] section)");
            };

            upload_manager
                .load_object_storage(upload_config.clone())
                .context("load object storage config")?;
        }

        let output_directory = self
            .output_directory
            .unwrap_or_else(|| PathBuf::from("game_data/images"));
        debug!(?output_directory);

        debug!("Detecting game version");
        let game_version = extract_game_version(&self.game_directory);
        let Some(game_version) = game_version else {
            bail!("Could not detect game version in the given install directory");
        };
        info!(
            "Detected game {:?} ({})",
            game_version,
            game_version.get_short_name()
        );

        // loading index of game files
        debug!("Reading pak file index");
        let pak_dir = self.game_directory.join("Data");
        let mut pak_index = PakIndex::read(&pak_dir, game_version).context("read data dir")?;
        info!("Loaded pak file index with {} entries", pak_index.len());

        extract_images(
            &mut pak_index,
            &output_directory,
            self.compression,
            &mut upload_manager,
        )?;
        info!("Extracted images");

        Ok(())
    }
}

fn extract_images(
    pak_index: &mut PakIndex,
    output_directory: &Path,
    opt_level: Option<u8>,
    upload_manager: &mut UploadManager,
) -> anyhow::Result<()> {
    info!("Extracting monster portraits");
    const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
    let monsters_path = output_directory.join(PATH_ENEMIES);
    extract_prefixed_with_texture_atlas(
        pak_index,
        MONSTER_PATTERN,
        &monsters_path,
        opt_level,
        upload_manager,
        PATH_ENEMIES,
    )
    .context("extract monster portraits")?;

    info!("Extracting item icons");
    const ITEM_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t";
    let items_path = output_directory.join(PATH_ITEMS);
    extract_prefixed_with_texture_atlas(
        pak_index,
        ITEM_PATTERN,
        &items_path,
        opt_level,
        upload_manager,
        PATH_ITEMS,
    )
    .context("extract item icons")?;

    Ok(())
}

fn extract_prefixed_with_texture_atlas(
    pak_index: &mut PakIndex,
    pattern: &'static str,
    output_folder: &Path,
    opt_level: Option<u8>,
    upload_manager: &mut UploadManager,
    object_storage_path: &'static str,
) -> anyhow::Result<()> {
    debug!("Creating output directory");
    std::fs::create_dir_all(output_folder).context("create output directory")?;

    let mut entries: Vec<_> = pak_index
        .iter_entries()
        .filter_map(|e| match_pattern(pattern, e.get_file_name()).map(|num| (e, num)))
        .map(|(f, num)| (f.get_file_name().to_string(), num))
        .collect();

    entries.sort_by_key(|(_, num)| *num);

    // create texture atlas
    let mut texture_atlas =
        UniformTextureAtlas::new_with_scaling((512, 512), (64, 64), entries.len())
            .context("create texture atlas")?;

    for (entry, num) in entries {
        let mut file = pak_index
            .get_file(&entry)
            .with_context(|| format!("read {entry}"))?
            .with_context(|| format!("cannot find entry {entry}"))?;

        debug!(?entry, "reading g1t header");
        let g1t = gust_g1t::GustG1t::read(&mut file).context("read g1t")?;
        let texture = &g1t.textures[0];

        if texture.width != 512 && texture.height != 512 {
            bail!(
                "Texture {entry} has invalid size {}x{}, expected 512x512",
                texture.width,
                texture.height,
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

        save_image(
            image,
            output_folder,
            object_storage_path,
            &format!("{}.png", num),
            opt_level,
            upload_manager,
        )
        .with_context(|| format!("save image {num}"))?;
    }

    // save the texture atlas image
    save_image(
        texture_atlas.into_image(),
        output_folder,
        object_storage_path,
        "packed.png",
        opt_level,
        upload_manager,
    )
    .context("save texture atlas")?;

    Ok(())
}

fn save_image(
    image: Rgba8Image,
    output_folder: &Path,
    object_storage_path: &str,
    file_name: &str,
    opt_level: Option<u8>,
    upload_manager: &mut UploadManager,
) -> anyhow::Result<()> {
    let file_path = output_folder.join(file_name);

    debug!(?file_path, "encoding image to png...");
    let png_bytes = match opt_level {
        Some(compression) => image
            .encode_oxipng(compression)
            .context("encode using oxipng"),
        None => image.encode_png().context("encode png"),
    }?;

    debug!(?file_path, "saving image...");
    std::fs::write(&file_path, &png_bytes).context("write to png file")?;
    debug!(?file_path, "saved image");

    // store image for upload
    upload_manager
        .upload(
            &format!("{}/{}", object_storage_path, file_name),
            &png_bytes,
        )
        .context("upload to s3")?;

    Ok(())
}
