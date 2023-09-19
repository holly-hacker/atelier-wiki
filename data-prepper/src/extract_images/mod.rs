mod rgba8_image;
mod texture_atlas;
mod upload_manager;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use argh::FromArgs;
pub use texture_atlas::UniformTextureAtlasInfo;
use tracing::{debug, info};

use crate::{
    config::Config,
    extract_images::{
        rgba8_image::Rgba8Image, texture_atlas::UniformTextureAtlas, upload_manager::UploadManager,
    },
    utils::{extract_game_version, game_slug, match_pattern_str, PakIndex},
};

const PATH_ITEMS: &str = "items";
const PATH_ENEMIES: &str = "enemies";
const PATH_MAPS: &str = "maps";

/// Extract and prepare the game data from the game install directory
#[derive(FromArgs)]
#[argh(subcommand, name = "extract-images")]
pub struct Args {
    /// the game install directory
    #[argh(option, short = 'i')]
    game_directory: PathBuf,

    /// the output directory for the generated files
    #[argh(option, short = 'o')]
    output_directory: Option<PathBuf>,

    /// don't write images to disk (this does not prevent in-memory decoding)
    #[argh(switch, short = 'd')]
    dont_write_images: bool,

    /// level of oxipng compression to use. if not present, use standard png encoder for png
    #[argh(option, short = 'c')]
    compression: Option<u8>,

    /// upload the extracted files to S3-compatible object storage
    #[argh(switch)]
    upload: bool,
}

impl Args {
    pub fn handle(self, config: Option<Config>) -> anyhow::Result<()> {
        debug!("Detecting game version");
        let game_version = extract_game_version(&self.game_directory);
        let Some(game_version) = game_version else {
            bail!("Could not detect game version in the given install directory");
        };
        let slug = game_slug(game_version);
        info!(
            "Detected game {game_version:?} ({}), using slug {slug}",
            game_version.get_short_name()
        );

        let mut upload_manager = UploadManager::new_with_prefix(format!("game-data/{slug}/"));
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

        let output_dir = self
            .output_directory
            .clone()
            .unwrap_or_else(|| PathBuf::from("game_data"));
        debug!(?output_dir);

        // loading index of game files
        debug!("Reading pak file index");
        let pak_dir = self.game_directory.join("Data");
        let mut pak_index = PakIndex::read(&pak_dir, game_version).context("read data dir")?;
        info!("Loaded pak file index with {} entries", pak_index.len());

        let output_directory = output_dir.join(slug);

        debug!("Creating output directory");
        std::fs::create_dir_all(&output_directory).context("create output directory")?;

        self.extract_images(&mut pak_index, &output_directory, &mut upload_manager)?;
        info!("Extracted images");

        Ok(())
    }

    fn extract_images(
        &self,
        pak_index: &mut PakIndex,
        output_directory: &Path,
        upload_manager: &mut UploadManager,
    ) -> anyhow::Result<()> {
        info!("Extracting monster portraits");
        const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
        self.extract_prefixed_with_texture_atlas(
            pak_index,
            MONSTER_PATTERN,
            output_directory,
            upload_manager,
            PATH_ENEMIES,
        )
        .context("extract monster portraits")?;

        info!("Extracting item icons");
        const ITEM_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t";
        self.extract_prefixed_with_texture_atlas(
            pak_index,
            ITEM_PATTERN,
            output_directory,
            upload_manager,
            PATH_ITEMS,
        )
        .context("extract item icons")?;

        info!("Extracting map textures");
        const MAP_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_minimap_ta_*.g1t";
        self.extract_prefixed(
            pak_index,
            MAP_PATTERN,
            output_directory,
            upload_manager,
            PATH_MAPS,
        )
        .context("extract map textures")?;

        Ok(())
    }

