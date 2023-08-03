use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use argh::FromArgs;
use tracing::{debug, info};

use crate::utils::{extract_game_version, match_pattern, PakIndex};

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
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
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

        extract_images(&mut pak_index, &output_directory, self.compression)?;
        info!("Extracted images");

        Ok(())
    }
}

fn extract_images(
    pak_index: &mut PakIndex,
    output_directory: &Path,
    opt_level: Option<u8>,
) -> anyhow::Result<()> {
    info!("Extracting monster portraits");
    const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
    let monsters_path = output_directory.join("monsters");
    extract_prefixed(pak_index, MONSTER_PATTERN, &monsters_path, opt_level)
        .context("extract monster portraits")?;

    info!("Extracting item icons");
    const ITEM_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t";
    let items_path = output_directory.join("items");
    extract_prefixed(pak_index, ITEM_PATTERN, &items_path, opt_level)
        .context("extract item icons")?;

    Ok(())
}

fn extract_prefixed(
    pak_index: &mut PakIndex,
    pattern: &'static str,
    output_folder: &Path,
    opt_level: Option<u8>,
) -> anyhow::Result<()> {
    debug!("Creating output directory");
    std::fs::create_dir_all(output_folder).context("create output directory")?;

    let mut entries: Vec<_> = pak_index
        .iter_entries()
        .filter(|e| match_pattern(pattern, e.get_file_name()).is_some())
        .map(|f| f.get_file_name().to_string())
        .collect();

    entries.sort();

    for entry in entries {
        let monster_num = match_pattern(pattern, &entry).context("Extract id from path")?;

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
                texture.height
            );
        }

        debug!(?entry, "reading image");
        let image_bytes = g1t.read_image(texture, &mut file).context("read image")?;

        debug!(?entry, "converting image");

        debug!(?entry, "encoding image to png...");
        let png_bytes = if let Some(compression) = opt_level {
            let image_buffer = oxipng::RawImage::new(
                texture.width,
                texture.height,
                oxipng::ColorType::RGBA,
                oxipng::BitDepth::Eight,
                image_bytes,
            )
            .context("load raw buffer as image")?;
            let mut opts = oxipng::Options::from_preset(compression);

            // explicitly allow modifying alpha, which gives another ~7% improvement on level 1
            opts.optimize_alpha = true;

            image_buffer
                .create_optimized_png(&opts)
                .context("optimize png")?
        } else {
            let mut png_bytes = vec![];

            let mut encoder = png::Encoder::new(&mut png_bytes, texture.width, texture.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);

            let mut writer = encoder.write_header().context("write png header")?;
            writer
                .write_image_data(&image_bytes)
                .context("write png data")?;

            drop(writer);

            png_bytes
        };

        let file_path = output_folder.join(format!("{monster_num}.png"));
        debug!(?entry, ?file_path, "saving image...");
        std::fs::write(&file_path, png_bytes).context("write to png file")?;
        debug!(?entry, ?file_path, "saved image");
    }

    Ok(())
}
