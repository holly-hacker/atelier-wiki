pub mod extract_maps;

use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Context};
use argh::FromArgs;
pub use extract_maps::MapInfoList;
use tracing::{debug, info};

use crate::utils::{
    extract_game_version, game_slug,
    images::{rgba8_image::Rgba8Image, texture_atlas::UniformTextureAtlas},
    match_pattern, PakIndex,
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

    /// the category of images to extract. if not present, extract all images
    #[argh(option)]
    category: Option<Category>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Category {
    Monsters,
    Items,
    Maps,
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monsters" => Ok(Self::Monsters),
            "items" => Ok(Self::Items),
            "maps" => Ok(Self::Maps),
            _ => Err(format!("Unknown category {}", s)),
        }
    }
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
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

        let output_dir = self
            .output_directory
            .clone()
            .unwrap_or_else(|| PathBuf::from("game_data"));
        debug!(?output_dir);

        // loading index of game files
        debug!("Reading pak file index");
        let mut pak_index =
            PakIndex::read(&self.game_directory, game_version).context("read data dir")?;
        info!("Loaded pak file index with {} entries", pak_index.len());

        let output_directory = output_dir.join(slug);

        debug!("Creating output directory");
        std::fs::create_dir_all(&output_directory).context("create output directory")?;

        self.extract_images(&mut pak_index, &output_directory, self.category)?;
        info!("Extracted images");

        Ok(())
    }

    fn extract_images(
        &self,
        pak_index: &mut PakIndex,
        output_directory: &Path,
        category: Option<Category>,
    ) -> anyhow::Result<()> {
        if category.is_none() || category == Some(Category::Monsters) {
            info!("Extracting monster portraits");
            const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
            self.extract_prefixed_with_texture_atlas(
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
            self.extract_prefixed_with_texture_atlas(
                pak_index,
                ITEM_PATTERN,
                output_directory,
                PATH_ITEMS,
            )
            .context("extract item icons")?;
        }

        if category.is_none() || category == Some(Category::Maps) {
            info!("Extracting map textures");
            extract_maps::extract_map_textures(self, pak_index, output_directory)
                .context("extract map textures")?;
        }

        Ok(())
    }

    fn extract_prefixed_with_texture_atlas(
        &self,
        pak_index: &mut PakIndex,
        pattern: &'static str,
        output_directory: &Path,
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
            .filter_map(|e| match_pattern::<usize>(pattern, e.get_file_name()).map(|num| (e, num)))
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
            let image =
                Rgba8Image::new(texture.width, image_bytes).context("image buffer to image")?;
            debug_assert_eq!(image.height(), texture.height);

            debug!(?entry, "adding image to texture atlas");
            texture_atlas
                .add_image(&image, num.to_string())
                .context("add image to texture atlas")?;

            self.save_image(image, &image_output_folder, &format!("{}.png", num))
                .with_context(|| format!("save image {num}"))?;
        }

        // save the texture atlas info
        let atlas_directory = output_directory.join("texture-atlasses");
        std::fs::create_dir_all(&atlas_directory).context("create atlas directory")?;
        crate::extract::write_data_to_file(
            &atlas_directory.join(format!("{subdirectory}.json")),
            &texture_atlas.create_info(),
        )
        .context("write texture atlas info")?;

        // save the texture atlas image
        self.save_image(
            texture_atlas.into_image(),
            &image_output_folder,
            "packed.webp",
        )
        .context("save texture atlas")?;

        Ok(())
    }

    fn save_image(
        &self,
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
            std::fs::write(&file_path, image_bytes).context("write to image file")?;
            debug!(?file_path, "saved image");
        }

        Ok(())
    }
}