    fn extract_prefixed(
        &self,
        pak_index: &mut PakIndex,
        pattern: &'static str,
        output_directory: &Path,
        upload_manager: &mut UploadManager,
        subdirectory: &'static str,
    ) -> anyhow::Result<()> {
        let image_output_folder = output_directory.join(subdirectory);
        if !self.dont_write_images {
            debug!("Creating image output directory");
            std::fs::create_dir_all(&image_output_folder)
                .context("create image output directory")?;
        }

        let mut entries: Vec<_> = pak_index
            .iter_entries()
            .filter_map(|e| {
                match_pattern_str(pattern, e.get_file_name()).map(|num| (e, num.to_string()))
            })
            .map(|(f, num)| (f.get_file_name().to_string(), num))
            .collect();

        entries.sort_by(|(_, a), (_, b)| a.cmp(b));

        for (entry, num) in entries {
            let mut file = pak_index
                .get_file(&entry)
                .with_context(|| format!("read {entry}"))?
                .with_context(|| format!("cannot find entry {entry}"))?;

            debug!(?entry, "reading g1t header");
            let g1t = gust_g1t::GustG1t::read(&mut file).context("read g1t")?;
            let texture = &g1t.textures[0];

            if texture.width != 2048 && texture.height != 2048 {
                bail!(
                    "Texture {entry} has invalid size {}x{}, expected 2048x2048",
                    texture.width,
                    texture.height,
                );
            }

            debug!(?entry, "reading image");
            let image_bytes = g1t.read_image(texture, &mut file).context("read image")?;
            let image =
                Rgba8Image::new(texture.width, image_bytes).context("image buffer to image")?;
            debug_assert_eq!(image.height(), texture.height);

            self.save_and_upload_image(
                image,
                &image_output_folder,
                subdirectory,
                &format!("{}.webp", num),
                upload_manager,
            )
            .with_context(|| format!("save image {num}"))?;
        }

        Ok(())
    }

    fn extract_prefixed_with_texture_atlas(
        &self,
        pak_index: &mut PakIndex,
        pattern: &'static str,
        output_directory: &Path,
        upload_manager: &mut UploadManager,
        subdirectory: &'static str,
    ) -> anyhow::Result<()> {
        let image_output_folder = output_directory.join(subdirectory);
        if !self.dont_write_images {
            debug!("Creating image output directory");
            std::fs::create_dir_all(&image_output_folder)
                .context("create image output directory")?;
        }

        let mut entries: Vec<_> = pak_index
            .iter_entries()
            .filter_map(|e| {
                match_pattern_str(pattern, e.get_file_name()).map(|num| (e, num.to_string()))
            })
            .map(|(f, num)| (f.get_file_name().to_string(), num))
            .collect();

        entries.sort_by(|(_, a), (_, b)| a.cmp(b));

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
            let image =
                Rgba8Image::new(texture.width, image_bytes).context("image buffer to image")?;
            debug_assert_eq!(image.height(), texture.height);

            debug!(?entry, "adding image to texture atlas");
            texture_atlas
                .add_image(&image, num.to_string())
                .context("add image to texture atlas")?;

            self.save_and_upload_image(
                image,
                &image_output_folder,
                subdirectory,
                &format!("{}.png", num),
                upload_manager,
            )
            .with_context(|| format!("save image {num}"))?;
        }

        // save the texture atlas info
        let atlas_directory = output_directory.join("texture-atlasses");
        std::fs::create_dir_all(&atlas_directory).context("create atlas directory")?;
        super::extract::write_data_to_file(
            &atlas_directory.join(format!("{subdirectory}.json")),
            &texture_atlas.create_info(),
        )
        .context("write texture atlas info")?;

        // save the texture atlas image
        self.save_and_upload_image(
            texture_atlas.into_image(),
            &image_output_folder,
            subdirectory,
            "packed.webp",
            upload_manager,
        )
        .context("save texture atlas")?;

        Ok(())
    }

    fn save_and_upload_image(
        &self,
        image: Rgba8Image,
        output_folder: &Path,
        object_storage_path: &str,
        file_name: &str,
        upload_manager: &mut UploadManager,
    ) -> anyhow::Result<()> {
        let file_path = output_folder.join(file_name);

        let ext = file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        debug!(?file_path, "encoding image to {ext}...");
        let image_bytes = match ext {
            "png" => match self.compression {
                Some(compression) => image
                    .encode_oxipng(compression)
                    .context("encode using oxipng"),
                None => image.encode_png().context("encode png"),
            }?,
            "webp" => image.encode_webp().context("encode webp")?,
            _ => bail!("Unknown image extension {}", ext),
        };

        if !self.dont_write_images {
            debug!(?file_path, "saving image...");
            std::fs::write(&file_path, &image_bytes).context("write to image file")?;
            debug!(?file_path, "saved image");
        }

        // store image for upload
        upload_manager
            .upload(
                &format!("{}/{}", object_storage_path, file_name),
                &image_bytes,
            )
            .context("upload to s3")?;

        Ok(())
    }
}
