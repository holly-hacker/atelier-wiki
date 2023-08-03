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

        info!("Extracting monster portraits");
        const MONSTER_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_monster_l_*.g1t";
        let monsters_path = output_directory.join("monsters");
        extract_prefixed(&mut pak_index, MONSTER_PATTERN, &monsters_path)
            .context("extract monster portraits")?;

        info!("Extracting item icons");
        const ITEM_PATTERN: &str = r"\data\x64\res_cmn\ui\neo\neo_a24_item_l_*.g1t";
        let items_path = output_directory.join("items");
        extract_prefixed(&mut pak_index, ITEM_PATTERN, &items_path)
            .context("extract item icons")?;

        Ok(())
    }
}

fn extract_prefixed(
    pak_index: &mut PakIndex,
    pattern: &'static str,
    output_path: &Path,
) -> anyhow::Result<()> {
    debug!("Creating output directory");
    std::fs::create_dir_all(output_path).context("create output directory")?;

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

        let g1t = gust_g1t::GustG1t::read(&mut file).context("read g1t")?;
        let texture = &g1t.textures[0];

        if texture.width != 512 && texture.height != 512 {
            bail!(
                "Texture {entry} has invalid size {}x{}, expected 512x512",
                texture.width,
                texture.height
            );
        }

        let image_bytes = g1t.read_image(texture, &mut file).context("read image")?;

        let image_buffer = image::RgbaImage::from_vec(texture.width, texture.height, image_bytes)
            .context("image to rgbimage vec")?;

        let output_path = output_path.join(format!("{monster_num}.png"));

        debug!("saving image...");
        image_buffer
            .save_with_format(output_path, image::ImageFormat::Png)
            .context("save file")?;
    }

    Ok(())
}
